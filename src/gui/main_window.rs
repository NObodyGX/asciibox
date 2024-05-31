use adw::{ColorScheme, StyleManager};
use gio::Settings;
use glib::subclass::InitializingObject;
use glib::Object;
use gtk::subclass::prelude::*;
use gtk::{gio, glib, prelude::*, CompositeTemplate, MenuButton};
use std::cell::OnceCell;

use crate::application::AsciiboxApplication;
use crate::gui::{AdocPage, SvgbobPage};
use crate::APP_ID;

mod imp {

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
        pub settings: OnceCell<Settings>,
    }

    impl MainWindow {
        pub fn clear_title(&self) {}
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
            klass.install_action("win.execute", None, |window, _, _| {
                window.execute();
            });
            klass.install_action("win.clearall", None, |window, _, _| {
                window.clearall();
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

    pub fn settings(&self) -> &Settings {
        self.imp().settings.get().expect("Could not get settings.")
    }

    fn setup_settings(&self) {
        let settings = Settings::new(APP_ID);
        self.imp()
            .settings
            .set(settings)
            .expect("`settings` should not be set before calling `setup_settings`.");

        // bind theme settings
        let style = StyleManager::default();
        self.settings()
            .bind("theme-style", &style, "color-scheme")
            .mapping(|themes, _| {
                let themes = themes
                    .get::<String>()
                    .expect("The variant needs to be of type `String`.");
                let scheme = match themes.as_str() {
                    "system" => ColorScheme::Default,
                    "light" => ColorScheme::ForceLight,
                    "dark" => ColorScheme::ForceDark,
                    _ => ColorScheme::Default,
                };
                Some(scheme.to_value())
            })
            .build();
    }

    fn setup_widget(&self) {
        let imp = self.imp();
        let popover = imp.main_menu_button.popover().unwrap();
        let popover_menu = popover.downcast::<gtk::PopoverMenu>().unwrap();
        let theme = crate::gui::ThemeSelector::new();
        popover_menu.add_child(&theme, "theme");
    }

    fn setup_actions(&self) {
        // 绑定设置与主题
        let action_style = self.settings().create_action("theme-style");
        self.add_action(&action_style);
    }

    fn execute(&self) {
        println!("exec task start !!!!")
    }

    fn clearall(&self) {
        println!("clear all input!!!!")
    }
}

#[gtk::template_callbacks]
impl MainWindow {
    #[template_callback]
    fn stack_visible_child_cb(&self) {}
}
