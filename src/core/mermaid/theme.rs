use std::fmt;

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
            MermaidTheme::Custom => "base",
        }
    }

    pub fn from(s: &str) -> MermaidTheme {
        match s {
            "default" => MermaidTheme::Default,
            "neutral" => MermaidTheme::Neutral,
            "dark" => MermaidTheme::Dark,
            "forest" => MermaidTheme::Forest,
            "base" => MermaidTheme::Base,
            "custom" => MermaidTheme::Custom,
            _ => {
                log::warn!("mermaid theme unsupport theme: {s}");
                MermaidTheme::Default
            }
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
// pub enum MermaidThemeVariable {
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
