use adw::prelude::PreferencesRowExt;
use adw::subclass::prelude::*;
use base64::Engine;
use base64::engine;
use gettextrs::gettext;
use gtk::CompositeTemplate;
use gtk::glib;
use gtk::glib::property::PropertySet;
use gtk::prelude::{TextBufferExt, TextViewExt};
use sourceview;
use sourceview::prelude::BufferExt;
use std::cell::Cell;
use webkit::WebView;
use webkit::prelude::*;

use crate::core::AppSettings;
use crate::core::MermaidTheme;
use crate::core::MermaidThemeConfig;
use crate::core::MermaidThemeManager;
use crate::utils;

mod imp {

    use std::cell::RefCell;

    use super::*;

    #[derive(Debug, Default, CompositeTemplate)]
    #[template(resource = "/com/github/nobodygx/asciibox/ui/page_mermaid.ui")]
    pub struct MermaidPage {
        #[template_child]
        pub in_view: TemplateChild<sourceview::View>,
        #[template_child]
        pub webview: TemplateChild<WebView>,
        #[template_child]
        pub theme_list: TemplateChild<gtk::ListBox>,

        pub html_content: RefCell<String>,
        pub cur_zoom: Cell<f64>,
        pub image_data: RefCell<String>,
        pub cur_theme: RefCell<String>,
        pub theme_manager: RefCell<MermaidThemeManager>,
    }

    #[glib::object_subclass]
    impl ObjectSubclass for MermaidPage {
        const NAME: &'static str = "MermaidPage";
        type Type = super::MermaidPage;
        type ParentType = adw::Bin;

        fn class_init(klass: &mut Self::Class) {
            klass.bind_template();

            klass.install_action("execute-transform", None, move |obj, _, _| {
                obj.execute_transform();
            });

            klass.install_action(
                "mermaid.switch-theme",
                Some(glib::VariantTy::STRING),
                move |obj, _, param| {
                    let var = param.unwrap().get::<String>().unwrap();
                    obj.switch_theme(&var);
                },
            );

            klass.install_action("mermaid.zoom-in", None, move |obj, _, _| {
                obj.zoom_in();
            });

            klass.install_action("mermaid.zoom-out", None, move |obj, _, _| {
                obj.zoom_out();
            });

            klass.install_action_async("mermaid.copy", None, move |obj, _, _| async move {
                obj.copy(&String::from("svg")).await;
            });

            klass.install_action_async(
                "mermaid.save",
                Some(glib::VariantTy::STRING),
                move |obj, _, param| async move {
                    let var = param.unwrap().get::<String>().unwrap();
                    obj.save(&var).await;
                },
            );
        }

        fn instance_init(obj: &glib::subclass::InitializingObject<Self>) {
            obj.init_template();
        }
    }

    impl ObjectImpl for MermaidPage {
        fn constructed(&self) {
            let obj = self.obj();
            self.parent_constructed();

            obj.setup_webview();
            obj.setup_settings();
            obj.setup_gtk_theme();
        }
    }
    impl WidgetImpl for MermaidPage {}
    impl BinImpl for MermaidPage {}
}

glib::wrapper! {
    pub struct MermaidPage(ObjectSubclass<imp::MermaidPage>)
    @extends gtk::Widget, adw::Bin,
    @implements gtk::Accessible;
}

impl MermaidPage {
    pub fn new() -> Self {
        let page: MermaidPage = glib::Object::new();
        page
    }

    /// 调整 webview 相关配置
    fn setup_webview(&self) {
        let imp = self.imp();
        imp.cur_zoom.set(1.0);

        let webview = imp.webview.get();
        webview.connect_context_menu(|_webview, context_menu, _hit_test_result| {
            // 清除默认菜单项
            context_menu.remove_all();
            true
        });

        let settings = webkit::Settings::new();
        #[cfg(debug_assertions)]
        {
            settings.set_enable_developer_extras(true);
            settings.set_enable_write_console_messages_to_stdout(true);
        }
        webview.set_settings(&settings);
    }

    /// 初始化配置
    fn setup_settings(&self) {
        {
            let tm = &self.imp().theme_manager;
            tm.borrow_mut().init();

            let theme_list = self.imp().theme_list.get();
            for name in tm.borrow().themes.keys() {
                let widget = adw::ActionRow::new();
                widget.set_title(name);
                widget.set_selectable(true);
                widget.set_activatable(true);
                widget.set_action_name(Some("mermaid.switch-theme"));
                widget.set_action_target(Some(format!("'{}'", name)));

                theme_list.append(&widget);
            }
        }

        let settings = AppSettings::get();
        let theme = &settings.mermaid.theme;
        self.setup_content(theme);

        #[cfg(debug_assertions)]
        {
            let buffer = self.imp().in_view.buffer();
            buffer.set_text("graph TD\n    A-->B;\n    A-->C;\n    B-->D;\n    C-->D;");
        }
    }

