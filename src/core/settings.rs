use std::{
    fs::{self, OpenOptions},
    io::{Read, Write},
    path::PathBuf,
};

use crate::config::APP_NAME;
use indexmap::IndexMap;
use serde::{Deserialize, Serialize};
use toml;

#[derive(Serialize, Deserialize, Debug, Clone, Copy)]
#[serde(deny_unknown_fields)]
pub struct Flowchart {
    pub expand_mode: bool,
}

#[derive(Serialize, Deserialize, Debug, Clone, Copy)]
#[serde(deny_unknown_fields)]
pub struct Table {
    pub cell_max_width: i32,
    pub line_max_width: i32,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct AppSettings {
    pub lang: String,
    pub flowchart: Flowchart,
    pub table: Table,
}

impl Default for AppSettings {
    fn default() -> Self {
        AppSettings {
            lang: "".to_string(),

            flowchart: Flowchart { expand_mode: false },

            table: Table {
                cell_max_width: 99,
                line_max_width: 299,
            },
        }
    }
}

impl AppSettings {
    fn filename() -> PathBuf {
        let home = homedir::my_home().unwrap().unwrap();
        let filename = home
            .join(".config")
            .join(APP_NAME)
            .join(APP_NAME)
            .with_extension("toml");
        return filename;
    }

    pub fn new() -> AppSettings {
        let filename = AppSettings::filename();
        if !filename.exists() {
            return AppSettings::default();
        }

        let mut file: std::fs::File = OpenOptions::new().read(true).open(filename).unwrap();
        let mut contents = String::new();
        file.read_to_string(&mut contents).unwrap();
        let settings: AppSettings = toml::from_str(&contents).unwrap();
        settings
    }

    pub fn save(&self) {
        let filename = AppSettings::filename();
        let toml = toml::to_string(self).unwrap();

        if !filename.parent().unwrap().exists() {
            fs::create_dir_all(filename.parent().unwrap()).unwrap();
        }
        // 注意不truncate会导致覆盖不完全
        let mut file: std::fs::File = OpenOptions::new()
            .write(true)
            .truncate(true)
            .create(true)
            .open(filename)
            .unwrap();
        file.write_all(toml.as_bytes()).unwrap();
    }

    /// 支持的多语言选项
    pub fn po_map() -> IndexMap<&'static str, &'static str> {
        let mut hashmap: IndexMap<&'static str, &'static str> = Default::default();
        hashmap.insert("english", "en_US.UTF-8");
        hashmap.insert("简体中文", "zh_CN.UTF-8");

        hashmap
    }

}
