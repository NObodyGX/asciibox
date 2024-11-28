use adw::subclass::prelude::AdwWindowImpl;
use adw::subclass::prelude::PreferencesWindowImpl;
use gtk::glib::clone;
use gtk::{glib, prelude::*, subclass::prelude::*, CompositeTemplate, *};

use crate::core::config::Config;

mod imp {

    use std::{cell::RefCell, sync::OnceLock};

    use glib::subclass::Signal;

    use super::*;

    #[derive(Debug, Default, CompositeTemplate)]
    #[template(resource = "/com/github/nobodygx/asciibox/ui/preferences.ui")]
    pub struct MainPreferences {
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

        pub config: RefCell<Config>,
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

            obj.setup_config();
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

    fn setup_config(&self) {
        let iconfig = Config::new();
        let mut config = self.imp().config.borrow_mut();
        config.clone_from(&iconfig);
    }
    fn bind_settings(&self) {
        // 注意: schema 里不能使用 _ 而是需要使用 - 才符合格式
        let imp = self.imp();
        let fdesc = imp.config.borrow().custom_font.clone();
        imp.font
            .set_font_desc(&gtk::pango::FontDescription::from_string(fdesc.as_str()));
        imp.font.connect_font_desc_notify(clone!(
            #[weak]
            imp,
            move |_| {
                let font_string = imp.font.font_desc().unwrap().to_string();

                let mut config = imp.config.borrow_mut();
                config.custom_font = font_string;
                config.save();

                imp.obj().emit_by_name::<()>("font-changed", &[]);
            },
        ));

        let use_custom_font = imp.use_custom_font.get();
        use_custom_font.connect_active_notify(clone!(
            #[weak]
            imp,
            move |switch| {
                let mut config = imp.config.borrow_mut();
                config.use_custom_font = switch.is_active();
                config.save();
            },
        ));

        let expand_mode = imp.expand_mode.get();
        expand_mode.connect_active_notify(clone!(
            #[weak]
            imp,
            move |switch| {
                let mut config = imp.config.borrow_mut();
                config.flowchart.expand_mode = switch.is_active();
                config.save();
            },
        ));

        let cell_max_width = imp.cell_max_width.get();
        cell_max_width.connect_value_notify(clone!(
            #[weak]
            imp,
            move |spin| {
                let mut config = imp.config.borrow_mut();
                config.table.cell_max_width = spin.value() as i32;
                config.save();
            },
        ));

        let line_max_width = imp.line_max_width.get();
        line_max_width.connect_value_notify(clone!(
            #[weak]
            imp,
            move |spin| {
                let mut config = imp.config.borrow_mut();
                config.table.line_max_width = spin.value() as i32;
                config.save();
            },
        ));
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
