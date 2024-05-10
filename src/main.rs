mod collection_object;
mod task_object;
mod utils;
mod window;

use adw::prelude::*;
use gtk::{gio, glib};
use window::Window;

const APP_ID: &str = "com.github.nobodygx.asciibox";
pub static PKGDATADIR: &str = "../data";

// ANCHOR: main
fn main() -> glib::ExitCode {
    // Register and include resources
    let resources = gio::Resource::load(PKGDATADIR.to_owned() + "/asciibox.gresource")
        .expect("Could not load resources");
    gio::resources_register(&resources);

    // Create a new application
    let app = adw::Application::builder().application_id(APP_ID).build();

    // Connect to "activate" signal of `app`
    app.connect_startup(|app| {
        setup_shortcuts(app);
    });
    app.connect_activate(build_ui);

    // Run the application
    app.run()
}

fn setup_shortcuts(app: &adw::Application) {
    app.set_accels_for_action("win.filter('All')", &["<Ctrl>a"]);
    app.set_accels_for_action("win.filter('Open')", &["<Ctrl>o"]);
    app.set_accels_for_action("win.filter('Done')", &["<Ctrl>d"]);
}

fn build_ui(app: &adw::Application) {
    // Create a new custom window and present it
    let window = Window::new(app);
    window.present();
}