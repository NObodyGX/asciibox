use std::{fs::OpenOptions, io::Write};

use sourceview::prelude::FileExt;

pub async fn save_dialog(
    window: &gtk::Window,
    title: &str,
    content: &String,
    extension: Option<String>,
) {
    let dialog = gtk::FileDialog::builder()
        .title(title)
        .accept_label("Save")
        .modal(true)
        .build();

    let ofile = dialog.save_future(Some(window)).await;
    if ofile.is_err() {
        log::error!("dialog error in : {ofile:#?}");
        return;
    }
    let ofile = ofile.unwrap();
    let filename = ofile.path();
    if filename.is_none() {
        log::error!("get ofile error");
        return;
    }
    let mut filename = filename.unwrap();
    if extension.is_some() {
        let extension = extension.unwrap();
        if !filename.ends_with(&extension) {
            filename.set_extension(&extension);
        }
    }
    match OpenOptions::new().write(true).create(true).open(&filename) {
        Ok(mut f2) => f2.write_all(content.as_bytes()).expect("write error"),
        Err(e) => {
            log::error!("create file error in {filename:#?}: {e}")
        }
    }
}
