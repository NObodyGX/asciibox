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
        #[template_callback]
        fn top_picks_cb(&self) {
            println!("click top picks cb");
        }

        #[template_callback]
        fn new_albums_cb(&self) {
            println!("click new_albums_cb");
        }
    }

    impl ObjectImpl for SvgbobPage {
        fn constructed(&self) {
            self.parent_constructed();
        }
    }
    impl WidgetImpl for SvgbobPage {}
    impl BoxImpl for SvgbobPage {}
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
