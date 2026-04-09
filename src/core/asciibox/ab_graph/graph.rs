//! Core `RenderedGraph` type that ties together physics, rendering, and styling.

use std::fmt::Display;

use petgraph::graph::{DiGraph, EdgeIndex, NodeIndex};
use petgraph::visit::EdgeRef;
use ratatui::buffer::Buffer;
use ratatui::layout::Rect;
use ratatui::style::Color;
use ratatui::widgets::Widget;

use super::physics::{PhysicsConfig, PhysicsEngine};
use super::render::{CharGrid, GraphRenderer, RenderedEdge, RenderedNode};
use super::style::{BoxBorder, EdgeStyle, NodeStyle};

/// A rendered graph ready for display in a TUI.
///
/// The graph structure is immutable after creation, but colors and styles
/// can be modified at any time.
pub struct RenderedGraph<N, E> {
    /// The underlying petgraph.
    graph: DiGraph<N, E>,
    /// Physics engine for layout.
    physics: PhysicsEngine,
    /// Renderer configuration.
    renderer: GraphRenderer,
    /// Per-node styles (indexed by node index).
    node_styles: Vec<NodeStyle>,
    /// Per-edge styles (indexed by edge index).
    edge_styles: Vec<EdgeStyle>,
    /// Default node style.
    default_node_style: NodeStyle,
    /// Default edge style.
    default_edge_style: EdgeStyle,
    /// Cached rendered nodes (updated after layout).
    rendered_nodes: Vec<RenderedNode<()>>,
    /// Cached rendered edges.
    rendered_edges: Vec<RenderedEdge<()>>,
    /// Whether layout is dirty and needs recalculation.
    layout_dirty: bool,
}

impl<N: Display + Clone, E: Display + Clone> RenderedGraph<N, E> {
    /// Create a new rendered graph from a petgraph DiGraph.
    pub fn from_graph(graph: DiGraph<N, E>) -> Self {
        let node_count = graph.node_count();
        let edge_count = graph.edge_count();

        let physics = PhysicsEngine::new(&graph, PhysicsConfig::default());
        let node_styles = vec![NodeStyle::default(); node_count];
        let edge_styles = vec![EdgeStyle::default(); edge_count];

        Self {
            graph,
            physics,
            renderer: GraphRenderer::default(),
            node_styles,
            edge_styles,
            default_node_style: NodeStyle::default(),
            default_edge_style: EdgeStyle::default(),
            rendered_nodes: Vec::new(),
            rendered_edges: Vec::new(),
            layout_dirty: true,
        }
    }

    /// Create a builder for more configuration options.
    pub fn builder() -> RenderedGraphBuilder<N, E> {
        RenderedGraphBuilder::new()
    }

    /// Get a reference to the underlying graph.
    pub fn graph(&self) -> &DiGraph<N, E> {
        &self.graph
    }

    /// Get the physics configuration.
    pub fn physics_config(&self) -> &PhysicsConfig {
        &self.physics.config
    }

    /// Set the physics configuration.
    pub fn set_physics_config(&mut self, config: PhysicsConfig) {
        self.physics.config = config;
    }

    /// Advance the physics simulation by one step.
    pub fn tick(&mut self) {
        self.physics.tick(&self.graph);
        self.layout_dirty = true;
    }

    /// Check if the simulation has converged.
    pub fn is_stable(&self) -> bool {
        self.physics.is_stable()
    }

    /// Run the physics simulation until stable.
    pub fn run_simulation(&mut self) {
        self.physics.run(&self.graph);
        self.physics.normalize_positions();
        self.layout_dirty = true;
    }

    /// Get the number of iterations run so far.
    pub fn iterations(&self) -> usize {
        self.physics.iterations()
    }

    // === Color API ===

    /// Helper to mutate a node's style.
    fn mutate_node_style<F>(&mut self, node: NodeIndex, f: F)
    where
        F: FnOnce(&mut NodeStyle),
    {
        if let Some(style) = self.node_styles.get_mut(node.index()) {
            f(style);
        }
    }

