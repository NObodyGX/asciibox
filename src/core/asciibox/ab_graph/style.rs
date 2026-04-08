//! Style definitions for nodes and edges.

use ratatui::style::Color;

/// Box border style for nodes.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum BoxBorder {
    /// Single line borders: ┌─┐│└─┘
    #[default]
    Single,
    /// Double line borders: ╔═╗║╚═╝
    Double,
    /// Rounded corners: ╭─╮│╰─╯
    Rounded,
    /// ASCII only: +-+|+-+
    Ascii,
}

impl BoxBorder {
    /// Get the corner and edge characters for this border style.
    /// Returns (top_left, top_right, bottom_left, bottom_right, horizontal, vertical)
    pub fn chars(self) -> BorderChars {
        match self {
            BoxBorder::Single => BorderChars {
                top_left: '┌',
                top_right: '┐',
                bottom_left: '└',
                bottom_right: '┘',
                horizontal: '─',
                vertical: '│',
            },
            BoxBorder::Double => BorderChars {
                top_left: '╔',
                top_right: '╗',
                bottom_left: '╚',
                bottom_right: '╝',
                horizontal: '═',
                vertical: '║',
            },
            BoxBorder::Rounded => BorderChars {
                top_left: '╭',
                top_right: '╮',
                bottom_left: '╰',
                bottom_right: '╯',
                horizontal: '─',
                vertical: '│',
            },
            BoxBorder::Ascii => BorderChars {
                top_left: '+',
                top_right: '+',
                bottom_left: '+',
                bottom_right: '+',
                horizontal: '-',
                vertical: '|',
            },
        }
    }
}

/// Characters used to draw box borders.
#[derive(Debug, Clone, Copy)]
pub struct BorderChars {
    pub top_left: char,
    pub top_right: char,
    pub bottom_left: char,
    pub bottom_right: char,
    pub horizontal: char,
    pub vertical: char,
}

/// Style for a node.
#[derive(Debug, Clone)]
pub struct NodeStyle {
    /// Border style for the node box.
    pub border: BoxBorder,
    /// Color of the border.
    pub border_color: Color,
    /// Color of the text inside the node.
    pub text_color: Color,
}

impl Default for NodeStyle {
    fn default() -> Self {
        Self {
            border: BoxBorder::default(),
            border_color: Color::White,
            text_color: Color::White,
        }
    }
}

/// Style for an edge.
#[derive(Debug, Clone)]
pub struct EdgeStyle {
    /// Color of the edge line.
    pub line_color: Color,
    /// Color of the edge label text.
    pub text_color: Color,
}

impl Default for EdgeStyle {
    fn default() -> Self {
        Self {
            line_color: Color::White,
            text_color: Color::Gray,
        }
    }
}

/// Characters used for drawing edges.
#[derive(Debug, Clone, Copy)]
pub struct EdgeChars {
    pub vertical: char,
    pub horizontal: char,
    pub corner_down_right: char,
    pub corner_down_left: char,
    pub corner_up_right: char,
    pub corner_up_left: char,
    pub arrow_down: char,
    pub arrow_up: char,
    pub arrow_right: char,
    pub arrow_left: char,
}

impl Default for EdgeChars {
    fn default() -> Self {
        Self {
            vertical: '│',
            horizontal: '─',
            corner_down_right: '┌',
            corner_down_left: '┐',
            corner_up_right: '└',
            corner_up_left: '┘',
            arrow_down: '↓',
            arrow_up: '↑',
            arrow_right: '→',
            arrow_left: '←',
        }
    }
}
