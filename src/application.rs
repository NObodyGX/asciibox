use crate::gui::main_window::AsciiboxMainWindow;
mod imp {

    use gtk::{glib, prelude::*, subclass::prelude::*};

    #[derive(Debug, Default)]
    pub struct AsciiboxApplication {}

    #[glib::object_subclass]
    impl ObjectSubclass for AsciiboxApplication {
        const NAME: &'static str = "AsciiboxApplication";
        type Type = super::AsciiboxApplication;
        type ParentType = gtk::Application;
    }

    impl ObjectImpl for AsciiboxApplication {}
    impl ApplicationImpl for AsciiboxApplication {
        fn activate(&self) {
            self.parent_activate();

            let obj = self.obj();
            let app = obj.downcast_ref::<super::AsciiboxApplication>().unwrap();
            let window = app.create_window();
            window.set_default_size(600, 350);
            window.set_title(Some("Application Subclass"));

            window.present();
        }
    }
    impl GtkApplicationImpl for AsciiboxApplication {}
}

use gtk::{gio, glib, prelude::*};

glib::wrapper! {
    pub struct AsciiboxApplication(ObjectSubclass<imp::AsciiboxApplication>)
        @extends gio::Application, gtk::Application,
        @implements gio::ActionGroup, gio::ActionMap;
}

impl AsciiboxApplication {
    fn create_window(&self) -> AsciiboxMainWindow {
        let window = AsciiboxMainWindow::new(&self.clone());

        window.present();
        window
    }
}

impl Default for AsciiboxApplication {
    fn default() -> Self {
        glib::Object::builder()
            .property("application-id", "org.gtk_rs.application-subclass")
            .build()
    }
}