    fn setup_gtk_theme(&self) {
        let style_mgr = adw::StyleManager::default();

        style_mgr.connect_color_scheme_notify(glib::clone!(
            #[weak(rename_to = app)]
            self,
            move |manager| {
                let buffer = app
                    .imp()
                    .in_view
                    .buffer()
                    .downcast::<sourceview::Buffer>()
                    .unwrap();
                let ssm = sourceview::StyleSchemeManager::default();
                match manager.color_scheme() {
                    adw::ColorScheme::ForceDark | adw::ColorScheme::PreferDark => {
                        for sc in vec![
                            "Adwaita-dark",
                            "classic-dark",
                            "cobalt",
                            "kate-dark",
                            "oblivion",
                            "solarized-dark",
                        ] {
                            if let Some(scheme) = ssm.scheme(sc) {
                                buffer.set_style_scheme(Some(&scheme));
                                break;
                            }
                        }
                    }
                    adw::ColorScheme::ForceLight | adw::ColorScheme::PreferLight => {
                        for sc in vec![
                            "Adwaita",
                            "classic",
                            "cobalt-light",
                            "kate",
                            "solarized-light",
                            "tango",
                        ] {
                            if let Some(scheme) = ssm.scheme(sc) {
                                buffer.set_style_scheme(Some(&scheme));
                                break;
                            }
                        }
                    }
                    _ => {
                        buffer.set_style_scheme(Some(&ssm.scheme("Adwaita").unwrap()));
                    }
                }
            }
        ));
    }

    /// 初始化默认html内容
    fn setup_content(&self, name: &String) {
        let mut content = utils::load_gresource("/com/github/nobodygx/asciibox/html/index.html");
        if content.is_empty() {
            return;
        }
        let theme = MermaidTheme::from(name);
        if theme != MermaidTheme::Default {
            if theme.is_custom() {
                let tm = self.imp().theme_manager.borrow();
                let style = tm.get_theme(name);
                if style.is_none() {
                    return;
                }
                let style = style.unwrap();
                let config_js = MermaidThemeConfig::from_mermaid_style(style);
                content = content.replace(
                    "const mermaid_config = { startOnLoad: false, theme: 'default' }",
                    format!("const mermaid_config = {{ startOnLoad: false, theme: 'base', themeVariables: {} }}", config_js.to_js_string())
                        .as_str(),
                );
            } else {
                content = content.replace(
                    "theme: 'default'",
                    format!("theme: '{}'", theme.mermaid_theme()).as_str(),
                );
            }
        }

        let mermaid_js_content =
            utils::load_gresource("/com/github/nobodygx/asciibox/html/mermaid.min.js");
        if !mermaid_js_content.is_empty() {
            content = content.replace("<script src=\"https://cdn.jsdelivr.net/npm/mermaid@11.6.0/dist/mermaid.min.js\"></script>", &format!("<script>{}</script>", &mermaid_js_content));
        }

        self.imp().html_content.set(content);
    }

    fn zoom_in(&self) {
        let imp = self.imp();
        let val = imp.cur_zoom.get();
        if val < 5.0 {
            let val = val + 0.2;
            imp.cur_zoom.set(val);

            let webview = imp.webview.get();
            webview.set_zoom_level(val);
        }
    }

    fn zoom_out(&self) {
        let imp = self.imp();
        let val = imp.cur_zoom.get();
        if val > 0.2 {
            let val = val - 0.2;
            imp.cur_zoom.set(val);

            let webview = imp.webview.get();
            webview.set_zoom_level(val);
        }
    }

    fn icontent(&self) -> String {
        let imp = self.imp();
        let ibuffer = imp.in_view.get().buffer();
        let content = ibuffer.text(&ibuffer.bounds().0, &ibuffer.bounds().1, false);
        content.to_string()
    }

    fn execute_transform(&self) {
        let imp = self.imp();
        let content = self.icontent();
        if content.len() <= 1 {
            return;
        }

        let html_content = imp
            .html_content
            .borrow()
            .replace("@@ASCIIBOX-NOBODYGX-PLACEHOLD@@", content.as_str());

        imp.webview.get().load_html(&html_content, None);
    }

    async fn get_image_data(&self, image_type: &String) {
        let script = format!(
            "document.getElementById('{}Data').value;",
            image_type.to_lowercase()
        );
        let res = self
            .imp()
            .webview
            .evaluate_javascript_future(script.as_str(), None, None)
            .await;
        match res {
            Ok(value) => {
                let content = value.to_string();
                log::info!("{content}");
                self.imp().image_data.set(content);
            }
            Err(e) => {
                log::error!("error get svg data: {e}");
            }
        }
    }

    async fn copy(&self, image_type: &String) {
        self.get_image_data(image_type).await;
        let content = self.imp().image_data.borrow();

        let clipboard = self.clipboard();
        clipboard.set_text(&content);
    }

    async fn save(&self, image_type: &String) {
        self.get_image_data(image_type).await;
        let content = self.imp().image_data.borrow();

        let title = format!("{} {} {}", &gettext("Save"), image_type, &gettext("file"));

        let save_content = match image_type.as_str() {
            "png" => {
                let content = content.replace("data:image/png;base64,", "");
                &engine::general_purpose::STANDARD
                    .decode(content.as_str())
                    .unwrap()
            }
            "svg" => content.as_bytes(),
            _ => content.as_bytes(),
        };
        utils::save_dialog(
            &self.root().and_downcast::<gtk::Window>().unwrap(),
            &title,
            &save_content,
            Some(image_type.clone()),
        )
        .await;
    }

    fn switch_theme(&self, theme: &String) {
        log::info!("select theme: {theme}");
        {
            let mut settings = AppSettings::get_mut();
            if !settings.mermaid.theme.eq(theme) {
                settings.mermaid.theme = String::from(theme.as_str());
                settings.set_changed();
            }
        }
        self.setup_content(theme);
        self.execute_transform();
    }
}

impl Default for MermaidPage {
    fn default() -> Self {
        Self::new()
    }
}
