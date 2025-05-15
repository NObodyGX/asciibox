use crate::application::BasicApplication;
use adw::ViewStack;
use adw::subclass::prelude::*;
use glib::Object;
use glib::subclass::InitializingObject;
use gtk::{Button, CompositeTemplate, gio, glib, prelude::WidgetExt};

use crate::gui::{FlowchartPage, TablePage};

mod imp {

    use std::cell::Cell;

    use super::*;

    #[derive(CompositeTemplate, Default)]
    #[template(resource = "/com/github/nobodygx/asciibox/ui/main_window.ui")]
    pub struct MainWindow {
        #[template_child]
        pub stack: TemplateChild<ViewStack>,
        #[template_child]
        pub flowchart: TemplateChild<FlowchartPage>,
        #[template_child]
        pub table: TemplateChild<TablePage>,
        #[template_child]
        pub dock_button_01: TemplateChild<Button>,
        #[template_child]
        pub dock_button_02: TemplateChild<Button>,
        #[template_child]
        pub dock_button_settings: TemplateChild<Button>,

        pub dock_index: Cell<usize>,
    }

    impl MainWindow {}

    #[glib::object_subclass]
    impl ObjectSubclass for MainWindow {
        const NAME: &'static str = "MainWindow";
        type Type = super::MainWindow;
        type ParentType = adw::ApplicationWindow;

        fn class_init(klass: &mut Self::Class) {
            klass.bind_template();
            klass.bind_template_instance_callbacks();
        }

        fn instance_init(obj: &InitializingObject<Self>) {
            obj.init_template();
        }
    }

    impl ObjectImpl for MainWindow {
        fn constructed(&self) {
            self.parent_constructed();

            let obj = self.obj();
            obj.setup_config();
            obj.setup_widget();
            obj.setup_actions();
        }
    }

    impl BuildableImpl for MainWindow {}

    impl WidgetImpl for MainWindow {}

    impl WindowImpl for MainWindow {
        fn close_request(&self) -> glib::Propagation {
            self.parent_close_request()
        }
    }

    impl ApplicationWindowImpl for MainWindow {}

    impl AdwApplicationWindowImpl for MainWindow {}
}

glib::wrapper! {
    pub struct MainWindow(ObjectSubclass<imp::MainWindow>)
        @extends gtk::Window, gtk::Widget, gtk::ApplicationWindow, adw::ApplicationWindow,
        @implements gio::ActionGroup, gio::ActionMap, gtk::Accessible, gtk::Buildable,
                    gtk::ConstraintTarget, gtk::Native, gtk::Root, gtk::ShortcutManager;
}

impl MainWindow {
    fn setup_config(&self) {}

    fn setup_widget(&self) {
        self.click_dock_button(1);
    }

    fn setup_actions(&self) {}
}

#[gtk::template_callbacks]
impl MainWindow {
    pub fn new(app: &BasicApplication) -> Self {
        Object::builder().property("application", app).build()
    }

    fn set_dock_button_state(&self, index: usize, is_clicked: bool) {
        let imp = self.imp();
        let key = format!("page{:02}", index);
        if is_clicked {
            imp.stack.set_visible_child_name(&key);
        }

        let button = match index {
            1 => imp.dock_button_01.get(),
            2 => imp.dock_button_02.get(),
            _ => {
                return;
            }
        };
        if is_clicked {
            button.remove_css_class("dock-button");
            button.add_css_class("clicked-dock-button");
        } else {
            button.remove_css_class("clicked-dock-button");
            button.add_css_class("dock-button");
        }
    }

    fn click_dock_button(&self, index: usize) {
        let imp = self.imp();
        if imp.dock_index.get() == index {
            return;
        }

        self.set_dock_button_state(imp.dock_index.get(), false);
        self.set_dock_button_state(index, true);
        imp.dock_index.set(index);
    }

    #[template_callback]
    pub fn switch_page_01(&self, _: gtk::Button) {
        self.click_dock_button(1);
    }

    #[template_callback]
    pub fn switch_page_02(&self, _: gtk::Button) {
        self.click_dock_button(2);
    }

    #[template_callback]
    pub fn clicked_settings(&self, _: gtk::Button) {
        // todo
    }
}
