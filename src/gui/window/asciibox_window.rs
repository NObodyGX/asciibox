use adw::ViewStack;
use adw::prelude::*;
use adw::subclass::prelude::*;
use glib::Object;
use glib::subclass::InitializingObject;
use gtk::{Button, gio, glib};
mod imp {

    use super::*;
    use glib::subclass::types::ObjectSubclassIsExt;
    use gtk::gio::Menu;
    use std::cell::{Cell, OnceCell};

    #[derive(Debug, Default)]
    pub struct AsciiboxWindow {
        pub stack: OnceCell<ViewStack>,
        pub dock_button_flowchart: OnceCell<Button>,
        pub dock_button_mermaid: OnceCell<Button>,
        pub dock_button_table: OnceCell<Button>,
        pub dock_button_menu: OnceCell<Button>,

        pub dock_index: Cell<usize>,
    }

    impl AsciiboxWindow {
        fn setup_ui(&self) {
            let obj = self.obj();
            obj.set_title(Some("Asciibox"));
            obj.set_default_width(1280);
            obj.set_default_height(720);
            obj.set_width_request(480);
            obj.set_height_request(320);

            let main_layout = gtk::Box::builder()
                .orientation(gtk::Orientation::Horizontal)
                .spacing(0)
                .build();
            // -- 左侧dock面板 --
            let dock_layout = gtk::Box::builder()
                .orientation(gtk::Orientation::Vertical)
                .spacing(0)
                .css_classes(vec!["dock-box".to_string()])
                .build();
            let icon_image = gtk::Image::builder()
                .icon_name("asciibox")
                .pixel_size(24)
                .tooltip_text("(ㅅ•ᴗ•)please don't click. (ㅅ•ᴗ•)")
                .css_classes(vec!["dock-label".to_string()])
                .build();
            dock_layout.append(&icon_image);
            let dock_button_layout = gtk::Box::builder()
                .orientation(gtk::Orientation::Vertical)
                .spacing(0)
                .vexpand(true)
                .build();
            let btn_flowchart = Button::builder()
                .width_request(48)
                .height_request(48)
                .icon_name("format-text-bold-symbolic")
                .action_name("win.switch-page")
                .action_target(&"'01'".to_variant())
                .css_classes(vec!["flat", "suggested-action"])
                .build();
            let btn_mermaid = Button::builder()
                .width_request(48)
                .height_request(48)
                .icon_name("network-wired-symbolic")
                .action_name("win.switch-page")
                .action_target(&"'02'".to_variant())
                .css_classes(vec!["flat", "suggested-action"])
                .build();
            let btn_table = Button::builder()
                .width_request(48)
                .height_request(48)
                .icon_name("view-list-bullet-symbolic-rtl")
                .action_name("win.switch-page")
                .action_target(&"'03'".to_variant())
                .css_classes(vec!["flat", "suggested-action"])
                .build();

            // 初始化菜单按钮
            let btn_menu = Button::builder()
                .width_request(48)
                .height_request(48)
                .icon_name("preferences-system-symbolic")
                .action_name("app.preferences")
                .css_classes(vec!["dock-button"])
                .tooltip_text("Preferences")
                .build();
            let imp = obj.imp();
            imp.dock_button_flowchart.set(btn_flowchart).unwrap();
            imp.dock_button_mermaid.set(btn_mermaid).unwrap();
            imp.dock_button_table.set(btn_table).unwrap();
            imp.dock_button_menu.set(btn_menu).unwrap();
            dock_button_layout.append(imp.dock_button_flowchart.get().unwrap());
            dock_button_layout.append(imp.dock_button_mermaid.get().unwrap());
            dock_button_layout.append(imp.dock_button_table.get().unwrap());
            dock_button_layout.append(imp.dock_button_menu.get().unwrap());
            dock_layout.append(&dock_button_layout);
            main_layout.append(&dock_layout);

            // -- 右侧内容面板 --
            let toolbar_view = adw::ToolbarView::builder()
                .top_bar_style(adw::ToolbarStyle::Raised)
                .build();
            // ---- 顶部：操作按钮 ----
            let title = adw::WindowTitle::builder().title("asciibox").build();
            let header = adw::HeaderBar::builder().title_widget(&title).build();
            let main_top_menu = Menu::new();
            // let preferences_section = Menu::new();
            // preferences_section.append(Some("Preferences"), Some("app.preferneces"));
            main_top_menu.append(Some("Preferences"), Some("app.preferneces"));
            main_top_menu.append(Some("Keyboard Shortcuts"), Some("app.show-shortcuts"));
            main_top_menu.append(Some("About"), Some("app.about"));
            main_top_menu.append(Some("Quit"), Some("app.quit"));

            let menu_button = gtk::MenuButton::builder()
                .icon_name("open-menu-symbolic")
                .primary(true)
                .build();
            menu_button.set_menu_model(Some(&main_top_menu));
            header.pack_end(&menu_button);
            header.set_title_widget(Some(&title));
            toolbar_view.add_top_bar(&header);
            // ---- 中间：内容面板 ----
            let content_box = gtk::Box::builder()
                .orientation(gtk::Orientation::Horizontal)
                .spacing(0)
                .build();
            let mid_box = gtk::Box::builder()
                .orientation(gtk::Orientation::Horizontal)
                .spacing(0)
                .build();
            content_box.append(&mid_box);

            toolbar_view.set_content(Some(&content_box));
            // ---- 底部：操作按钮 ----
            let btn = Button::builder()
                .label("关闭窗口")
                .halign(gtk::Align::Center)
                .margin_bottom(20)
                .css_classes(vec!["suggested-action".to_string()])
                .build();

            // 整体面板配置
            toolbar_view.add_bottom_bar(&btn);
            main_layout.append(&toolbar_view);
            obj.set_content(Some(&main_layout));
        }
    }

    #[glib::object_subclass]
    impl ObjectSubclass for AsciiboxWindow {
        const NAME: &'static str = "AsciiboxWindow";
        type Type = super::AsciiboxWindow;
        type ParentType = adw::ApplicationWindow;

        fn class_init(klass: &mut Self::Class) {
            klass.install_action(
                "win.switch-page",
                Some(glib::VariantTy::STRING),
                move |obj, _, param| {
                    let var = param.unwrap().get::<String>().unwrap();
                    obj.switch_page(&var);
                },
            );
        }

        fn instance_init(_obj: &InitializingObject<Self>) {}
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

    fn set_dock_button_state(&self, index: usize, is_clicked: bool) {
        let imp = self.imp();
        // let _key = format!("page{:02}", index);
        // if is_clicked {
        //     imp.stack.set_visible_child_name(&key);
        // }

        let button = match index {
            1 => imp.dock_button_flowchart.get(),
            2 => imp.dock_button_mermaid.get(),
            3 => imp.dock_button_table.get(),
            _ => {
                return;
            }
        };
        if button.is_some() {
            if is_clicked {
                button.unwrap().remove_css_class("dock-button");
                button.unwrap().add_css_class("clicked-dock-button");
            } else {
                button.unwrap().remove_css_class("clicked-dock-button");
                button.unwrap().add_css_class("dock-button");
            }
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

    fn switch_page(&self, page_str: &String) {
        let index = page_str.parse::<usize>().unwrap_or_default();
        self.click_dock_button(index);
    }
}
