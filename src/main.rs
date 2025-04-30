mod application;
mod config;
mod core;
mod gui;
use application::AsciiboxApplication;
use fork::{daemon, Fork};
use gettextrs::LocaleCategory;
use gtk::prelude::*;
use gtk::{gio, glib};
use rust_embed::Embed;

#[derive(Embed)]
#[folder = "data/bin"]
struct Asset;

use config::{APP_ID, APP_NAME, PKGDATA_DIR};

fn init_resource() -> bool {
    let res_name = format!("{}.gresource", APP_NAME);
    let res_name = &res_name;
    let resource = if Asset::get(res_name).is_some() {
        let emfile = Asset::get(res_name).unwrap();
        let emdata = emfile.data.into_owned();
        let data = glib::Bytes::from_owned(emdata);
        gio::Resource::from_data(&data).unwrap()
    } else {
        gio::Resource::load(PKGDATA_DIR.to_owned() + "/" + res_name).unwrap()
    };
    gio::resources_register(&resource);
    return true;
}

fn init_i18n() {
    gettextrs::setlocale(LocaleCategory::LcAll, "");
    gettextrs::bindtextdomain(config::APP_NAME, config::LOCALE_DIR)
        .expect("Unable to bind the text domain");
    gettextrs::textdomain(config::APP_NAME).expect("Unable to switch to the text domain");
}

fn do_main_run() -> glib::ExitCode {
    init_resource();
    init_i18n();

    let app = AsciiboxApplication::new(APP_ID, &gio::ApplicationFlags::empty());

    app.connect_startup(|app| {
        setup_shortcuts(app);
    });

    app.run()
}

fn main() -> glib::ExitCode {
    match daemon(false, true) {
        Ok(Fork::Child) => do_main_run(),
        Ok(Fork::Parent(pid)) => {
            println!("daemon pid: {}", pid);
            return glib::ExitCode::from(0);
        }
        Err(_) => {
            println!("Fork failed");
            return glib::ExitCode::from(1);
        }
    }
}

fn setup_shortcuts(app: &AsciiboxApplication) {
    app.set_accels_for_action("app.quit", &["<Ctrl>q"]);
    app.set_accels_for_action("win.execute", &["<Ctrl>r"]);
    app.set_accels_for_action("win.clear_all", &["<Ctrl>BackSpace"]);
    app.set_accels_for_action("win.switch_tab", &["<Ctrl>h"]);
}
