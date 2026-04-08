//! ASCII/Unicode rendering for graphs.

use std::fmt::Display;

use crate::core::asciibox::ab_graph::style::{EdgeChars, EdgeStyle, NodeStyle};
use petgraph::graph::{EdgeIndex, NodeIndex};
use ratatui::style::Color;

/// A character cell in the render grid.
#[derive(Debug, Clone)]
pub struct Cell {
    pub char: char,
    pub fg: Color,
    pub bg: Color,
}

impl Default for Cell {
    fn default() -> Self {
        Self {
            char: ' ',
            fg: Color::White,
            bg: Color::Reset,
        }
    }
}

/// A 2D grid of characters for rendering.
pub struct CharGrid {
    cells: Vec<Cell>,
    width: usize,
    height: usize,
}

impl CharGrid {
    /// Create a new empty grid.
    pub fn new(width: usize, height: usize) -> Self {
        Self {
            cells: vec![Cell::default(); width * height],
            width,
            height,
        }
    }

    /// Get grid dimensions.
    pub fn size(&self) -> (usize, usize) {
        (self.width, self.height)
    }

    /// Set a cell at (x, y).
    pub fn set(&mut self, x: usize, y: usize, cell: Cell) {
        if x < self.width && y < self.height {
            self.cells[y * self.width + x] = cell;
        }
    }

    /// Set a character at (x, y) with a color.
    pub fn set_char(&mut self, x: usize, y: usize, c: char, fg: Color) {
        self.set(
            x,
            y,
            Cell {
                char: c,
                fg,
                bg: Color::Reset,
            },
        );
    }

    /// Get a cell at (x, y).
    pub fn get(&self, x: usize, y: usize) -> Option<&Cell> {
        if x < self.width && y < self.height {
            Some(&self.cells[y * self.width + x])
        } else {
            None
        }
    }

    /// Get a mutable cell at (x, y).
    pub fn get_mut(&mut self, x: usize, y: usize) -> Option<&mut Cell> {
        if x < self.width && y < self.height {
            Some(&mut self.cells[y * self.width + x])
        } else {
            None
        }
    }

    /// Draw a horizontal line.
    pub fn draw_hline(&mut self, x: usize, y: usize, len: usize, c: char, fg: Color) {
        for i in 0..len {
            self.set_char(x + i, y, c, fg);
        }
    }

    /// Draw a vertical line.
    pub fn draw_vline(&mut self, x: usize, y: usize, len: usize, c: char, fg: Color) {
        for i in 0..len {
            self.set_char(x, y + i, c, fg);
        }
    }

    /// Draw text at position.
    pub fn draw_text(&mut self, x: usize, y: usize, text: &str, fg: Color) {
        for (i, c) in text.chars().enumerate() {
            self.set_char(x + i, y, c, fg);
        }
    }

    /// Iterate over all cells with coordinates.
    pub fn iter(&self) -> impl Iterator<Item = (usize, usize, &Cell)> {
        self.cells.iter().enumerate().map(move |(i, cell)| {
            let x = i % self.width;
            let y = i / self.width;
            (x, y, cell)
        })
    }

    /// Convert the grid to a string (without color information).
    /// Each row is terminated with a newline character.
    pub fn to_string(&self) -> String {
        let mut output = String::with_capacity((self.width + 1) * self.height);
        for y in 0..self.height {
            for x in 0..self.width {
                if let Some(cell) = self.get(x, y) {
                    output.push(cell.char);
                }
            }
            output.push('\n');
        }
        output
    }

    /// Print the grid to stdout (without color information).
    pub fn print(&self) {
        for y in 0..self.height {
            for x in 0..self.width {
                if let Some(cell) = self.get(x, y) {
                    print!("{}", cell.char);
                }
            }
            println!();
        }
    }
}

/// Rendered node with position and size information.
#[derive(Debug, Clone)]
pub struct RenderedNode<N> {
    pub index: NodeIndex,
    pub label: N,
    /// Top-left corner X in character grid.
    pub x: usize,
    /// Top-left corner Y in character grid.
    pub y: usize,
    /// Width in characters (including border).
    pub width: usize,
    /// Height in characters (including border).
    pub height: usize,
    /// Style for this node.
    pub style: NodeStyle,
}

