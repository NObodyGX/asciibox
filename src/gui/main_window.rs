use crate::application::BasicApplication;
use adw::ViewStack;
use adw::subclass::prelude::*;
use glib::Object;
use glib::subclass::InitializingObject;
use gtk::{CompositeTemplate, gio, glib};

use crate::gui::{FlowchartPage, TablePage};

mod imp {

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

    fn setup_widget(&self) {}

    fn setup_actions(&self) {}
}

#[gtk::template_callbacks]
impl MainWindow {
    pub fn new(app: &BasicApplication) -> Self {
        Object::builder().property("application", app).build()
    }

    #[template_callback]
    pub fn switch_pagea(&self, _: gtk::Button) {
        self.imp().stack.set_visible_child_name("flowchart");
    }

    #[template_callback]
    pub fn switch_pageb(&self, _: gtk::Button) {
        self.imp().stack.set_visible_child_name("table");
    }
}
