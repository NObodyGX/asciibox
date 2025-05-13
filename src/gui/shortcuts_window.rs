use adw::subclass::prelude::*;
use gtk::CompositeTemplate;
use gtk::glib;
use gtk::glib::Object;
use gtk::glib::subclass::InitializingObject;

mod imp {
    use super::*;

    #[derive(Default, CompositeTemplate)]
    #[template(resource = "/com/github/nobodygx/asciibox/ui/shortcuts.ui")]
    pub struct ShortcutsWindow {}

    #[glib::object_subclass]
    impl ObjectSubclass for ShortcutsWindow {
        const NAME: &'static str = "ShortcutsWindow";
        type Type = super::ShortcutsWindow;
        type ParentType = adw::Dialog;

        fn class_init(klass: &mut Self::Class) {
            klass.bind_template();
        }

        fn instance_init(obj: &InitializingObject<Self>) {
            obj.init_template();
        }
    }

    impl ObjectImpl for ShortcutsWindow {}

    impl WidgetImpl for ShortcutsWindow {}

    impl AdwDialogImpl for ShortcutsWindow {}
}

glib::wrapper! {
    pub struct ShortcutsWindow(ObjectSubclass<imp::ShortcutsWindow>)
        @extends gtk::Widget, adw::Dialog,
        @implements gtk::Accessible, gtk::Buildable;
}

impl Default for ShortcutsWindow {
    fn default() -> Self {
        Object::builder().build()
    }
}