impl<N> RenderedNode<N> {
    /// Get the center X position (for edge routing).
    pub fn center_x(&self) -> usize {
        self.x + self.width / 2
    }

    /// Get the bottom Y position (for edge routing).
    pub fn bottom_y(&self) -> usize {
        self.y + self.height - 1
    }

    /// Get the top Y position (for edge routing).
    pub fn top_y(&self) -> usize {
        self.y
    }
}

/// Rendered edge with routing information.
#[derive(Debug, Clone)]
pub struct RenderedEdge<E> {
    pub index: EdgeIndex,
    pub label: E,
    pub source: NodeIndex,
    pub target: NodeIndex,
    /// Path segments as (x, y) points.
    pub path: Vec<(usize, usize)>,
    /// Style for this edge.
    pub style: EdgeStyle,
    /// Horizontal offset for parallel edges (0 = center, negative = left, positive = right).
    pub parallel_offset: i32,
}

/// Scaling mode for handling large graphs.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ScalingMode {
    /// Use full labels.
    Full,
    /// Truncate long labels.
    Truncate(usize),
    /// Use numeric IDs instead of labels.
    NumericIds,
}

/// Graph renderer that converts physics positions to character grid.
pub struct GraphRenderer {
    /// Scaling factor from physics units to characters.
    pub scale_x: f64,
    pub scale_y: f64,
    /// Padding around the graph.
    pub padding: usize,
    /// Node height (in characters, including border).
    pub node_height: usize,
    /// Minimum spacing between nodes.
    pub min_spacing: usize,
    /// Edge characters.
    pub edge_chars: EdgeChars,
    /// Current scaling mode.
    pub scaling_mode: ScalingMode,
}

impl Default for GraphRenderer {
    fn default() -> Self {
        Self {
            scale_x: 0.15,
            scale_y: 0.08,
            padding: 2,
            node_height: 3,
            min_spacing: 3,
            edge_chars: EdgeChars::default(),
            scaling_mode: ScalingMode::Full,
        }
    }
}

impl GraphRenderer {
    /// Calculate the display label for a node.
    pub fn display_label<N: Display>(&self, index: NodeIndex, label: &N) -> String {
        match self.scaling_mode {
            ScalingMode::Full => label.to_string(),
            ScalingMode::Truncate(max_len) => {
                let s = label.to_string();
                if s.len() > max_len {
                    format!("{}…", &s[..max_len - 1])
                } else {
                    s
                }
            }
            ScalingMode::NumericIds => index.index().to_string(),
        }
    }

    /// Calculate node width based on label.
    pub fn node_width(&self, label: &str) -> usize {
        // Border + padding + text + padding + border
        label.len() + 4
    }

    /// Render a node box to the grid.
    pub fn render_node<N: Display>(&self, grid: &mut CharGrid, node: &RenderedNode<N>) {
        let chars = node.style.border.chars();
        let label = self.display_label(node.index, &node.label);
        let width = node.width;

        // Top border
        grid.set_char(node.x, node.y, chars.top_left, node.style.border_color);
        grid.draw_hline(
            node.x + 1,
            node.y,
            width - 2,
            chars.horizontal,
            node.style.border_color,
        );
        grid.set_char(
            node.x + width - 1,
            node.y,
            chars.top_right,
            node.style.border_color,
        );

        // Middle row with label
        let mid_y = node.y + 1;
        grid.set_char(node.x, mid_y, chars.vertical, node.style.border_color);
        grid.set_char(node.x + 1, mid_y, ' ', node.style.text_color);
        grid.draw_text(node.x + 2, mid_y, &label, node.style.text_color);
        grid.set_char(node.x + width - 2, mid_y, ' ', node.style.text_color);
        grid.set_char(
            node.x + width - 1,
            mid_y,
            chars.vertical,
            node.style.border_color,
        );

        // Bottom border
        let bot_y = node.y + 2;
        grid.set_char(node.x, bot_y, chars.bottom_left, node.style.border_color);
        grid.draw_hline(
            node.x + 1,
            bot_y,
            width - 2,
            chars.horizontal,
            node.style.border_color,
        );
        grid.set_char(
            node.x + width - 1,
            bot_y,
            chars.bottom_right,
            node.style.border_color,
        );
    }

