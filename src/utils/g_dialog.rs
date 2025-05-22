use sourceview::prelude::FileExt;

use super::save_file;

pub async fn save_dialog(
    window: &gtk::Window,
    title: &str,
    content: &[u8],
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
    save_file(&filename, content);
}
