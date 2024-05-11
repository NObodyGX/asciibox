use gtk::{gio, glib};
use adw::prelude::*;
use adw::subclass::prelude::*;
use crate::window::Window;


mod imp {

    use super::*;    

    #[derive(Debug, Default)]
    pub struct AsciiboxApplication {}

    #[glib::object_subclass]
    impl ObjectSubclass for AsciiboxApplication {
        const NAME: &'static str = "AsciiboxApplication";
        type Type = super::AsciiboxApplication;
        type ParentType = adw::Application;
    }

    impl ObjectImpl for AsciiboxApplication {}
    impl ApplicationImpl for AsciiboxApplication {
        fn activate(&self) {
            self.parent_activate();

            let obj = self.obj();
            let app = obj.downcast_ref::<super::AsciiboxApplication>().unwrap();
            let window = app.create_window();
            window.set_default_size(600, 350);
            window.set_title(Some("Asciibox"));

            window.present();
        }
    }
    impl AdwApplicationImpl for AsciiboxApplication {}
    impl GtkApplicationImpl for AsciiboxApplication {}
}

glib::wrapper! {
    pub struct AsciiboxApplication(ObjectSubclass<imp::AsciiboxApplication>)
        @extends adw::Application, gio::Application, gtk::Application,
        @implements gio::ActionGroup, gio::ActionMap;
}

impl AsciiboxApplication {
    pub fn new(application_id: &str, flags: &gio::ApplicationFlags) -> Self {
        glib::Object::builder()
            .property("application-id", application_id)
            .property("flags", flags)
            .build()
    }
    fn create_window(&self) -> Window {
        let window = Window::new(&self.clone());

        window.present();
        window
    }
}

impl Default for AsciiboxApplication {
    fn default() -> Self {
        glib::Object::builder()
            .property("application-id", "com.github.nobodygx.asciibox")
            .build()
    }
}