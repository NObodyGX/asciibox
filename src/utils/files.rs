use std::path::PathBuf;
use std::{fs::OpenOptions, io::Write};

pub fn save_file(filename: &PathBuf, content: &[u8]) {
    match OpenOptions::new().write(true).create(true).open(&filename) {
        Ok(mut f) => f.write_all(content).expect("write error"),
        Err(e) => {
            log::error!("create file error in {filename:#?}: {e}")
        }
    }
}
