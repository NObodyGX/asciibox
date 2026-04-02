use std::fs;
use std::{
    collections::BTreeMap,
    path::PathBuf,
    sync::{LazyLock, RwLock, RwLockReadGuard, RwLockWriteGuard},
};

use crate::config::APP_NAME;
use crate::utils;
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
pub struct MermaidStyle {
    #[serde(default = "default_dark_mode")]
    pub dark_mode: bool,
    #[serde(default = "default_white_color")]
    pub background: String,
    #[serde(default = "default_font")]
    pub font_family: String,
    #[serde(default = "default_font_size")]
    pub font_size: i32,
    #[serde(default = "default_white_color")]
    pub primary_color: String,
    #[serde(default = "default_black_color")]
    pub primary_border_color: String,
    #[serde(default = "default_black_color")]
    pub primary_text_color: String,
    #[serde(default = "default_black_color")]
    pub line_color: String,
    #[serde(default = "default_green_color")]
    pub secondary_color: String,
    #[serde(default = "default_red_color")]
    pub tertiary_color: String,
}

fn default_dark_mode() -> bool {
    false
}

fn default_white_color() -> String {
    String::from("#f4f4f4")
}

fn default_black_color() -> String {
    String::from("#000000")
}

fn default_green_color() -> String {
    String::from("#00f33d")
}

fn default_red_color() -> String {
    String::from("#c30000")
}

fn default_font() -> String {
    String::from("Maple Mono NF CN")
}

fn default_font_size() -> i32 {
    14
}

impl Default for MermaidStyle {
    fn default() -> Self {
        MermaidStyle {
            dark_mode: default_dark_mode(),
            background: default_white_color(),
            font_family: default_font(),
            font_size: default_font_size(),
            primary_color: default_white_color(),
            primary_border_color: default_black_color(),
            primary_text_color: default_black_color(),
            line_color: default_black_color(),
            secondary_color: default_green_color(),
            tertiary_color: default_red_color(),
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Mermaid {
    #[serde(default = "default_mermaid_theme")]
    pub theme: String,
    pub theme_styles: BTreeMap<String, MermaidStyle>,
}

fn default_mermaid_theme() -> String {
    String::from("default")
}

impl Default for Mermaid {
    fn default() -> Self {
        Mermaid {
            theme: default_mermaid_theme(),
            theme_styles: BTreeMap::default(),
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

    pub fn get_theme(&self, name: &str) -> Option<&MermaidStyle> {
        self.mermaid.theme_styles.get(name)
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
        utils::save_file(&filename, toml.as_bytes());
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

    fn theme_dir() -> PathBuf {
        let home = homedir::my_home().unwrap().unwrap();
        let dirname = home.join(".config").join(APP_NAME).join("themes");
        return dirname;
    }

    #[allow(dead_code)]
    fn mermaid_theme_dir() -> PathBuf {
        let dirname = AppSettings::theme_dir();
        let dirname = dirname.join("mermaid");
        return dirname;
    }

    fn load() -> AppSettings {
        let filename = AppSettings::filename();
        if !filename.exists() {
            return AppSettings::default();
        }

        let content = utils::read_text(&filename);
        let mut settings: AppSettings = match toml::from_str(&content) {
            Ok(settings) => settings,
            Err(e) => {
                log::error!(
                    "error to deserialize {filename:#?}: {e}\n========use deault setting========"
                );
                AppSettings::default()
            }
        };

        let dirname = AppSettings::theme_dir();
        if !dirname.exists() {
            return settings;
        }
        // 读取主题配置

        // 读取 mermaid 主题配置
        let mdir = AppSettings::mermaid_theme_dir();
        let extensions = vec!["toml"];
        let mlist = utils::list_files_in_dir(&mdir, &extensions);
        for mfile in mlist.iter() {
            let mcontent = utils::read_text(&mfile);
            match toml::from_str(&mcontent) {
                Ok(style) => {
                    let name = mfile.file_stem().unwrap_or_default();
                    let mut name = format!("{}", name.to_str().unwrap_or("error"));
                    if name.len() == 0 {
                        name = String::from("error");
                    }
                    settings.mermaid.theme_styles.insert(name, style);
                }
                Err(e) => {
                    log::error!("error to deserialize mermaid style {filename:#?}: {e}");
                }
            };
        }

        settings
    }
}
