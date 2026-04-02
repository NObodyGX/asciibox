use std::collections::BTreeMap;

use crate::core::{AppSettings, settings::MermaidStyle};

#[derive(Debug, Clone)]
pub struct MermaidThemeManager {
    pub themes: BTreeMap<String, MermaidStyle>,
}

impl Default for MermaidThemeManager {
    fn default() -> Self {
        MermaidThemeManager {
            themes: BTreeMap::new(),
        }
    }
}

impl MermaidThemeManager {
    pub fn init(&mut self) {
        self.add_theme(
            "adwaita",
            MermaidStyle {
                dark_mode: false,
                background: "#f4f4f4".to_string(),
                font_family: "Maple Mono NF CN".to_string(),
                font_size: 14,
                primary_color: "#f4f4f4".to_string(),
                primary_border_color: "#000".to_string(),
                primary_text_color: "#000".to_string(),
                line_color: "#000".to_string(),
                secondary_color: "#00f33d".to_string(),
                tertiary_color: "#c30000".to_string(),
            },
        );

        let settings = AppSettings::get();
        for (name, value) in settings.mermaid.theme_styles.iter() {
            self.add_theme(name, value.clone());
        }
    }

    pub fn add_theme(&mut self, name: &str, style: MermaidStyle) {
        self.themes.insert(name.to_string(), style);
    }

    pub fn get_theme(&self, name: &str) -> Option<&MermaidStyle> {
        self.themes.get(name)
    }
}
