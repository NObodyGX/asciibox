use std::{
    fs::{self, OpenOptions},
    io::{Read, Write},
    path::PathBuf,
    sync::{LazyLock, RwLock, RwLockReadGuard, RwLockWriteGuard},
};

use crate::config::APP_NAME;
use indexmap::IndexMap;
use serde::{Deserialize, Serialize};
use toml;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct General {
    #[serde(default = "default_lang")]
    pub lang: String,
}

fn default_lang() -> String {
    String::new()
}

impl Default for General {
    fn default() -> Self {
        General {
            lang: default_lang(),
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Flowchart {
    #[serde(default = "default_expand_mode")]
    pub expand_mode: bool,
}

fn default_expand_mode() -> bool {
    false
}

impl Default for Flowchart {
    fn default() -> Self {
        Flowchart {
            expand_mode: default_expand_mode(),
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Table {
    #[serde(default = "default_cell_max_width")]
    pub cell_max_width: i32,
    #[serde(default = "default_line_max_width")]
    pub line_max_width: i32,
}

fn default_cell_max_width() -> i32 {
    99
}

fn default_line_max_width() -> i32 {
    299
}

impl Default for Table {
    fn default() -> Self {
        Table {
            cell_max_width: default_cell_max_width(),
            line_max_width: default_line_max_width(),
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Mermaid {
    #[serde(default = "default_mermaid_theme")]
    pub theme: String,
}

fn default_mermaid_theme() -> String {
    String::from("default")
}

impl Default for Mermaid {
    fn default() -> Self {
        Mermaid {
            theme: default_mermaid_theme(),
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct AppSettings {
    #[serde(default)]
    pub general: General,
    #[serde(default)]
    pub flowchart: Flowchart,
    #[serde(default)]
    pub table: Table,
    #[serde(default)]
    pub mermaid: Mermaid,
    #[serde(skip)]
    pub changed: bool, // default to false
}

static APP_SETTINGS_VAR: LazyLock<RwLock<AppSettings>> =
    LazyLock::new(|| RwLock::new(AppSettings::load()));

impl AppSettings {
    /// 判断配置是否发生变化
    pub fn is_changed(&self) -> bool {
        self.changed
    }

    pub fn set_changed(&mut self) {
        self.changed = true;
    }

    /// 获取当前 AppSettings 的引用（此时才会初始化）
    pub fn get() -> RwLockReadGuard<'static, AppSettings> {
        return APP_SETTINGS_VAR.read().unwrap();
    }

    /// 获取当前 AppSettings 可写引用
    pub fn get_mut() -> RwLockWriteGuard<'static, AppSettings> {
        return APP_SETTINGS_VAR.write().unwrap();
    }

    /// 保存当前配置
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

    fn filename() -> PathBuf {
        let home = homedir::my_home().unwrap().unwrap();
        let filename = home
            .join(".config")
            .join(APP_NAME)
            .join(APP_NAME)
            .with_extension("toml");
        return filename;
    }

    fn load() -> AppSettings {
        let filename = AppSettings::filename();
        if !filename.exists() {
            return AppSettings::default();
        }

        let mut file: std::fs::File = OpenOptions::new().read(true).open(&filename).unwrap();
        let mut contents = String::new();
        file.read_to_string(&mut contents).unwrap();
        let settings = match toml::from_str(&contents) {
            Ok(settings) => settings,
            Err(e) => {
                log::error!(
                    "error to deserialize {filename:#?}: {e}\n========use deault setting========"
                );
                AppSettings::default()
            }
        };
        settings
    }
}
