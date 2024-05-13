use gtk::{glib, subclass::prelude::*, CompositeTemplate};

glib::wrapper! {
    pub struct SvgbobPage(ObjectSubclass<imp::SvgbobPage>)
        @extends gtk::Widget, gtk::Box,
        @implements gtk::Accessible, gtk::Buildable,gtk::ConstraintTarget, gtk::Orientable;
}

impl SvgbobPage {
    pub fn new() -> Self {
        let page: SvgbobPage = glib::Object::new();
        page
    }

    pub fn init_page(&self) {
       println!("init page");
    }

}

impl Default for SvgbobPage {
    fn default() -> Self {
        Self::new()
    }
}

mod imp {

    use super::*;

    #[derive(Debug, Default, CompositeTemplate)]
    #[template(resource = "/com/github/nobodygx/asciibox/ui/page_svgbob.ui")]
    pub struct SvgbobPage {
        #[template_child]
        pub in_view: TemplateChild<gtk::TextView>,
        #[template_child]
        pub out_view: TemplateChild<gtk::TextView>,
    }

    #[glib::object_subclass]
    impl ObjectSubclass for SvgbobPage {
        const NAME: &'static str = "SvgbobPage";
        type Type = super::SvgbobPage;
        type ParentType = gtk::Box;

        fn class_init(klass: &mut Self::Class) {
            klass.bind_template();
            klass.bind_template_callbacks();
            load_css();
        }

        fn instance_init(obj: &glib::subclass::InitializingObject<Self>) {
            obj.init_template();
        }
    }

    

    #[gtk::template_callbacks]
    impl SvgbobPage {
        
    }

    impl ObjectImpl for SvgbobPage {
        fn constructed(&self) {
            self.parent_constructed();

            let obj = self.obj();
            obj.setup_text_view();
        }
    }
    impl WidgetImpl for SvgbobPage {}
    impl BoxImpl for SvgbobPage {}
}

impl SvgbobPage {
    fn setup_text_view(&self) {
    }
}

fn load_css() {
    // Load the CSS file and add it to the provider
    // let provider = CssProvider::new();
    // provider.load_from_resource("/com/gitee/gmg137/NeteaseCloudMusicGtk4/themes/discover.css");

    // // Add the provider to the default screen
    // style_context_add_provider_for_display(
    //     &gdk::Display::default().expect("Could not connect to a display."),
    //     &provider,
    //     gtk::STYLE_PROVIDER_PRIORITY_APPLICATION,
    // );
}
