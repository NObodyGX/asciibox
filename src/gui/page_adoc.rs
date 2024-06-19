use crate::core::adoc::TableFormator;
use adw::prelude::*;
use adw::subclass::prelude::*;
use gtk::gdk;
use gtk::gio::Settings;
use gtk::glib;
use gtk::pango::Weight;
use gtk::prelude::{TextBufferExt, TextViewExt};
use gtk::CompositeTemplate;

mod imp {
    use super::*;

    #[derive(Debug, Default, CompositeTemplate)]
    #[template(resource = "/com/github/nobodygx/asciibox/ui/page_adoc.ui")]
    pub struct AdocPage {
        #[template_child]
        pub in_view: TemplateChild<gtk::TextView>,
        #[template_child]
        pub out_view: TemplateChild<gtk::TextView>,

        pub provider: gtk::CssProvider,
    }

    #[glib::object_subclass]
    impl ObjectSubclass for AdocPage {
        const NAME: &'static str = "AdocPage";
        type Type = super::AdocPage;
        type ParentType = gtk::Box;

        fn class_init(klass: &mut Self::Class) {
            klass.bind_template();
            klass.bind_template_callbacks();

            klass.install_action("adoc.do_transform_copy", None, move |obj, _, _| {
                obj.do_transform_copy();
            });

            klass.install_action("adoc.do_transform", None, move |obj, _, _| {
                obj.do_transform();
            });

            klass.install_action("adoc.do_clear", None, move |obj, _, _| {
                obj.do_clear();
            });
            klass.install_action("adoc.refresh_font", None, move |obj, _, _| {
                obj.refresh_font();
            });
        }

        fn instance_init(obj: &glib::subclass::InitializingObject<Self>) {
            obj.init_template();
        }
    }

    #[gtk::template_callbacks]
    impl AdocPage {
        #[template_callback]
        fn top_picks_cb(&self) {
            println!("click top picks cb");
        }

        #[template_callback]
        fn new_albums_cb(&self) {
            println!("click new_albums_cb");
        }
    }

    impl ObjectImpl for AdocPage {
        fn constructed(&self) {
            let obj = self.obj();
            self.parent_constructed();

            obj.setup_font_setting();
        }
    }
    impl WidgetImpl for AdocPage {}
    impl BoxImpl for AdocPage {}
}

glib::wrapper! {
    pub struct AdocPage(ObjectSubclass<imp::AdocPage>)
        @extends gtk::Widget, gtk::Box,
        @implements gtk::Accessible, gtk::Buildable,gtk::ConstraintTarget, gtk::Orientable;
}

impl AdocPage {
    pub fn new() -> Self {
        let page: AdocPage = glib::Object::new();
        page
    }
    fn do_transform_copy(&self) {
        let clipboard = self.clipboard();
        let buffer = self.imp().out_view.get().buffer();
        let content = buffer.text(&buffer.bounds().0, &buffer.bounds().1, false);
        clipboard.set_text(content.as_str());
    }

    fn do_transform(&self) {
        let ibuffer: gtk::TextBuffer = self.imp().in_view.get().buffer();
        let content = ibuffer.text(&ibuffer.bounds().0, &ibuffer.bounds().1, false);

        // 当输入为 0 的时候不覆盖，这样可以编辑 svgbob 窗口并转换
        if content.len() != 0 {
            let mut formator: TableFormator = TableFormator::new();
            let otext: String = formator.do_format(content.as_str());

            let obuffer = self.imp().out_view.get().buffer();
            obuffer.set_text(otext.as_str());
        }
    }

    fn do_clear(&self) {
        let ibuffer: gtk::TextBuffer = self.imp().in_view.get().buffer();
        ibuffer.set_text("");
    }

    fn setup_font_setting(&self) {
        let imp = self.imp();

        if let Some(display) = gdk::Display::default() {
            gtk::style_context_add_provider_for_display(
                &display,
                &imp.provider,
                gtk::STYLE_PROVIDER_PRIORITY_APPLICATION,
            );
        }
        self.refresh_font();
    }

    fn refresh_font(&self) {
        // TODO: 目前会改动全局 textview 配置
        let imp = self.imp();
        // update font show
        let settings = Settings::new(crate::APP_ID);

        let use_custom_font = settings.boolean("use-custom-font");
        if use_custom_font {
            let custom_font = settings.string("custom-font");
            let fontdesc = gtk::pango::FontDescription::from_string(custom_font.as_str());
            let mut css = String::new();
            css.push_str("textview {\n");
            let family = fontdesc.family().expect("error in family");
            css.push_str(format!("font-family: {};", family).as_str());
            // // todo: add font scale
            let size = fontdesc.size() / gtk::pango::SCALE;
            css.push_str(format!("font-size: {}px;", size).as_str());
            let weight = fontdesc.weight();
            match weight {
                Weight::Bold => {
                    css.push_str("font-weight: bold;");
                }
                _ => {
                    css.push_str("font-weight: normal;");
                }
            }
            // 看 gnome-text-view 是不需要加 } 的，很奇怪
            css.push_str("}\n");
            imp.provider.load_from_string(css.as_str());
        }
    }
}

impl Default for AdocPage {
    fn default() -> Self {
        Self::new()
    }
}
