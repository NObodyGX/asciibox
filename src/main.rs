mod application;
mod config;
mod gui;

use gtk::prelude::*;
use gtk::{gio, glib};
use application::AsciiboxApplication;

const APP_ID: &str = "com.github.nobodygx.asciibox";
const APP_NAME: &str = "asciibox";
use config::PKGDATADIR;


mod core;
use crate::core::svgbob::GSMap;
fn do_svgbob_main() {
    let a = "a[a1] --> b[b1]";
    // let a = concat!(
    //     "a --> b\n",
    //     "a[a1]\n",
    //     "b[b1]",
    // );
    let mut mmap = GSMap::new();
    let _ = mmap.load_content(a);
}

fn main() { // -> glib::ExitCode {
    // Register and include resources
    // let resources =
    //     gio::Resource::load(PKGDATADIR.to_owned() + "/asciibox.gresource")
    //         .expect("Could not load resources");
    // gio::resources_register(&resources);

    // // Create a new application
    // let app = AsciiboxApplication::new(APP_ID, &gio::ApplicationFlags::empty());

    // app.connect_startup(|app| {
    //     setup_shortcuts(app);
    // });

    // app.run()

    do_svgbob_main();
}

fn setup_shortcuts(app: &AsciiboxApplication) {
    app.set_accels_for_action("win.execute", &["<Ctrl>r"]);
    app.set_accels_for_action("win.clearall", &["<Ctrl>l"]);
}