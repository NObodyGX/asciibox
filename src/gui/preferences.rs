use adw::subclass::prelude::AdwWindowImpl;
use adw::subclass::prelude::PreferencesWindowImpl;
use gio::Settings;
use gtk::gio::SettingsBindFlags;
use gtk::glib::clone;
use gtk::{glib, prelude::*, subclass::prelude::*, CompositeTemplate, *};
use std::cell::OnceCell;

mod imp {

    use std::sync::OnceLock;

    use glib::subclass::Signal;

    use super::*;

    #[derive(Debug, Default, CompositeTemplate)]
    #[template(resource = "/com/github/nobodygx/asciibox/ui/preferences.ui")]
    pub struct MainPreferences {
        pub settings: OnceCell<Settings>,
        #[template_child]
        pub use_custom_font: TemplateChild<Switch>,
        #[template_child]
        pub font: TemplateChild<FontDialogButton>,
        #[template_child]
        pub expand_mode: TemplateChild<Switch>,
        #[template_child]
        pub cell_max_width: TemplateChild<adw::SpinRow>,
        #[template_child]
        pub line_max_width: TemplateChild<adw::SpinRow>,
        #[template_child]
        pub gfm_table_enable: TemplateChild<Switch>,
    }

    #[glib::object_subclass]
    impl ObjectSubclass for MainPreferences {
        const NAME: &'static str = "MainPreferences";
        type Type = super::MainPreferences;
        type ParentType = adw::PreferencesWindow;

        fn class_init(klass: &mut Self::Class) {
            klass.bind_template();
        }

        fn instance_init(obj: &glib::subclass::InitializingObject<Self>) {
            obj.init_template();
        }
    }

    impl ObjectImpl for MainPreferences {
        fn constructed(&self) {
            let obj = self.obj();
            self.parent_constructed();

            obj.setup_settings();
            obj.setup_font();
            obj.bind_settings();
        }

        fn signals() -> &'static [Signal] {
            static SIGNALS: OnceLock<Vec<Signal>> = OnceLock::new();
            SIGNALS.get_or_init(|| {
                vec![
                    Signal::builder("font-changed").build(),
                    Signal::builder("theme-changed").build(),
                ]
            })
        }
    }
    impl WidgetImpl for MainPreferences {}
    impl WindowImpl for MainPreferences {}
    impl AdwWindowImpl for MainPreferences {}
    impl PreferencesWindowImpl for MainPreferences {}
}

glib::wrapper! {
    pub struct MainPreferences(ObjectSubclass<imp::MainPreferences>)
        @extends Widget, Window, adw::Window,
        @implements Accessible, Buildable, ConstraintTarget, Native, Root, ShortcutManager;
}

impl MainPreferences {
    pub fn new() -> Self {
        glib::Object::new()
    }

    fn setup_settings(&self) {
        let settings = Settings::new(crate::APP_ID);
        self.imp()
            .settings
            .set(settings)
            .expect("Could not set `Settings`.");
    }

    fn setup_font(&self) {
        let imp = self.imp();
        let fdesc = self.settings().string("custom-font");
        imp.font
            .set_font_desc(&gtk::pango::FontDescription::from_string(fdesc.as_str()));
        imp.font.connect_font_desc_notify(clone!(
            #[weak]
            imp,
            move |_| {
                let font_string = imp.font.font_desc().unwrap().to_string();
                let settings = imp
                    .settings
                    .get()
                    .expect("Could not get settings from imp.");
                settings
                    .set_string("custom-font", font_string.as_str())
                    .unwrap();
                // println!("{}", font_string);
                imp.obj().emit_by_name::<()>("font-changed", &[]);
            },
        ));
    }

    fn settings(&self) -> &Settings {
        self.imp().settings.get().expect("Could not get settings.")
    }

    fn bind_settings(&self) {
        // 注意: schema 里不能使用 _ 而是需要使用 - 才符合格式
        let use_custom_font = self.imp().use_custom_font.get();
        self.settings()
            .bind("use-custom-font", &use_custom_font, "active")
            .flags(SettingsBindFlags::DEFAULT)
            .build();

        let expand_mode = self.imp().expand_mode.get();
        self.settings()
            .bind("expand-mode", &expand_mode, "active")
            .flags(SettingsBindFlags::DEFAULT)
            .build();

        let cell_max_width = self.imp().cell_max_width.get();
        self.settings()
            .bind("cell-max-width", &cell_max_width, "value")
            .flags(SettingsBindFlags::DEFAULT)
            .build();
        let line_max_width = self.imp().line_max_width.get();
        self.settings()
            .bind("line-max-width", &line_max_width, "value")
            .flags(SettingsBindFlags::DEFAULT)
            .build();
        let gfm_table_enable = self.imp().gfm_table_enable.get();
        self.settings()
            .bind("gfm-table-enable", &gfm_table_enable, "active")
            .flags(SettingsBindFlags::DEFAULT)
            .build();
    }

    pub(crate) fn connect_font_changed<F: Fn(&Self) + 'static>(
        &self,
        f: F,
    ) -> glib::SignalHandlerId {
        self.connect_local("font-changed", true, move |values| {
            let obj = values[0].get().unwrap();
            f(obj);
            None
        })
    }
}

impl Default for MainPreferences {
    fn default() -> Self {
        Self::new()
    }
}
