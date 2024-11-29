use std::{
    fs::{self, OpenOptions},
    io::{Read, Write},
    path::PathBuf,
};

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
#[serde(deny_unknown_fields)]
pub struct Config {
    pub theme_style: String,
    pub use_custom_font: bool,
    pub custom_font: String,
    pub flowchart: Flowchart,
    pub table: Table,
}

impl Default for Config {
    fn default() -> Self {
        Config {
            theme_style: "system".to_string(), // system-light-dark
            use_custom_font: false,
            custom_font: "nolxgw".to_string(),
            flowchart: Flowchart { expand_mode: false },
            table: Table {
                cell_max_width: 99,
                line_max_width: 299,
            },
        }
    }
}

impl Config {
    fn get_filename() -> PathBuf {
        let home = homedir::my_home().unwrap().unwrap();
        let filename = home.join(".config").join("asciibox").join("asciibox.toml");
        return filename;
    }

    pub fn new() -> Config {
        let filename = Config::get_filename();
        if !filename.exists() {
            return Config::default();
        }

        let mut file: std::fs::File = OpenOptions::new().read(true).open(filename).unwrap();
        let mut contents = String::new();
        file.read_to_string(&mut contents).unwrap();
        let config: Config = toml::from_str(&contents).unwrap();
        config
    }

    pub fn save(&self) {
        let filename = Config::get_filename();
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
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_save() {
        let config = Config {
            theme_style: "haha".to_string(),
            use_custom_font: false,
            custom_font: "hel".to_string(),
            flowchart: Flowchart { expand_mode: false },
            table: Table {
                cell_max_width: 33,
                line_max_width: 177,
            },
        };

        let toml = toml::to_string(&config).unwrap();
        println!("{toml}");

        let mut file: std::fs::File = OpenOptions::new()
            .write(true)
            .create(true)
            .open("config.toml")
            .unwrap();
        file.write_all(toml.as_bytes()).unwrap();
    }

    #[test]
    fn test_load() {
        let mut file: std::fs::File = OpenOptions::new().read(true).open("config.toml").unwrap();
        let mut contents = String::new();
        file.read_to_string(&mut contents).unwrap();
        let config: Config = toml::from_str(&contents).unwrap();
        println!("{:?}", config);
    }
}
