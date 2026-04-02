use adw::ViewStack;
use adw::prelude::*;
use adw::subclass::prelude::*;
use glib::Object;
use glib::subclass::InitializingObject;
use gtk::{Button, gio, glib};
mod imp {

    use std::cell::Cell;

    use adw::{HeaderBar, PreferencesPage, ToolbarView};
    use gtk::MenuButton;

    use super::*;

    #[derive(Debug, Default)]
    pub struct AsciiboxWindow {
        pub stack: ViewStack,
        pub dock_button_01: Button,
        pub dock_button_02: Button,
        pub dock_button_03: Button,
        pub menu_button: MenuButton,

        pub dock_index: Cell<usize>,
    }

    impl AsciiboxWindow {
        fn setup_ui(&self) {
            let obj = self.obj();
            obj.set_title(Some("AsciiboxWindow"));
            obj.set_default_width(400);
            obj.set_default_height(500);

            let toolbar_view = adw::ToolbarView::builder().build();
            // --- 顶部：操作按钮 ---
            let header = adw::HeaderBar::builder().build();
            let title = gtk::Label::builder().label("欢迎面板").build();
            title.add_css_class("heading");
            header.set_title_widget(Some(&title));
            toolbar_view.add_top_bar(&header);

            // --- 中间：内容面板 ---

            // --- 底部：操作按钮 ---
            let btn = Button::builder()
                .label("关闭窗口")
                .halign(gtk::Align::Center)
                .margin_bottom(20)
                .css_classes(vec!["suggested-action".to_string()])
                .build();

            toolbar_view.add_bottom_bar(&btn);
            obj.set_content(Some(&toolbar_view));
        }
    }

    #[glib::object_subclass]
    impl ObjectSubclass for AsciiboxWindow {
        const NAME: &'static str = "AsciiboxWindow";
        type Type = super::AsciiboxWindow;
        type ParentType = adw::ApplicationWindow;

        fn class_init(klass: &mut Self::Class) {}

        fn instance_init(obj: &InitializingObject<Self>) {}
    }

    impl ObjectImpl for AsciiboxWindow {
        fn constructed(&self) {
            self.parent_constructed();

            self.setup_ui();
            let obj = self.obj();
            obj.setup_widget();
            obj.setup_actions();
        }
    }

    impl BuildableImpl for AsciiboxWindow {}

    impl WidgetImpl for AsciiboxWindow {}

    impl WindowImpl for AsciiboxWindow {
        fn close_request(&self) -> glib::Propagation {
            self.parent_close_request()
        }
    }

    impl ApplicationWindowImpl for AsciiboxWindow {}

    impl AdwApplicationWindowImpl for AsciiboxWindow {}
}

glib::wrapper! {
    pub struct AsciiboxWindow(ObjectSubclass<imp::AsciiboxWindow>)
        @extends gtk::Window, gtk::Widget, gtk::ApplicationWindow, adw::ApplicationWindow,
        @implements gio::ActionGroup, gio::ActionMap, gtk::Accessible, gtk::Buildable,
                    gtk::ConstraintTarget, gtk::Native, gtk::Root, gtk::ShortcutManager;
}

impl AsciiboxWindow {
    pub fn new(app: &adw::Application) -> Self {
        Object::builder().property("application", app).build()
    }

    fn setup_widget(&self) {}

    fn setup_actions(&self) {}
}
