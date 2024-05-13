use gtk::{glib, subclass::prelude::*, CompositeTemplate};

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

    pub fn init_page(&self) {
       println!("init page");
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
        
    }

    #[glib::object_subclass]
    impl ObjectSubclass for AdocPage {
        const NAME: &'static str = "AdocPage";
        type Type = super::AdocPage;
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
