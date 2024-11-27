use adw::{ColorScheme, StyleManager};
use glib::subclass::InitializingObject;
use glib::Object;
use gtk::subclass::prelude::*;
use gtk::{gio, glib, prelude::*, CompositeTemplate, MenuButton};

use crate::application::AsciiboxApplication;
use crate::core::config;
use crate::gui::{FlowchartPage, TablePage};

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
        pub flowchart: TemplateChild<FlowchartPage>,
        #[template_child]
        pub table: TemplateChild<TablePage>,
        pub config: config::Config,
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
            klass.install_action("win.clear_all", None, |window, _, _| {
                window.clear_all();
            });
            klass.install_action("win.switch_tab", None, |window, _, _| {
                window.switch_tab();
            });
            klass.install_action("win.refresh_text_view_font", None, |window, _, _| {
                window.refresh_text_view_font();
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

    fn setup_widget(&self) {
        let imp = self.imp();
        let popover = imp.main_menu_button.popover().unwrap();
        let popover_menu = popover.downcast::<gtk::PopoverMenu>().unwrap();
        let theme = crate::gui::ThemeSelector::new();
        popover_menu.add_child(&theme, "theme");
    }

    fn setup_actions(&self) {
        // 绑定设置与主题
        // let action_style = self.settings().create_action("theme-style");
        // self.add_action(&action_style);
    }

    fn execute(&self) {
        let imp = self.imp();
        let s = imp.stack.visible_child_name();
        match s {
            Some(name) => {
                if name.as_str() == "flowchart" {
                    let _ = imp
                        .flowchart
                        .activate_action("flowchart.do_transform", None);
                } else if name.as_str() == "table" {
                    let _ = imp.table.activate_action("table.do_transform", None);
                }
            }
            None => {}
        }
    }

    fn clear_all(&self) {
        let imp = self.imp();
        let s = imp.stack.visible_child_name();
        match s {
            Some(name) => {
                if name.as_str() == "flowchart" {
                    let _ = imp.flowchart.activate_action("flowchart.do_clear", None);
                } else if name.as_str() == "table" {
                    let _ = imp.table.activate_action("table.do_clear", None);
                }
            }
            None => {}
        }
    }

    fn switch_tab(&self) {
        let mut names: Vec<&str> = Vec::new();
        names.push("flowchart");
        names.push("table");
        names.push(names[0]);
        let s = self.imp().stack.visible_child_name();
        match s {
            Some(target) => {
                let mut flag = false;
                for name in names.iter() {
                    if flag {
                        self.imp().stack.set_visible_child_name(name);
                        return;
                    }
                    flag = if target.as_str().eq(*name) {
                        true
                    } else {
                        false
                    };
                }
            }
            None => {}
        }
    }

    fn refresh_text_view_font(&self) {
        println!("refresh_text_view_font");
        let imp = self.imp();
        // 只需要改动一个，因为目前直接改动的是 textview 基类 css
        let _ = imp.table.activate_action("table.refresh_font", None);
    }
}

#[gtk::template_callbacks]
impl MainWindow {
    #[template_callback]
    fn stack_visible_child_cb(&self) {}
}
