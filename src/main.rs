mod application;
mod config;
mod gui;

use gtk::prelude::*;
use gtk::{gio, glib};
use application::AsciiboxApplication;

const APP_ID: &str = "com.github.nobodygx.asciibox";
const APP_NAME: &str = "asciibox";

// only for local temp test
pub static PKGDATADIR: &str = "../data";

// ANCHOR: main
fn main() -> glib::ExitCode {
    // Register and include resources
    let resources = gio::Resource::load(PKGDATADIR.to_owned() + "/asciibox.gresource")
        .expect("Could not load resources");
    gio::resources_register(&resources);

    // Create a new application
    let app = AsciiboxApplication::new(APP_ID, &gio::ApplicationFlags::empty());

    app.connect_startup(|app| {
        setup_shortcuts(app);
    });

    app.run()
}

fn setup_shortcuts(app: &AsciiboxApplication) {
    app.set_accels_for_action("win.execute", &["<Ctrl>r"]);
    app.set_accels_for_action("win.clearall", &["<Ctrl>l"]);
}