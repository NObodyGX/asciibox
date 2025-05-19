use crate::config::{APP_ID, APP_NAME, APP_PATH_ID, APP_URL, VERSION};
use crate::gui::ShortcutsWindow;
use crate::gui::{MainPreferences, MainWindow};
use adw::prelude::*;
use adw::subclass::prelude::*;
use gettextrs::gettext;
use gtk::CssProvider;
use gtk::{gdk, gio, glib};

mod imp {

    use std::cell::OnceCell;

    use super::*;

    #[derive(Debug, Default)]
    pub struct BasicApplication {
        pub provicder: OnceCell<CssProvider>,
    }

    #[glib::object_subclass]
    impl ObjectSubclass for BasicApplication {
        const NAME: &'static str = "BasicApplication";
        type Type = super::BasicApplication;
        type ParentType = adw::Application;
    }

    impl ObjectImpl for BasicApplication {
        fn constructed(&self) {
            let obj = self.obj();
            self.parent_constructed();

            obj.setup_gactions();
            obj.setup_style();
        }
    }
    impl ApplicationImpl for BasicApplication {
        fn activate(&self) {
            self.parent_activate();

            let obj = self.obj();
            let app = obj.downcast_ref::<super::BasicApplication>().unwrap();
            let window = app.create_window();
            window.set_title(Some(APP_NAME));
            window.set_icon_name(Some(APP_NAME));
            window.present();
        }
    }
    impl AdwApplicationImpl for BasicApplication {}
    impl GtkApplicationImpl for BasicApplication {}
}

glib::wrapper! {
    pub struct BasicApplication(ObjectSubclass<imp::BasicApplication>)
        @extends adw::Application, gio::Application, gtk::Application,
        @implements gio::ActionGroup, gio::ActionMap;
}

impl BasicApplication {
    pub fn new(application_id: &str, flags: &gio::ApplicationFlags) -> Self {
        glib::Object::builder()
            .property("application-id", application_id)
            .property("flags", flags)
            .build()
    }

    pub fn css_provider(&self) -> &CssProvider {
        self.imp().provicder.get_or_init(CssProvider::new)
    }

    fn create_window(&self) -> MainWindow {
        let window = MainWindow::new(&self);

        window.present();
        window
    }

    fn setup_gactions(&self) {
        let preferences_action = gio::SimpleAction::new("preferences", None);
        preferences_action.connect_activate(glib::clone!(
            #[weak(rename_to = app)]
            self,
            move |_action, _parameter| {
                app.show_prefrerences();
            }
        ));
        self.add_action(&preferences_action);

        let quit_action = gio::SimpleAction::new("quit", None);
        quit_action.connect_activate(glib::clone!(
            #[weak(rename_to = app)]
            self,
            move |_action, _parameter| {
                app.quit();
            }
        ));
        self.add_action(&quit_action);

        let about_action = gio::SimpleAction::new("about", None);
        about_action.connect_activate(glib::clone!(
            #[weak(rename_to = app)]
            self,
            move |_action, _parameter| {
                app.show_about();
            }
        ));
        self.add_action(&about_action);

        let shortcuts_action = gio::SimpleAction::new("show-shortcuts", None);
        shortcuts_action.connect_activate(glib::clone!(
            #[weak(rename_to = app)]
            self,
            move |_action, _parameter| {
                app.show_help_shortcuts();
            }
        ));
        self.add_action(&shortcuts_action);
    }

    fn setup_style(&self) {
        let provider = self.css_provider();
        let fpath = format!("{}/css/app.css", APP_PATH_ID);
        provider.load_from_resource(fpath.as_str());
        gtk::style_context_add_provider_for_display(
            &gdk::Display::default().expect("Could not connect to a display."),
            provider,
            gtk::STYLE_PROVIDER_PRIORITY_APPLICATION,
        );
    }

    fn show_prefrerences(&self) {
        let window = self.active_window().unwrap();
        let preferences = MainPreferences::new();
        preferences.present(Some(&window));
    }

    fn show_about(&self) {
        let mut comments = String::new();
        comments.push_str("<b>");
        comments.push_str(&gettext("Asciibox"));
        comments.push_str("</b>\n\n");
        comments.push_str(&gettext("asciibox is an auxiliary tool intended to simplify ascii text manipulation, include svgbob and asciidoc"));

        let window = self.active_window().unwrap();
        let dialog = adw::AboutDialog::builder()
            .application_icon(APP_NAME)
            .application_name(APP_NAME)
            .version(VERSION)
            .developer_name("NObodyGX")
            .developers(vec!["NObodyGX"])
            .designers(vec!["NObodyGX"])
            .artists(vec!["NObodyGX"])
            .license_type(gtk::License::MitX11)
            .website(APP_URL)
            .issue_url(format!("{APP_URL}/issues"))
            .comments(comments)
            .copyright("© 2024-2025 NObodyGX")
            .build();
        dialog.present(Some(&window));
    }

    fn show_help_shortcuts(&self) {
        let window = self.active_window().unwrap();
        let dialog = ShortcutsWindow::default();
        dialog.present(Some(&window));
    }
}

impl Default for BasicApplication {
    fn default() -> Self {
        glib::Object::builder()
            .property("application-id", APP_ID)
            .build()
    }
}
