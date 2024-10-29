mod application;
mod config;
mod core;
mod gui;
use application::AsciiboxApplication;
use gettextrs::LocaleCategory;
use gtk::prelude::*;
use gtk::{gio, glib};

use config::{APP_ID, PKGDATA_DIR};

fn main() -> glib::ExitCode {
    // Register and include resources
    let resources = gio::Resource::load(PKGDATA_DIR.to_owned() + "/asciibox.gresource")
        .expect("Could not load resources");
    gio::resources_register(&resources);

    // Prepare i18n
    gettextrs::setlocale(LocaleCategory::LcAll, "");
    gettextrs::bindtextdomain(config::APP_NAME, config::LOCALE_DIR)
        .expect("Unable to bind the text domain");
    gettextrs::textdomain(config::APP_NAME).expect("Unable to switch to the text domain");

    // Create a new application
    let app = AsciiboxApplication::new(APP_ID, &gio::ApplicationFlags::empty());

    app.connect_startup(|app| {
        setup_shortcuts(app);
    });

    app.run()
}

fn setup_shortcuts(app: &AsciiboxApplication) {
    app.set_accels_for_action("app.quit", &["<Ctrl>q"]);
    app.set_accels_for_action("win.execute", &["<Ctrl>r"]);
    app.set_accels_for_action("win.clear_all", &["<Ctrl>BackSpace"]);
    app.set_accels_for_action("win.switch_tab", &["<Ctrl>h"]);
}
