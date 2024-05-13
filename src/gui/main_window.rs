use gio::Settings;
use glib::Object;
use gtk::subclass::prelude::*;
use gtk::{gio, glib, CompositeTemplate, MenuButton, prelude::*};
use glib::subclass::InitializingObject;
use std::cell::OnceCell;
use std::collections::LinkedList;

use crate::application::AsciiboxApplication;
use crate::APP_ID;


mod imp {
    use std::sync::{Arc, Mutex};

    use gtk::Label;

    use crate::gui::{AdocPage, SvgbobPage};

    use super::*;

    // Object holding the state
    #[derive(CompositeTemplate, Default)]
    #[template(resource = "/com/github/nobodygx/asciibox/ui/main_window.ui")]
    pub struct MainWindow {
        #[template_child]
        pub switcher_title: TemplateChild<adw::ViewSwitcher>,
        #[template_child]
        pub main_menu_button: TemplateChild<MenuButton>,
        #[template_child]
        pub stack: TemplateChild<adw::ViewStack>,
        #[template_child]
        pub svgbob: TemplateChild<SvgbobPage>,
        #[template_child]
        pub adoc: TemplateChild<AdocPage>,
        pub stack_child: Arc<Mutex<LinkedList<(String, String)>>>,
        pub settings: OnceCell<Settings>,
    }

    impl MainWindow {
        pub fn clear_title(&self) {
        }
    }

    #[glib::object_subclass]
    impl ObjectSubclass for MainWindow {
        // `NAME` needs to match `class` attribute of template
        const NAME: &'static str = "MainWindow";
        type Type = super::MainWindow;
        type ParentType = gtk::ApplicationWindow;

        fn class_init(klass: &mut Self::Class) {
            klass.bind_template();
            klass.bind_template_instance_callbacks();

            // Create action to remove done tasks and add to action group "win"
            klass.install_action("win.execute_task", None, |window, _, _| {
                window.execute_task();
            });
        }

        fn instance_init(obj: &InitializingObject<Self>) {
            obj.init_template();
        }
    }

    impl ObjectImpl for MainWindow {
        fn constructed(&self) {
            self.parent_constructed();

            let obj = self.obj();
            obj.setup_settings();
            obj.setup_widget();
            obj.setup_actions();
        }
    }

    impl WidgetImpl for MainWindow {}

    impl WindowImpl for MainWindow {
        fn close_request(&self) -> glib::Propagation {
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

#[gtk::template_callbacks]
impl MainWindow {

    #[template_callback]
    fn stack_visible_child_cb(&self) {
        
    }
}
