use crate::core::AppSettings;
use adw::ResponseAppearance;
use adw::prelude::{
    ActionRowExt, AdwDialogExt, AlertDialogExt, ComboRowExt, PreferencesGroupExt, PreferencesRowExt,
};
use gettextrs::gettext;
use gtk::glib::clone;
use gtk::glib::object::Cast;
use gtk::prelude::{ActionableExt, ActionableExtManual, ButtonExt, WidgetExt};
use gtk::{CompositeTemplate, glib, subclass::prelude::*, *};

mod imp {

    use std::{cell::RefCell, sync::OnceLock};

    use adw::subclass::{dialog::AdwDialogImpl, prelude::PreferencesDialogImpl};
    use glib::subclass::Signal;

    use super::*;

    #[derive(Debug, Default, CompositeTemplate)]
    #[template(resource = "/com/github/nobodygx/asciibox/ui/preferences.ui")]
    pub struct MainPreferences {
        #[template_child]
        pub lang_combox: TemplateChild<adw::ComboRow>,
        #[template_child]
        pub expand_mode: TemplateChild<gtk::Switch>,
        #[template_child]
        pub cell_max_width: TemplateChild<adw::SpinRow>,
        #[template_child]
        pub line_max_width: TemplateChild<adw::SpinRow>,
        #[template_child]
        pub mermaid_group: TemplateChild<adw::PreferencesGroup>,

        pub settings: RefCell<AppSettings>,
    }

    #[glib::object_subclass]
    impl ObjectSubclass for MainPreferences {
        const NAME: &'static str = "MainPreferences";
        type Type = super::MainPreferences;
        type ParentType = adw::PreferencesDialog;

        fn class_init(klass: &mut Self::Class) {
            klass.bind_template();

            klass.install_action_async(
                "preference.modify-theme",
                Some(glib::VariantTy::STRING),
                move |obj, _, param| async move {
                    let var = param.unwrap().get::<String>().unwrap();
                    obj.modify_theme(&var).await;
                },
            );
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
            obj.bind_settings();
        }

        fn signals() -> &'static [Signal] {
            static SIGNALS: OnceLock<Vec<Signal>> = OnceLock::new();
            SIGNALS.get_or_init(|| vec![Signal::builder("lang-changed").build()])
        }
    }
    impl WidgetImpl for MainPreferences {}
    impl WindowImpl for MainPreferences {}
    impl AdwDialogImpl for MainPreferences {}
    impl PreferencesDialogImpl for MainPreferences {}
}

glib::wrapper! {
    pub struct MainPreferences(ObjectSubclass<imp::MainPreferences>)
        @extends Widget, adw::Dialog, adw::PreferencesDialog,
        @implements Accessible;
}

impl MainPreferences {
    pub fn new() -> Self {
        glib::Object::new()
    }

    fn setup_settings(&self) {
        let settings = AppSettings::get();

        let imp = self.imp();
        // 初始化 lang_combox
        {
            // init lang default item
            let lang_combox = imp.lang_combox.get();
            let pomap = AppSettings::po_map();
            let mut position = 0;
            for (i, v) in pomap.values().enumerate() {
                if v.eq_ignore_ascii_case(&settings.general.lang) {
                    position = i;
                    break;
                }
            }
            lang_combox.set_selected(position as u32);
        }

        // 初始化 mermaid_group
        {
            let mgroup = imp.mermaid_group.get();
            for name in settings.mermaid.theme_styles.keys() {
                let arow = adw::ActionRow::new();
                let btn = gtk::Button::new();
                btn.set_icon_name("isettings");
                btn.add_css_class("flat");
                arow.add_suffix(&btn);
                btn.set_action_name(Some("preference.modify-theme"));
                btn.set_action_target(Some(format!("'{}'", name)));
                arow.set_title(name);
                mgroup.add(&arow);
            }
        }
    }

    fn select_lang(&self, key: glib::GString) {
        let key = key.as_str();
        let pomap = AppSettings::po_map();
        if !pomap.contains_key(&key) {
            print!("no translate po find for {}", key);
            return;
        }
        let value = pomap.get(key).map_or("", |v| v);
        let mut settings = self.imp().settings.borrow_mut();
        if settings.general.lang.eq_ignore_ascii_case(value) {
            return;
        }
        settings.general.lang = value.to_string();
        settings.save();

        let ok_response = "ok";
        let dialog = adw::AlertDialog::builder()
            .body(&gettext(
                "Please restart the application for the changes to take effect",
            ))
            .default_response(ok_response)
            .build();
        dialog.add_response(ok_response, &gettext("ok"));
        dialog.set_response_appearance(ok_response, ResponseAppearance::Suggested);
        dialog.present(Some(
            self.root().unwrap().downcast_ref::<gtk::Window>().unwrap(),
        ));
    }

    fn bind_settings(&self) {
        let imp = self.imp();
        let lang_combox = imp.lang_combox.get();
        lang_combox.connect_selected_notify(clone!(
            #[weak(rename_to = pw)]
            self,
            move |dd| {
                let item = dd.selected_item().unwrap();
                let s = item.downcast_ref::<StringObject>().expect("string_object");
                pw.select_lang(s.string());
            }
        ));
    }

    async fn modify_theme(&self, theme: &String) {
        let dialog = adw::AlertDialog::new(Some(theme), Some("body"));
        dialog.present(Some(
            self.root().unwrap().downcast_ref::<gtk::Window>().unwrap(),
        ));
    }
}

impl Default for MainPreferences {
    fn default() -> Self {
        Self::new()
    }
}