    /// Helper to mutate an edge's style.
    fn mutate_edge_style<F>(&mut self, edge: EdgeIndex, f: F)
    where
        F: FnOnce(&mut EdgeStyle),
    {
        if let Some(style) = self.edge_styles.get_mut(edge.index()) {
            f(style);
        }
    }

    /// Helper to mutate all node styles.
    fn mutate_all_node_styles<F>(&mut self, f: F)
    where
        F: Fn(&mut NodeStyle),
    {
        for style in &mut self.node_styles {
            f(style);
        }
    }

    /// Helper to mutate all edge styles.
    fn mutate_all_edge_styles<F>(&mut self, f: F)
    where
        F: Fn(&mut EdgeStyle),
    {
        for style in &mut self.edge_styles {
            f(style);
        }
    }

    /// Set the border color of a node.
    pub fn set_node_border_color(&mut self, node: NodeIndex, color: Color) {
        self.mutate_node_style(node, |style| style.border_color = color);
    }

    /// Set the text color of a node.
    pub fn set_node_text_color(&mut self, node: NodeIndex, color: Color) {
        self.mutate_node_style(node, |style| style.text_color = color);
    }

    /// Set both border and text color of a node.
    pub fn set_node_colors(&mut self, node: NodeIndex, border: Color, text: Color) {
        self.mutate_node_style(node, |style| {
            style.border_color = border;
            style.text_color = text;
        });
    }

    /// Set the line color of an edge.
    pub fn set_edge_color(&mut self, edge: EdgeIndex, color: Color) {
        self.mutate_edge_style(edge, |style| style.line_color = color);
    }

    /// Set the text color of an edge label.
    pub fn set_edge_text_color(&mut self, edge: EdgeIndex, color: Color) {
        self.mutate_edge_style(edge, |style| style.text_color = color);
    }

    /// Set both line and text color of an edge.
    pub fn set_edge_colors(&mut self, edge: EdgeIndex, line: Color, text: Color) {
        self.mutate_edge_style(edge, |style| {
            style.line_color = line;
            style.text_color = text;
        });
    }

    /// Set border color for all nodes.
    pub fn set_all_node_border_colors(&mut self, color: Color) {
        self.mutate_all_node_styles(|style| style.border_color = color);
    }

    /// Set text color for all nodes.
    pub fn set_all_node_text_colors(&mut self, color: Color) {
        self.mutate_all_node_styles(|style| style.text_color = color);
    }

    /// Set line color for all edges.
    pub fn set_all_edge_colors(&mut self, color: Color) {
        self.mutate_all_edge_styles(|style| style.line_color = color);
    }

    /// Reset all node styles to default.
    pub fn reset_node_styles(&mut self) {
        let default = self.default_node_style.clone();
        self.mutate_all_node_styles(|style| *style = default.clone());
    }

    /// Reset all edge styles to default.
    pub fn reset_edge_styles(&mut self) {
        let default = self.default_edge_style.clone();
        self.mutate_all_edge_styles(|style| *style = default.clone());
    }

    /// Set the default node style (used for new nodes and reset).
    pub fn set_default_node_style(&mut self, style: NodeStyle) {
        self.default_node_style = style;
    }

    /// Set the default edge style.
    pub fn set_default_edge_style(&mut self, style: EdgeStyle) {
        self.default_edge_style = style;
    }

    /// Set the box border style for all nodes.
    pub fn set_border_style(&mut self, border: BoxBorder) {
        self.mutate_all_node_styles(|style| style.border = border);
    }

    /// Set the scaling mode for handling large graphs.
    pub fn set_scaling_mode(&mut self, mode: super::render::ScalingMode) {
        self.renderer.scaling_mode = mode;
        self.layout_dirty = true;
    }

