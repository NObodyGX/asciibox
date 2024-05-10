mod task_object;
mod task_row;
mod window;

use gtk::prelude::*;
use gtk::{gio, glib, Application};
use window::Window;

pub static PKGDATADIR: &str = "../data";

// ANCHOR: main
fn main() -> glib::ExitCode {
    // Register and include resources
    let resources = gio::Resource::load(PKGDATADIR.to_owned() + "/asciibox.gresource")
        .expect("Could not load resources");
    gio::resources_register(&resources);

    // Create a new application
    let app = Application::builder()
        .application_id("com.github.nobodygx.asciibox")
        .build();

    // Connect to "activate" signal of `app`
    app.connect_activate(build_ui);

    // Run the application
    app.run()
}

fn build_ui(app: &Application) {
    // Create a new custom window and present it
    let window = Window::new(app);
    window.present();
}
// ANCHOR_END: main