    /// Render an edge to the grid.
    pub fn render_edge<E: Display, N>(
        &self,
        grid: &mut CharGrid,
        edge: &RenderedEdge<E>,
        nodes: &[RenderedNode<N>],
    ) {
        let chars = &self.edge_chars;
        let color = edge.style.line_color;
        let label = edge.label.to_string();
        let offset = edge.parallel_offset;

        // Find source and target nodes
        let source = nodes.iter().find(|n| n.index == edge.source);
        let target = nodes.iter().find(|n| n.index == edge.target);

        let (source, target) = match (source, target) {
            (Some(s), Some(t)) => (s, t),
            _ => return,
        };

        // Apply horizontal offset for parallel edges
        let apply_offset = |x: usize| -> usize {
            if offset >= 0 {
                x.saturating_add(offset as usize)
            } else {
                x.saturating_sub((-offset) as usize)
            }
        };

        // Calculate connection points - determine if we go down or up
        let source_bottom = source.bottom_y();
        let source_top = source.top_y();
        let target_bottom = target.bottom_y();
        let target_top = target.top_y();

        if source_bottom < target_top {
            // Source is above target - go down
            let start_x = apply_offset(source.center_x());
            let start_y = source_bottom + 1;
            let end_x = apply_offset(target.center_x());
            let end_y = target_top.saturating_sub(1);

            self.render_vertical_edge(
                grid,
                chars,
                color,
                start_x,
                start_y,
                end_x,
                end_y,
                true,
                &label,
                edge.style.text_color,
                offset,
            );
        } else if target_bottom < source_top {
            // Target is above source - go up (back edge)
            let start_x = apply_offset(source.center_x());
            let start_y = source_top.saturating_sub(1);
            let end_x = apply_offset(target.center_x());
            let end_y = target_bottom + 1;

            self.render_vertical_edge(
                grid,
                chars,
                color,
                start_x,
                start_y,
                end_x,
                end_y,
                false,
                &label,
                edge.style.text_color,
                offset,
            );
        } else {
            // Nodes overlap vertically - draw horizontally
            let (start_x, end_x) = if source.x + source.width < target.x {
                (source.x + source.width, target.x.saturating_sub(1))
            } else {
                (source.x.saturating_sub(1), target.x + target.width)
            };
            // Apply vertical offset for parallel horizontal edges (symmetric around center)
            let base_y = source.y + source.height / 2;
            let y = if offset >= 0 {
                base_y.saturating_add(offset as usize)
            } else {
                base_y.saturating_sub((-offset) as usize)
            };

            // Simple horizontal edge
            let (left, right) = if start_x < end_x {
                (start_x, end_x)
            } else {
                (end_x, start_x)
            };
            for x in left..=right {
                grid.set_char(x, y, chars.horizontal, color);
            }
            let arrow = if end_x > start_x {
                chars.arrow_right
            } else {
                chars.arrow_left
            };
            grid.set_char(end_x, y, arrow, color);

            // Edge label for horizontal - place based on offset (top edge above, bottom edge below)
            if !label.is_empty() {
                let edge_len = right.saturating_sub(left);
                let label_x = if edge_len > label.len() + 2 {
                    // Center the label within the edge span
                    left + (edge_len - label.len()) / 2
                } else {
                    // Edge too short - place label starting at left edge
                    left
                };
                // Place label above for negative/zero offset, below for positive offset
                let label_y = if offset <= 0 {
                    y.saturating_sub(1)
                } else {
                    y + 1
                };
                grid.draw_text(label_x, label_y, &label, edge.style.text_color);
            }
        }
    }