    /// Auto-detect and apply appropriate scaling mode based on terminal width.
    pub fn auto_scale(&mut self, max_width: usize) {
        use super::render::ScalingMode;

        // First try full labels
        self.renderer.scaling_mode = ScalingMode::Full;
        self.layout_dirty = true;
        self.update_layout();

        let current_width = self
            .rendered_nodes
            .iter()
            .map(|n| n.x + n.width)
            .max()
            .unwrap_or(0)
            + self.renderer.padding;

        if current_width <= max_width {
            return;
        }

        // Try truncating labels
        self.renderer.scaling_mode = ScalingMode::Truncate(8);
        self.layout_dirty = true;
        self.update_layout();

        let current_width = self
            .rendered_nodes
            .iter()
            .map(|n| n.x + n.width)
            .max()
            .unwrap_or(0)
            + self.renderer.padding;

        if current_width <= max_width {
            return;
        }

        // Fall back to numeric IDs
        self.renderer.scaling_mode = ScalingMode::NumericIds;
        self.layout_dirty = true;
    }

    // === Rendering ===

    /// Update the rendered layout from current physics positions.
    fn update_layout(&mut self) {
        if !self.layout_dirty {
            return;
        }

        self.rendered_nodes.clear();
        self.rendered_edges.clear();

        // Calculate node positions and sizes
        for (idx, node_idx) in self.graph.node_indices().enumerate() {
            let pos = self.physics.position(node_idx);
            let label = self.graph[node_idx].to_string();
            let display_label = self.renderer.display_label(node_idx, &label);
            let width = self.renderer.node_width(&display_label);

            let x = (pos.x * self.renderer.scale_x) as usize + self.renderer.padding;
            let y = (pos.y * self.renderer.scale_y) as usize + self.renderer.padding;

            self.rendered_nodes.push(RenderedNode {
                index: node_idx,
                label: (),
                x,
                y,
                width,
                height: self.renderer.node_height,
                style: self.node_styles.get(idx).cloned().unwrap_or_default(),
            });
        }

        // Calculate edge paths and parallel offsets
        // First, collect all edges and detect parallel pairs
        use std::collections::HashMap;
        let mut parallel_groups: HashMap<(usize, usize), Vec<EdgeIndex>> = HashMap::new();

        for edge in self.graph.edge_references() {
            let s = edge.source().index();
            let t = edge.target().index();
            // Use canonical ordering to group edges between same nodes
            let key = (s.min(t), s.max(t));
            parallel_groups.entry(key).or_default().push(edge.id());
        }

        // Assign offsets to parallel edges
        let mut edge_offsets: HashMap<EdgeIndex, i32> = HashMap::new();
        for edges in parallel_groups.values() {
            if edges.len() > 1 {
                // Multiple edges between same nodes - assign symmetric offsets
                // For 2 edges: -1, +1; for 3 edges: -2, 0, +2; etc.
                let count = edges.len() as i32;
                for (i, &edge_id) in edges.iter().enumerate() {
                    let offset = 2 * i as i32 - (count - 1);
                    edge_offsets.insert(edge_id, offset);
                }
            }
        }

        for edge in self.graph.edge_references() {
            let idx = edge.id().index();
            let offset = edge_offsets.get(&edge.id()).copied().unwrap_or(0);
            self.rendered_edges.push(RenderedEdge {
                index: edge.id(),
                label: (),
                source: edge.source(),
                target: edge.target(),
                path: Vec::new(), // Path calculated during rendering
                style: self.edge_styles.get(idx).cloned().unwrap_or_default(),
                parallel_offset: offset,
            });
        }

        self.layout_dirty = false;
    }

    /// Render the graph to a character grid.
    pub fn render_to_grid(&mut self) -> CharGrid {
        self.update_layout();

        // Calculate grid size with extra room for edge labels
        let max_label_len = self
            .graph
            .edge_weights()
            .map(|w| w.to_string().len())
            .max()
            .unwrap_or(0);

        let max_x = self
            .rendered_nodes
            .iter()
            .map(|n| n.x + n.width)
            .max()
            .unwrap_or(0)
            + self.renderer.padding
            + max_label_len
            + 2;
        let max_y = self
            .rendered_nodes
            .iter()
            .map(|n| n.y + n.height)
            .max()
            .unwrap_or(0)
            + self.renderer.padding
            + 2;

        let mut grid = CharGrid::new(max_x.max(1), max_y.max(1));

        // Render edges first (so nodes draw on top)
        for (idx, edge) in self.rendered_edges.iter().enumerate() {
            let edge_with_label = RenderedEdge {
                index: edge.index,
                label: self
                    .graph
                    .edge_weight(edge.index)
                    .map(|w| w.to_string())
                    .unwrap_or_default(),
                source: edge.source,
                target: edge.target,
                path: edge.path.clone(),
                style: self.edge_styles.get(idx).cloned().unwrap_or_default(),
                parallel_offset: edge.parallel_offset,
            };
            self.renderer
                .render_edge(&mut grid, &edge_with_label, &self.rendered_nodes);
        }

        // Render nodes
        for (idx, node) in self.rendered_nodes.iter().enumerate() {
            let label = self
                .renderer
                .display_label(node.index, &self.graph[node.index]);
            let node_with_label = RenderedNode {
                index: node.index,
                label,
                x: node.x,
                y: node.y,
                width: node.width,
                height: node.height,
                style: self.node_styles.get(idx).cloned().unwrap_or_default(),
            };
            self.renderer.render_node(&mut grid, &node_with_label);
        }

        grid
    }

