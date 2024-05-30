use adw::prelude::*;
use adw::subclass::prelude::*;
use gtk::glib;
use gtk::prelude::{TextBufferExt, TextViewExt};
use gtk::CompositeTemplate;
use crate::core::adoc::TableFormator;

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

}

impl Default for AdocPage {
    fn default() -> Self {
        Self::new()
    }
}

mod imp {

    use super::*;

    #[derive(Debug, Default, CompositeTemplate)]
    #[template(resource = "/com/github/nobodygx/asciibox/ui/page_adoc.ui")]
    pub struct AdocPage {
        #[template_child]
        pub in_view: TemplateChild<gtk::TextView>,
        #[template_child]
        pub out_view: TemplateChild<gtk::TextView>,
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
            self.parent_constructed();
        }
    }
    impl WidgetImpl for AdocPage {}
    impl BoxImpl for AdocPage {}
}
