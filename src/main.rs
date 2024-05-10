mod application;
mod gui;

use application::AsciiboxApplication;
use gtk::{gio, glib, prelude::*};

// temp test
pub static PKGDATADIR: &str = "../data";

fn main() -> glib::ExitCode {
    // Load resources
    let resources = gio::Resource::load(PKGDATADIR.to_owned() + "/asciibox.gresource")
        .expect("Could not load resources");
    gio::resources_register(&resources);

    let app = AsciiboxApplication::default();
    app.run()
}