    /// Create a widget for rendering with ratatui.
    pub fn widget(&mut self) -> GraphWidget<'_, N, E> {
        GraphWidget { graph: self }
    }
}

/// Builder for RenderedGraph with configuration options.
pub struct RenderedGraphBuilder<N, E> {
    graph: Option<DiGraph<N, E>>,
    physics_config: PhysicsConfig,
    border_style: BoxBorder,
    default_node_style: NodeStyle,
    default_edge_style: EdgeStyle,
}

impl<N, E> RenderedGraphBuilder<N, E> {
    pub fn new() -> Self {
        Self {
            graph: None,
            physics_config: PhysicsConfig::default(),
            border_style: BoxBorder::default(),
            default_node_style: NodeStyle::default(),
            default_edge_style: EdgeStyle::default(),
        }
    }

    pub fn graph(mut self, graph: DiGraph<N, E>) -> Self {
        self.graph = Some(graph);
        self
    }

    pub fn physics_config(mut self, config: PhysicsConfig) -> Self {
        self.physics_config = config;
        self
    }

    pub fn border_style(mut self, border: BoxBorder) -> Self {
        self.border_style = border;
        self.default_node_style.border = border;
        self
    }

    pub fn default_node_style(mut self, style: NodeStyle) -> Self {
        self.default_node_style = style;
        self
    }

    pub fn default_edge_style(mut self, style: EdgeStyle) -> Self {
        self.default_edge_style = style;
        self
    }

    pub fn gravity(mut self, g: f64) -> Self {
        self.physics_config.gravity = g;
        self
    }

    pub fn spring_constant(mut self, k: f64) -> Self {
        self.physics_config.spring_constant = k;
        self
    }

    pub fn repulsion_constant(mut self, r: f64) -> Self {
        self.physics_config.repulsion_constant = r;
        self
    }
}

impl<N: Display + Clone, E: Display + Clone> RenderedGraphBuilder<N, E> {
    pub fn build(self) -> RenderedGraph<N, E> {
        let graph = self.graph.expect("graph is required");
        let mut rendered = RenderedGraph::from_graph(graph);
        rendered.set_physics_config(self.physics_config);
        rendered.set_default_node_style(self.default_node_style.clone());
        rendered.set_default_edge_style(self.default_edge_style.clone());
        rendered.set_border_style(self.border_style);
        rendered
    }
}

impl<N, E> Default for RenderedGraphBuilder<N, E> {
    fn default() -> Self {
        Self::new()
    }
}

/// Ratatui widget for rendering the graph.
pub struct GraphWidget<'a, N, E> {
    graph: &'a mut RenderedGraph<N, E>,
}

