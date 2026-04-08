use crate::utils;
use adw::prelude::*;
use adw::subclass::prelude::*;
use gettextrs::gettext;
use gtk::CompositeTemplate;
use gtk::glib;
use gtk::glib::property::PropertySet;
use gtk::prelude::{TextBufferExt, TextViewExt};
use sourceview;
use sourceview::prelude::BufferExt;
use std::cell::RefCell;
use svgbob;

use super::widget::ImagePreviewDialog;

mod imp {

    use super::*;

    #[derive(Debug, Default, CompositeTemplate)]
    #[template(resource = "/com/github/nobodygx/asciibox/ui/page_asciibox.ui")]
    pub struct AsciiboxPage {
        #[template_child]
        pub in_view: TemplateChild<sourceview::View>,
        #[template_child]
        pub out_view: TemplateChild<sourceview::View>,

        pub svg_content: RefCell<String>,
    }

    #[glib::object_subclass]
    impl ObjectSubclass for AsciiboxPage {
        const NAME: &'static str = "AsciiboxPage";
        type Type = super::AsciiboxPage;
        type ParentType = adw::Bin;

        fn class_init(klass: &mut Self::Class) {
            klass.bind_template();
            klass.bind_template_callbacks();

            klass.install_action("execute-transform", None, move |obj, _, _| {
                obj.execute_transform();
            });
            klass.install_action("asciibox.execute-clear", None, move |obj, _, _| {
                obj.execute_clear();
            });

            klass.install_action("asciibox.execute-clear-result", None, move |obj, _, _| {
                obj.execute_clear_result();
            });

            klass.install_action("asciibox.execute-preview-svgbob", None, move |obj, _, _| {
                obj.execute_preview_svgbob();
            });

            klass.install_action_async("asciibox.execute-save", None, |obj, _, _| async move {
                obj.save().await
            });
        }

        fn instance_init(obj: &glib::subclass::InitializingObject<Self>) {
            obj.init_template();
        }
    }

    impl ObjectImpl for AsciiboxPage {
        fn constructed(&self) {
            self.parent_constructed();

            let obj = self.obj();
            obj.setup_text_view();
            obj.setup_gtk_theme();
        }
    }
    impl WidgetImpl for AsciiboxPage {}
    impl BinImpl for AsciiboxPage {}

    #[gtk::template_callbacks]
    impl AsciiboxPage {
        #[template_callback]
        fn svgbob_svg_copy(&self) {
            self.obj().do_copy_svg_file();
        }
    }
}

glib::wrapper! {
    pub struct AsciiboxPage(ObjectSubclass<imp::AsciiboxPage>)
        @extends adw::Bin, gtk::Widget,
        @implements gtk::Accessible, gtk::Buildable, gtk::ConstraintTarget;
}

impl Default for AsciiboxPage {
    fn default() -> Self {
        Self::new()
    }
}

impl AsciiboxPage {
    pub fn new() -> Self {
        let page: AsciiboxPage = glib::Object::new();
        page
    }

    pub fn init_page(&self) {
        log::debug!("init page");
    }
}

impl AsciiboxPage {
    // 配置默认的 placeholdtext
    fn setup_text_view(&self) {}

    fn setup_gtk_theme(&self) {
        let style_mgr = adw::StyleManager::default();

        style_mgr.connect_color_scheme_notify(glib::clone!(
            #[weak(rename_to = app)]
            self,
            move |manager| {
                let in_buffer = app
                    .imp()
                    .in_view
                    .buffer()
                    .downcast::<sourceview::Buffer>()
                    .unwrap();
                let out_buffer = app
                    .imp()
                    .out_view
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
                                in_buffer.set_style_scheme(Some(&scheme));
                                out_buffer.set_style_scheme(Some(&scheme));
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
                                in_buffer.set_style_scheme(Some(&scheme));
                                out_buffer.set_style_scheme(Some(&scheme));
                                break;
                            }
                        }
                    }
                    _ => {
                        in_buffer.set_style_scheme(Some(&ssm.scheme("Adwaita").unwrap()));
                        out_buffer.set_style_scheme(Some(&ssm.scheme("Adwaita").unwrap()));
                    }
                }
            }
        ));
    }

    fn execute_transform(&self) {
        // let ibuffer: gtk::TextBuffer = self.imp().in_view.get().buffer();
        // let content = ibuffer.text(&ibuffer.bounds().0, &ibuffer.bounds().1, false);

        // // 当输入为 0 的时候不覆盖，这样可以编辑 flowchart 窗口并转换
        // if content.len() != 0 {
        //     let mut mmap: AMap = AMap::new(true);
        //     let otext: String = mmap.load_content(content.as_str());

        //     let obuffer = self.imp().out_view.get().buffer();
        //     obuffer.set_text(otext.as_str());
        // }
    }

    fn execute_clear(&self) {
        let ibuffer: gtk::TextBuffer = self.imp().in_view.get().buffer();
        ibuffer.set_text("");
    }

    fn execute_clear_result(&self) {
        let obuffer = self.imp().out_view.get().buffer();
        obuffer.set_text("");
    }

    fn execute_preview_svgbob(&self) {
        let buffer = self.imp().out_view.get().buffer();
        let content = buffer.text(&buffer.bounds().0, &buffer.bounds().1, false);
        let svg_content = svgbob::to_svg_string_pretty(content.as_str());
        self.imp().svg_content.set(svg_content.clone());

        let dialog = ImagePreviewDialog::new();
        dialog.set_svg(svg_content);

        let Some(parent_window) = self.root().and_downcast::<gtk::Window>() else {
            return;
        };
        dialog.present(Some(&parent_window));
    }

    fn do_copy_svg_file(&self) {
        let clipboard = self.clipboard();
        clipboard.set_text(self.imp().svg_content.borrow().as_str());
    }

    async fn save(&self) {
        let title = format!("{} {} {}", &gettext("Save"), "svg", &gettext("file"));

        utils::save_dialog(
            &self.root().and_downcast::<gtk::Window>().unwrap(),
            &title,
            &self.imp().svg_content.borrow().as_bytes(),
            Some("svg".to_string()),
        )
        .await;
    }
}
