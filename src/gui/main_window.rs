use crate::application::AsciiboxApplication;
use gtk::{gio, glib, prelude::*, subclass::prelude::*, CompositeTemplate};

mod imp {

    use super::*;

    #[derive(Default, CompositeTemplate)]
    #[template(resource = "/com/github/nobodygx/asciibox/ui/main_window.ui")]
    pub struct AsciiboxMainWindow {}

    impl AsciiboxMainWindow {}

    #[glib::object_subclass]
    impl ObjectSubclass for AsciiboxMainWindow {
        const NAME: &'static str = "AsciiboxMainWindow";
        type Type = super::AsciiboxMainWindow;
        type ParentType = gtk::ApplicationWindow;

        fn class_init(klass: &mut Self::Class) {
            klass.bind_template();
            klass.bind_template_instance_callbacks();
        }

        fn instance_init(obj: &glib::subclass::InitializingObject<Self>) {
            obj.init_template();
        }
    }

    impl ObjectImpl for AsciiboxMainWindow {}
    impl WidgetImpl for AsciiboxMainWindow {}
    impl WindowImpl for AsciiboxMainWindow {}
    impl ApplicationWindowImpl for AsciiboxMainWindow {}
}

glib::wrapper! {
    pub struct AsciiboxMainWindow(ObjectSubclass<imp::AsciiboxMainWindow>)
        @extends gtk::Widget, gtk::Window, gtk::ApplicationWindow,
        @implements gio::ActionGroup, gio::ActionMap;
}

impl AsciiboxMainWindow {
    pub fn new<P: glib::object::IsA<gtk::Application>>(application: &P) -> Self {
        let window: AsciiboxMainWindow = glib::Object::builder()
            .property("application", application)
            .build();

        window.setup_widget();
        window
    }

    fn setup_widget(&self) {}
}

#[gtk::template_callbacks]
impl AsciiboxMainWindow {}

impl Default for AsciiboxMainWindow {
    fn default() -> Self {
        AsciiboxApplication::default()
            .active_window()
            .unwrap()
            .downcast()
            .unwrap()
    }
}