impl<N: Display + Clone, E: Display + Clone> Widget for GraphWidget<'_, N, E> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let grid = self.graph.render_to_grid();
        let (grid_width, grid_height) = grid.size();

        for (gx, gy, cell) in grid.iter() {
            // Map grid coordinates to buffer coordinates
            let bx = area.x + gx as u16;
            let by = area.y + gy as u16;

            // Check bounds
            if bx >= area.x + area.width || by >= area.y + area.height {
                continue;
            }
            if gx >= grid_width || gy >= grid_height {
                continue;
            }

            let buf_cell = buf.cell_mut((bx, by));
            if let Some(buf_cell) = buf_cell {
                buf_cell.set_char(cell.char);
                buf_cell.set_fg(cell.fg);
                if cell.bg != Color::Reset {
                    buf_cell.set_bg(cell.bg);
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rendered_graph_creation() {
        let mut graph: DiGraph<&str, &str> = DiGraph::new();
        let a = graph.add_node("A");
        let b = graph.add_node("B");
        graph.add_edge(a, b, "edge");

        let rendered = RenderedGraph::from_graph(graph);
        assert_eq!(rendered.graph().node_count(), 2);
        assert_eq!(rendered.graph().edge_count(), 1);
    }

    #[test]
    fn test_simulation_runs() {
        let mut graph: DiGraph<&str, &str> = DiGraph::new();
        let a = graph.add_node("A");
        let b = graph.add_node("B");
        let c = graph.add_node("C");
        graph.add_edge(a, b, "1");
        graph.add_edge(b, c, "2");

        let mut rendered = RenderedGraph::from_graph(graph);
        rendered.run_simulation();

        assert!(rendered.is_stable());
        assert!(rendered.iterations() >= 10);
    }

    #[test]
    fn test_color_api() {
        let mut graph: DiGraph<&str, &str> = DiGraph::new();
        let a = graph.add_node("A");
        let b = graph.add_node("B");
        let e = graph.add_edge(a, b, "edge");

        let mut rendered = RenderedGraph::from_graph(graph);

        // Set node colors
        rendered.set_node_border_color(a, Color::Red);
        rendered.set_node_text_color(a, Color::Green);

        // Set edge colors
        rendered.set_edge_color(e, Color::Blue);
        rendered.set_edge_text_color(e, Color::Yellow);

        // Bulk operations
        rendered.set_all_node_border_colors(Color::White);
        rendered.set_all_edge_colors(Color::Gray);

        // Reset
        rendered.reset_node_styles();
        rendered.reset_edge_styles();
    }

    #[test]
    fn test_render_to_grid() {
        let mut graph: DiGraph<&str, &str> = DiGraph::new();
        let a = graph.add_node("Hello");
        let b = graph.add_node("World");
        graph.add_edge(a, b, "");

        let mut rendered = RenderedGraph::from_graph(graph);
        rendered.run_simulation();

        let grid = rendered.render_to_grid();
        let (width, height) = grid.size();

        assert!(width > 0);
        assert!(height > 0);

        // Check that some cells have content
        let mut has_content = false;
        for (_, _, cell) in grid.iter() {
            if cell.char != ' ' {
                has_content = true;
                break;
            }
        }
        assert!(has_content);
    }

    #[test]
    fn test_builder() {
        let mut graph: DiGraph<&str, &str> = DiGraph::new();
        graph.add_node("A");
        graph.add_node("B");

        let rendered = RenderedGraph::builder()
            .graph(graph)
            .border_style(BoxBorder::Rounded)
            .gravity(2.0)
            .spring_constant(0.2)
            .build();

        assert_eq!(rendered.physics_config().gravity, 2.0);
        assert_eq!(rendered.physics_config().spring_constant, 0.2);
    }

    #[test]
    fn test_scaling_mode() {
        let mut graph: DiGraph<&str, &str> = DiGraph::new();
        let a = graph.add_node("VeryLongNodeLabel");
        let b = graph.add_node("AnotherLongLabel");
        graph.add_edge(a, b, "");

        let mut rendered = RenderedGraph::from_graph(graph);
        rendered.run_simulation();

        // Test different scaling modes
        rendered.set_scaling_mode(super::super::render::ScalingMode::Full);
        let grid1 = rendered.render_to_grid();

        rendered.set_scaling_mode(super::super::render::ScalingMode::Truncate(5));
        rendered.render_to_grid(); // Trigger re-render with truncated mode

        rendered.set_scaling_mode(super::super::render::ScalingMode::NumericIds);
        let grid3 = rendered.render_to_grid();

        // NumericIds should produce smallest grid
        let (w1, _) = grid1.size();
        let (w3, _) = grid3.size();
        assert!(w3 <= w1);
    }
}
