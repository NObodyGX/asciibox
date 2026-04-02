use std::fmt;

use crate::{core::settings::MermaidStyle, utils};

#[allow(dead_code)]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum MermaidTheme {
    Default,
    Neutral,
    Dark,
    Forest,
    Base,
    Custom,
}

impl MermaidTheme {
    pub fn as_str(&self) -> &'static str {
        match *self {
            MermaidTheme::Default => "default",
            MermaidTheme::Neutral => "neutral",
            MermaidTheme::Dark => "dark",
            MermaidTheme::Forest => "forest",
            MermaidTheme::Base => "base",
            MermaidTheme::Custom => "custom",
        }
    }

    pub fn mermaid_theme(&self) -> &'static str {
        match *self {
            MermaidTheme::Default => "default",
            MermaidTheme::Neutral => "neutral",
            MermaidTheme::Dark => "dark",
            MermaidTheme::Forest => "forest",
            _ => "base",
        }
    }

    pub fn from(s: &str) -> MermaidTheme {
        match s {
            "default" => MermaidTheme::Default,
            "neutral" => MermaidTheme::Neutral,
            "dark" => MermaidTheme::Dark,
            "forest" => MermaidTheme::Forest,
            "base" => MermaidTheme::Base,
            _ => MermaidTheme::Custom,
        }
    }

    pub fn is_custom(&self) -> bool {
        match *self {
            MermaidTheme::Default => false,
            MermaidTheme::Neutral => false,
            MermaidTheme::Dark => false,
            MermaidTheme::Forest => false,
            MermaidTheme::Base => false,
            _ => true,
        }
    }
}

impl fmt::Display for MermaidTheme {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

// 参考[theme](https://mermaid.nodejs.cn/config/theming.html#theme-variables)

// #[derive(Debug, Clone, PartialEq, Eq, Hash)]
// pub enum MermaidThemeVar {
//     /// 是否是黑夜模式，影响所有派生的颜色
//     DarkMode,
//     /// 背景, 默认 #f4f4f4
//     Background,
//     /// 字体，默认 mono, verdana, arial
//     FontFamily,
//     /// 字体大小，单位像素
//     FontSize,
//     /// 节点背景色，默认 #fff4dd
//     PrimaryColor,
//     /// 节点字体颜色，根据 #ddd 或者 #333 来计算(黑夜模式)
//     PrimaryTextColor,
//     /// 第三补充色
//     TertiaryColor,
//     /// 节点边框颜色
//     PrimaryBorderColor,
//     /// 连线颜色
//     LineColor,
//     /// 第二补充色
//     SecondaryColor,
//     /// 注意的背景色
//     NoteBkgColor,
//     /// 注意的边框色
//     NoteBorderColor,
//     /// 错误的背景色
//     ErrBkgColor,
//     /// 错误的边框色
//     ErrTextColor,
// }

#[derive(Debug, Clone)]
pub struct MermaidThemeConfig {
    pub dark_mode: bool,
    pub background: String,
    pub font_family: String,
    pub font_size: i32,
    pub primary_color: String,
    pub primary_text_color: String,
    pub primary_border_color: String,
    pub line_color: String,
    pub secondary_color: String,
    pub tertiary_color: String,
}

impl Default for MermaidThemeConfig {
    fn default() -> Self {
        Self {
            dark_mode: false,
            background: "#f4f4f4".to_string(),
            font_family: "'Maple Mono NF CN', verdana, arial".to_string(),
            font_size: 16,
            primary_color: "#fff4dd".to_string(),
            primary_text_color: "#333".to_string(),
            primary_border_color: "#ddd".to_string(),
            line_color: "#333".to_string(),
            secondary_color: "#eee".to_string(),
            tertiary_color: "#ccc".to_string(),
        }
    }
}

impl MermaidThemeConfig {
    pub fn from_mermaid_style(style: &MermaidStyle) -> Self {
        let config = MermaidThemeConfigBuilder::new()
            .dark_mode(style.dark_mode)
            .background(&style.background)
            .font_family(&style.font_family)
            .font_size(style.font_size)
            .primary_color(&style.primary_color)
            .primary_border_color(&style.primary_border_color)
            .primary_text_color(&style.primary_text_color)
            .line_color(&style.line_color)
            .secondary_color(&style.secondary_color)
            .tertiary_color(&style.tertiary_color)
            .build();

        config
    }