    /// Helper to render vertical or L-shaped edges
    #[allow(clippy::too_many_arguments)]
    fn render_vertical_edge(
        &self,
        grid: &mut CharGrid,
        chars: &EdgeChars,
        color: Color,
        start_x: usize,
        start_y: usize,
        end_x: usize,
        end_y: usize,
        going_down: bool,
        label: &str,
        label_color: Color,
        parallel_offset: i32,
    ) {
        if start_x == end_x {
            // Straight vertical line
            let (y_start, y_end) = if start_y < end_y {
                (start_y, end_y)
            } else {
                (end_y, start_y)
            };
            for y in y_start..y_end {
                grid.set_char(start_x, y, chars.vertical, color);
            }
            let arrow = if going_down {
                chars.arrow_down
            } else {
                chars.arrow_up
            };
            grid.set_char(end_x, end_y, arrow, color);

            // Label placement based on offset: left for negative, right for positive/zero
            if !label.is_empty() {
                let label_y = (y_start + y_end) / 2;
                if parallel_offset < 0 {
                    // Place to the left of vertical edge
                    let label_x = start_x.saturating_sub(label.len() + 1);
                    grid.draw_text(label_x, label_y, label, label_color);
                } else {
                    // Place to the right of vertical edge
                    grid.draw_text(start_x + 1, label_y, label, label_color);
                }
            }
        } else {
            // L-shaped routing
            let mid_y = (start_y + end_y) / 2;
            let (y_min, y_max) = if start_y < end_y {
                (start_y, end_y)
            } else {
                (end_y, start_y)
            };
            let mid_y = mid_y.max(y_min).min(y_max);

            // Vertical from source to mid
            let (v1_start, v1_end) = if start_y < mid_y {
                (start_y, mid_y)
            } else {
                (mid_y, start_y)
            };
            for y in v1_start..v1_end {
                grid.set_char(start_x, y, chars.vertical, color);
            }

            // Corner at source column
            let corner1 = if end_x > start_x {
                if going_down {
                    chars.corner_up_right
                } else {
                    chars.corner_down_right
                }
            } else if going_down {
                chars.corner_up_left
            } else {
                chars.corner_down_left
            };
            grid.set_char(start_x, mid_y, corner1, color);

            // Horizontal line
            let (left_x, right_x) = if start_x < end_x {
                (start_x + 1, end_x)
            } else {
                (end_x + 1, start_x)
            };
            for x in left_x..right_x {
                grid.set_char(x, mid_y, chars.horizontal, color);
            }

            // Corner at target column
            let corner2 = if end_x > start_x {
                if going_down {
                    chars.corner_down_left
                } else {
                    chars.corner_up_left
                }
            } else if going_down {
                chars.corner_down_right
            } else {
                chars.corner_up_right
            };
            grid.set_char(end_x, mid_y, corner2, color);

            // Vertical from mid to target
            let (v2_start, v2_end) = if mid_y < end_y {
                (mid_y + 1, end_y)
            } else {
                (end_y + 1, mid_y)
            };
            for y in v2_start..v2_end {
                grid.set_char(end_x, y, chars.vertical, color);
            }

            let arrow = if going_down {
                chars.arrow_down
            } else {
                chars.arrow_up
            };
            grid.set_char(end_x, end_y, arrow, color);

            // Label placement strategy based on parallel_offset:
            // - Negative offset: place labels on the left side
            // - Positive/zero offset: place labels on the right side
            if !label.is_empty() {
                let v1_len = v1_end.saturating_sub(v1_start);
                let h_len = right_x.saturating_sub(left_x);

                if v1_len >= 2 {
                    // Place along first vertical segment
                    let label_y = (v1_start + v1_end) / 2;
                    if parallel_offset < 0 {
                        let label_x = start_x.saturating_sub(label.len() + 1);
                        grid.draw_text(label_x, label_y, label, label_color);
                    } else {
                        grid.draw_text(start_x + 1, label_y, label, label_color);
                    }
                } else if h_len > label.len() + 2 {
                    // Place along horizontal segment
                    let label_x = (left_x + right_x) / 2;
                    let label_x = label_x.saturating_sub(label.len() / 2);
                    // Above for negative offset, below for positive
                    let label_y = if parallel_offset < 0 {
                        mid_y.saturating_sub(1)
                    } else {
                        mid_y + 1
                    };
                    grid.draw_text(label_x, label_y, label, label_color);
                } else {
                    // Place along second vertical segment
                    let label_y = (v2_start + v2_end) / 2;
                    if parallel_offset < 0 {
                        let label_x = end_x.saturating_sub(label.len() + 1);
                        grid.draw_text(label_x, label_y, label, label_color);
                    } else {
                        grid.draw_text(end_x + 1, label_y, label, label_color);
                    }
                }
            }
        }
    }
}
