mod application;
mod config;
mod core;
mod gui;
use application::AsciiboxApplication;
use gettextrs::LocaleCategory;
use gtk::prelude::*;
use gtk::{gio, glib};
use rust_embed::Embed;

#[derive(Embed)]
#[folder = "data"]
struct Asset;

use config::{APP_ID, PKGDATA_DIR};

fn load_resource() -> gio::Resource {
    let fname = "asciibox.gresource";
    let resource = if Asset::get(fname).is_some() {
        let emfile = Asset::get(fname).unwrap();
        let emdata = emfile.data.into_owned();
        let data = glib::Bytes::from_owned(emdata);
        gio::Resource::from_data(&data).unwrap()
    } else {
        gio::Resource::load(PKGDATA_DIR.to_owned() + "/" + fname).unwrap()
    };
    return resource;
}

fn main() -> glib::ExitCode {
    // Register and include resources

    let resources = load_resource();
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
