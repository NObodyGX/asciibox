use adw;
use gtk::{glib, subclass::prelude::*};

mod imp {
    use super::*;

    #[derive(gtk::CompositeTemplate)]
    #[template(resource = "/com/github/nobodygx/asciibox/ui/theme_switcher.ui")]
    pub struct ThemeSwitcher {
        #[template_child]
        pub system_button: TemplateChild<gtk::CheckButton>,
        #[template_child]
        pub light_button: TemplateChild<gtk::CheckButton>,
        #[template_child]
        pub dark_button: TemplateChild<gtk::CheckButton>,
        #[template_child]
        pub sepia_button: TemplateChild<gtk::CheckButton>,
    }

    impl Default for ThemeSwitcher {
        fn default() -> Self {
            Self {
                system_button: TemplateChild::default(),
                light_button: TemplateChild::default(),
                dark_button: TemplateChild::default(),
                sepia_button: TemplateChild::default(),
            }
        }
    }

    #[glib::object_subclass]
    impl ObjectSubclass for ThemeSwitcher {
        const NAME: &'static str = "ThemeSwitcher";
        type Type = super::ThemeSwitcher;
        type ParentType = gtk::Box;

        fn class_init(klass: &mut Self::Class) {
            klass.bind_template();
            klass.set_css_name("theme-switcher");

            klass.install_action(
                "theme_switcher.switch-theme",
                Some(glib::VariantTy::STRING),
                move |obj, _, param| {
                    let var = param.unwrap().get::<String>().unwrap();
                    obj.switch_theme(&var);
                },
            );
        }

        fn instance_init(obj: &glib::subclass::InitializingObject<Self>) {
            obj.init_template();
        }
    }

    impl ObjectImpl for ThemeSwitcher {
        fn constructed(&self) {
            self.parent_constructed();
        }
    }

    impl WidgetImpl for ThemeSwitcher {}

    impl BoxImpl for ThemeSwitcher {}
}

glib::wrapper! {
    pub struct ThemeSwitcher(ObjectSubclass<imp::ThemeSwitcher>)
        @extends gtk::Box, gtk::Widget,
        @implements gtk::Buildable;
}

impl ThemeSwitcher {
    pub fn new() -> Self {
        glib::Object::builder().build()
    }

    fn switch_theme(&self, theme: &String) {
        let style = adw::StyleManager::default();

        let color_scheme = match theme.as_str() {
            "system" => adw::ColorScheme::Default,
            "light" => adw::ColorScheme::ForceLight,
            "dark" => adw::ColorScheme::ForceDark,
            _ => adw::ColorScheme::Default,
        };
        style.set_color_scheme(color_scheme);
    }
}