    pub fn to_js_string(&self) -> String {
        format!(
            "{{\
                darkMode: {},\
                background: '{}',\
                fontFamily: '{}',\
                fontSize: {},\
                primaryColor: '{}',\
                primaryTextColor: '{}',\
                primaryBorderColor: '{}',\
                lineColor: '{}',\
                secondaryColor: '{}',\
                tertiaryColor: '{}'\
            }}",
            self.dark_mode,
            self.background,
            self.font_family,
            self.font_size,
            self.primary_color,
            self.primary_text_color,
            self.primary_border_color,
            self.line_color,
            self.secondary_color,
            self.tertiary_color
        )
    }
}

#[derive(Debug, Clone)]
pub struct MermaidThemeConfigBuilder {
    dark_mode: Option<bool>,
    background: Option<String>,
    font_family: Option<String>,
    font_size: Option<i32>,
    primary_color: Option<String>,
    primary_text_color: Option<String>,
    primary_border_color: Option<String>,
    line_color: Option<String>,
    secondary_color: Option<String>,
    tertiary_color: Option<String>,
}

impl MermaidThemeConfigBuilder {
    fn new() -> Self {
        Self {
            dark_mode: None,
            background: None,
            font_family: None,
            font_size: None,
            primary_color: None,
            primary_text_color: None,
            primary_border_color: None,
            line_color: None,
            secondary_color: None,
            tertiary_color: None,
        }
    }

    fn dark_mode(mut self, dark_mode: bool) -> Self {
        self.dark_mode = Some(dark_mode);
        self
    }

    fn background<T: Into<String>>(mut self, background: T) -> Self {
        let background = background.into();
        if utils::check_is_color(&background) {
            self.background = Some(background);
        }
        self
    }

    fn font_family<T: Into<String>>(mut self, font_family: T) -> Self {
        self.font_family = Some(font_family.into());
        self
    }

    fn font_size(mut self, font_size: i32) -> Self {
        self.font_size = Some(font_size);
        self
    }

    fn primary_color<T: Into<String>>(mut self, primary_color: T) -> Self {
        let primary_color = primary_color.into();
        if utils::check_is_color(&primary_color) {
            self.primary_color = Some(primary_color);
        }
        self
    }

    fn primary_text_color<T: Into<String>>(mut self, primary_text_color: T) -> Self {
        let primary_text_color = primary_text_color.into();
        if utils::check_is_color(&primary_text_color) {
            self.primary_text_color = Some(primary_text_color);
        }
        self
    }

    fn primary_border_color<T: Into<String>>(mut self, primary_border_color: T) -> Self {
        let primary_border_color = primary_border_color.into();
        if utils::check_is_color(&primary_border_color) {
            self.primary_border_color = Some(primary_border_color);
        }
        self
    }

    fn line_color<T: Into<String>>(mut self, line_color: T) -> Self {
        let line_color = line_color.into();
        if utils::check_is_color(&line_color) {
            self.line_color = Some(line_color);
        }
        self
    }

    fn secondary_color<T: Into<String>>(mut self, secondary_color: T) -> Self {
        let secondary_color = secondary_color.into();
        if utils::check_is_color(&secondary_color) {
            self.secondary_color = Some(secondary_color);
        }
        self
    }

    fn tertiary_color<T: Into<String>>(mut self, tertiary_color: T) -> Self {
        let tertiary_color = tertiary_color.into();
        if utils::check_is_color(&tertiary_color) {
            self.tertiary_color = Some(tertiary_color);
        }
        self
    }

    fn build(self) -> MermaidThemeConfig {
        MermaidThemeConfig {
            dark_mode: self.dark_mode.unwrap_or(false),
            background: self.background.unwrap_or_else(|| "#f4f4f4".to_string()),
            font_family: self
                .font_family
                .unwrap_or_else(|| "mono, verdana, arial".to_string()),
            font_size: self.font_size.unwrap_or(16),
            primary_color: self.primary_color.unwrap_or_else(|| "#fff4dd".to_string()),
            primary_text_color: self
                .primary_text_color
                .unwrap_or_else(|| "#333".to_string()),
            primary_border_color: self
                .primary_border_color
                .unwrap_or_else(|| "#ddd".to_string()),
            line_color: self.line_color.unwrap_or_else(|| "#333".to_string()),
            secondary_color: self.secondary_color.unwrap_or_else(|| "#eee".to_string()),
            tertiary_color: self.tertiary_color.unwrap_or_else(|| "#ccc".to_string()),
        }
    }
}
