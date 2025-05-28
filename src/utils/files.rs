use std::fs;
use std::io::Read;
use std::path::PathBuf;
use std::{fs::OpenOptions, io::Write};

pub fn save_file(filename: &PathBuf, content: &[u8]) {
    match OpenOptions::new()
        .write(true)
        .create(true)
        .truncate(true)
        .open(&filename)
    {
        Ok(mut f) => f.write_all(content).expect("write error"),
        Err(e) => {
            log::error!("create file error in {filename:#?}: {e}")
        }
    }
}

pub fn read_text(filename: &PathBuf) -> String {
    let mut file: std::fs::File = OpenOptions::new()
        .read(true)
        .open(&filename)
        .expect("Failed to open file");
    let mut contents = String::new();
    file.read_to_string(&mut contents)
        .expect("Failed to read file contents");

    contents
}

pub fn list_files_in_dir(path: &PathBuf, extensions: &Vec<&str>) -> Vec<PathBuf> {
    let mut result: Vec<PathBuf> = Vec::new();
    if !path.exists() {
        return result;
    }
    if let Ok(entries) = fs::read_dir(path) {
        for entry in entries {
            if entry.is_err() {
                continue;
            }
            let entry = entry.unwrap();
            if !entry.file_type().unwrap().is_file() {
                continue;
            }
            let fpath = entry.path();

            if extensions.len() == 0 {
                result.push(fpath);
                continue;
            }
            if fpath.extension().is_none() {
                continue;
            }
            let ext = fpath.extension().unwrap();
            if let Some(ext_str) = ext.to_str() {
                if extensions.contains(&ext_str) {
                    result.push(fpath);
                }
            }
        }
    }
    result
}
