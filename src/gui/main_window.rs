use gio::Settings;
use glib::Object;
use gtk::subclass::prelude::*;
use gtk::{gio, glib, CompositeTemplate, ListView, MenuButton, prelude::*};
use glib::subclass::InitializingObject;
use std::cell::OnceCell;

use crate::application::AsciiboxApplication;
use crate::APP_ID;


mod imp {
    use super::*;

    // Object holding the state
    #[derive(CompositeTemplate, Default)]
    #[template(resource = "/com/github/nobodygx/asciibox/ui/main_window.ui")]
    pub struct MainWindow {
        #[template_child]
        pub tasks_list: TemplateChild<ListView>,
        pub settings: OnceCell<Settings>,
        #[template_child]
        pub main_menu_button: TemplateChild<MenuButton>,
    }

    // The central trait for subclassing a GObject
    #[glib::object_subclass]
    impl ObjectSubclass for MainWindow {
        // `NAME` needs to match `class` attribute of template
        const NAME: &'static str = "AsciiboxWinodw";
        type Type = super::MainWindow;
        type ParentType = gtk::ApplicationWindow;

        fn class_init(klass: &mut Self::Class) {
            klass.bind_template();

            // Create action to remove done tasks and add to action group "win"
            klass.install_action("win.execute_task", None, |window, _, _| {
                window.execute_task();
            });
        }

        fn instance_init(obj: &InitializingObject<Self>) {
            obj.init_template();
        }
    }

    // Trait shared by all GObjects
    impl ObjectImpl for MainWindow {
        fn constructed(&self) {
            // Call "constructed" on parent
            self.parent_constructed();

            // Setup
            let obj = self.obj();
            obj.setup_settings();
            obj.setup_widget();
            obj.setup_actions();
        }
    }

    // Trait shared by all widgets
    impl WidgetImpl for MainWindow {}

    // Trait shared by all windows
    impl WindowImpl for MainWindow {
        fn close_request(&self) -> glib::Propagation {
            // Pass close request on to the parent
            self.parent_close_request()
        }
    }

    // Trait shared by all application windows
    impl ApplicationWindowImpl for MainWindow {}
}

glib::wrapper! {
    pub struct MainWindow(ObjectSubclass<imp::MainWindow>)
        @extends gtk::ApplicationWindow, gtk::Window, gtk::Widget,
        @implements gio::ActionGroup, gio::ActionMap, gtk::Accessible, gtk::Buildable,
                    gtk::ConstraintTarget, gtk::Native, gtk::Root, gtk::ShortcutManager;
}

impl MainWindow {
    pub fn new(app: &AsciiboxApplication) -> Self {
        // Create new window
        Object::builder().property("application", app).build()
    }

    fn setup_settings(&self) {
        let settings = Settings::new(APP_ID);
        self.imp()
            .settings
            .set(settings)
            .expect("`settings` should not be set before calling `setup_settings`.");
    }

    // fn settings(&self) -> &Settings {
    //     self.imp()
    //         .settings
    //         .get()
    //         .expect("`settings` should be set in `setup_settings`.")
    // }

    fn setup_widget(&self) {
        let imp = self.imp();
        let popover = imp.main_menu_button.popover().unwrap();
        let popover_menu = popover.downcast::<gtk::PopoverMenu>().unwrap();
        let theme = crate::gui::ThemeSelector::new();
        popover_menu.add_child(&theme, "theme");
    }

    fn setup_actions(&self) {
        
    }

    fn execute_task(&self) {
        println!("exec task start !!!!")
    }
}
