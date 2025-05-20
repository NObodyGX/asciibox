use adw::subclass::prelude::*;
use gtk::CompositeTemplate;
use gtk::glib;
use gtk::glib::property::PropertySet;
use gtk::prelude::{TextBufferExt, TextViewExt};
use sourceview;
use std::cell::{Cell, OnceCell};
use webkit::WebView;
use webkit::prelude::*;

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

        pub html_content: OnceCell<String>,
        pub cur_zoom: Cell<f64>,
        pub svg_data: RefCell<String>,
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

            klass.install_action("mermaid.zoom-in", None, move |obj, _, _| {
                obj.zoom_in();
            });

            klass.install_action("mermaid.zoom-out", None, move |obj, _, _| {
                obj.zoom_out();
            });
            klass.install_action_async("mermaid.save", None, move |obj, _, _| async move {
                obj.save().await;
            });
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
            obj.setup_content();
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
        settings.set_enable_developer_extras(true);
        settings.set_enable_write_console_messages_to_stdout(true);
        webview.set_settings(&settings);
    }

    /// 初始化默认html内容
    fn setup_content(&self) {
        let mermaid_js_content =
            utils::load_gresource("/com/github/nobodygx/asciibox/html/mermaid.min.js");
        let mut content = utils::load_gresource("/com/github/nobodygx/asciibox/html/index.html");
        if content.is_empty() {
            return;
        }
        if !mermaid_js_content.is_empty() {
            content = content.replace("<script src=\"https://cdn.jsdelivr.net/npm/mermaid/dist/mermaid.min.js\"></script>", &format!("<script>{}</script>", &mermaid_js_content));
        }

        match self.imp().html_content.set(content) {
            Ok(_) => {}
            Err(e) => {
                log::error!("failed to set content: {e}");
            }
        }
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

    fn execute_transform(&self) {
        let imp = self.imp();
        let ibuffer = imp.in_view.get().buffer();
        let content = ibuffer.text(&ibuffer.bounds().0, &ibuffer.bounds().1, false);
        let content = content.as_str();

        if content.len() <= 1 {
            return;
        }

        let html_content = imp
            .html_content
            .get()
            .unwrap()
            .replace("@@ASCIIBOX-NOBODYGX-PLACEHOLD@@", content);

        imp.webview.get().load_html(&html_content, None);
    }

    async fn get_svg_data(&self) {
        let script = r#"document.getElementById('svgData').value;"#;
        let res = self
            .imp()
            .webview
            .evaluate_javascript_future(script, None, None)
            .await;
        match res {
            Ok(value) => {
                let content = value.to_string();
                self.imp().svg_data.set(content);
            }
            Err(e) => {
                log::error!("error get svg data: {e}");
            }
        }
    }

    async fn save(&self) {
        self.get_svg_data().await;
        let content = self.imp().svg_data.borrow();
        log::info!("{}", content);

        utils::save_dialog(
            &self.root().and_downcast::<gtk::Window>().unwrap(),
            &gettextrs::gettext("Save Svg File"),
            &content,
            Some("svg".to_string()),
        )
        .await;
    }
}

impl Default for MermaidPage {
    fn default() -> Self {
        Self::new()
    }
}
