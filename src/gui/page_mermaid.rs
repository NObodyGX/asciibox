use adw::subclass::prelude::*;
use gtk::CompositeTemplate;
use gtk::glib;
use gtk::prelude::{TextBufferExt, TextViewExt};

mod imp {

    use super::*;

    #[derive(Debug, Default, CompositeTemplate)]
    #[template(resource = "/com/github/nobodygx/asciibox/ui/page_mermaid.ui")]
    pub struct MermaidPage {
        #[template_child]
        pub in_view: TemplateChild<gtk::TextView>,
        #[template_child]
        pub out_image: TemplateChild<gtk::Picture>,

        pub provider: gtk::CssProvider,
    }

    #[glib::object_subclass]
    impl ObjectSubclass for MermaidPage {
        const NAME: &'static str = "MermaidPage";
        type Type = super::MermaidPage;
        type ParentType = adw::Bin;

        fn class_init(klass: &mut Self::Class) {
            klass.bind_template();

            klass.install_action("mermaid.execute-transform", None, move |obj, _, _| {
                obj.execute_transform();
            });

            klass.install_action("mermaid.execute-clear", None, move |obj, _, _| {
                obj.execute_clear();
            });

            klass.install_action("mermaid.execute-copy-result", None, move |obj, _, _| {
                obj.execute_copy_result();
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

            obj.setup_config();
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
    fn execute_copy_result(&self) {}

    fn execute_transform(&self) {}

    fn execute_clear(&self) {
        let ibuffer: gtk::TextBuffer = self.imp().in_view.get().buffer();
        ibuffer.set_text("");
    }

    fn setup_config(&self) {}

    fn refresh_font(&self) {}
}

impl Default for MermaidPage {
    fn default() -> Self {
        Self::new()
    }
}
