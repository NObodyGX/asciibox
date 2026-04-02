mod gui;

use adw::prelude::*;
use gtk::gio;

fn main() {
    adw::init().expect("Failed to initialize libadwaita");
    let app = adw::Application::new(
        Some("com.github.NObodyGX.Asciibox"),
        gio::ApplicationFlags::FLAGS_NONE,
    );

    let style = app.style_manager();

    style.set_color_scheme(adw::ColorScheme::PreferDark);

    app.connect_activate(|app| {
        let window = gui::AsciiboxWindow::new(app);
        window.set_application(Some(app));
        window.present();
    });

    std::process::exit(app.run().into());
}
