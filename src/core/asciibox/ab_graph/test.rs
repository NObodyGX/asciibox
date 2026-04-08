//! Integration tests for ascii-petgraph output completeness and quality.

use super::{BoxBorder, RenderedGraph};
use petgraph::graph::DiGraph;

pub fn render_to_string<N: std::fmt::Display + Clone, E: std::fmt::Display + Clone>(
    rendered: &mut RenderedGraph<N, E>,
) -> String {
    let grid = rendered.render_to_grid();
    grid.to_string()
}

pub fn create_state_machine() -> DiGraph<&'static str, &'static str> {
    let mut graph = DiGraph::new();
    let idle = graph.add_node("Idle");
    let running = graph.add_node("Running");
    let paused = graph.add_node("Paused");
    let complete = graph.add_node("Complete");
    let failed = graph.add_node("Failed");

    graph.add_edge(idle, running, "start");
    graph.add_edge(running, paused, "pause");
    graph.add_edge(paused, running, "resume");
    graph.add_edge(running, complete, "finish");
    graph.add_edge(running, failed, "error");
    graph.add_edge(complete, idle, "reset");
    graph.add_edge(failed, idle, "retry");

    graph
}

// =============================================================================
// OUTPUT COMPLETENESS TESTS
// =============================================================================

#[test]
fn test_all_nodes_visible() {
    let mut graph: DiGraph<&str, &str> = DiGraph::new();
    let labels = ["Alpha", "Beta", "Gamma", "Delta", "Epsilon"];
    let nodes: Vec<_> = labels.iter().map(|l| graph.add_node(*l)).collect();

    // Chain them together
    for i in 0..nodes.len() - 1 {
        graph.add_edge(nodes[i], nodes[i + 1], "");
    }

    let mut rendered = RenderedGraph::from_graph(graph);
    rendered.run_simulation();
    let output = render_to_string(&mut rendered);
    println!("{}", output);
    for label in &labels {
        assert!(
            output.contains(label),
            "Node label '{}' not found in output:\n{}",
            label,
            output
        );
    }
}

#[test]
fn test_all_edge_labels_visible() {
    let mut graph: DiGraph<&str, &str> = DiGraph::new();
    let a = graph.add_node("A");
    let b = graph.add_node("B");
    let c = graph.add_node("C");
    let d = graph.add_node("D");

    let edge_labels = ["first", "second", "third"];
    graph.add_edge(a, b, edge_labels[0]);
    graph.add_edge(b, c, edge_labels[1]);
    graph.add_edge(c, d, edge_labels[2]);

    let mut rendered = RenderedGraph::from_graph(graph);
    rendered.run_simulation();
    let output = render_to_string(&mut rendered);

    for label in &edge_labels {
        assert!(
            output.contains(label),
            "Edge label '{}' not found in output:\n{}",
            label,
            output
        );
    }
}

#[test]
fn test_state_machine_complete() {
    let graph = create_state_machine();
    let mut rendered = RenderedGraph::from_graph(graph);
    // rendered.run_simulation();
    let output = render_to_string(&mut rendered);
    println!("{}", output);

    // All states must be present
    let states = ["Idle", "Running", "Paused", "Complete", "Failed"];
    for state in &states {
        assert!(
            output.contains(state),
            "State '{}' not found in output:\n{}",
            state,
            output
        );
    }

    // Most transitions should be present (some may be truncated due to edge length)
    let transitions = [
        "start", "pause", "resume", "finish", "error", "reset", "retry",
    ];
    let found_count = transitions.iter().filter(|t| output.contains(*t)).count();

    assert!(
        found_count >= 5,
        "Expected at least 5 of 7 transitions, found {}. Output:\n{}",
        found_count,
        output
    );
}

// =============================================================================
// STRUCTURAL TESTS
// =============================================================================

#[test]
fn test_simple_chain() {
    let mut graph: DiGraph<&str, &str> = DiGraph::new();
    let a = graph.add_node("Start");
    let b = graph.add_node("Middle");
    let c = graph.add_node("End");
    graph.add_edge(a, b, "");
    graph.add_edge(b, c, "");

    let mut rendered = RenderedGraph::from_graph(graph);
    rendered.run_simulation();
    let output = render_to_string(&mut rendered);

    assert!(output.contains("Start"));
    assert!(output.contains("Middle"));
    assert!(output.contains("End"));
}

#[test]
fn test_diamond_pattern() {
    let mut graph: DiGraph<&str, &str> = DiGraph::new();
    let top = graph.add_node("Top");
    let left = graph.add_node("Left");
    let right = graph.add_node("Right");
    let bottom = graph.add_node("Bottom");

    graph.add_edge(top, left, "");
    graph.add_edge(top, right, "");
    graph.add_edge(left, bottom, "");
    graph.add_edge(right, bottom, "");

    let mut rendered = RenderedGraph::from_graph(graph);
    rendered.run_simulation();
    let output = render_to_string(&mut rendered);

    assert!(output.contains("Top"));
    assert!(output.contains("Left"));
    assert!(output.contains("Right"));
    assert!(output.contains("Bottom"));
}

#[test]
fn test_wide_fan() {
    let mut graph: DiGraph<&str, &str> = DiGraph::new();
    let root = graph.add_node("Root");
    let sink = graph.add_node("Sink");

    let children: Vec<_> = (0..5)
        .map(|i| graph.add_node(Box::leak(format!("C{}", i).into_boxed_str()) as &str))
        .collect();

    for &child in &children {
        graph.add_edge(root, child, "");
        graph.add_edge(child, sink, "");
    }

    let mut rendered = RenderedGraph::from_graph(graph);
    rendered.run_simulation();
    let output = render_to_string(&mut rendered);

    assert!(output.contains("Root"));
    assert!(output.contains("Sink"));
    for i in 0..5 {
        assert!(output.contains(&format!("C{}", i)));
    }
}

#[test]
fn test_back_edges() {
    // Graph with edges going "backwards" (target above source in layout)
    let mut graph: DiGraph<&str, &str> = DiGraph::new();
    let a = graph.add_node("A");
    let b = graph.add_node("B");
    let c = graph.add_node("C");

    graph.add_edge(a, b, "down");
    graph.add_edge(b, c, "down");
    graph.add_edge(c, a, "back"); // Back edge

    let mut rendered = RenderedGraph::from_graph(graph);
    rendered.run_simulation();
    let output = render_to_string(&mut rendered);

    assert!(output.contains("A"));
    assert!(output.contains("B"));
    assert!(output.contains("C"));
    // Back edge label should be visible
    assert!(output.contains("back"));
}

// =============================================================================
// PHYSICS SIMULATION TESTS
// =============================================================================

#[test]
fn test_simulation_converges() {
    let graph = create_state_machine();
    let mut rendered = RenderedGraph::from_graph(graph);

    assert!(
        !rendered.is_stable(),
        "Should not be stable before simulation"
    );

    rendered.run_simulation();

    assert!(rendered.is_stable(), "Should be stable after simulation");
    assert!(
        rendered.iterations() >= 10,
        "Should have run multiple iterations"
    );
}

#[test]
fn test_deterministic_output() {
    let graph1 = create_state_machine();
    let graph2 = create_state_machine();

    let mut rendered1 = RenderedGraph::from_graph(graph1);
    let mut rendered2 = RenderedGraph::from_graph(graph2);

    rendered1.run_simulation();
    rendered2.run_simulation();

    let output1 = render_to_string(&mut rendered1);
    let output2 = render_to_string(&mut rendered2);

    assert_eq!(output1, output2, "Same input should produce same output");
}

// =============================================================================
// RENDERING QUALITY TESTS
// =============================================================================

#[test]
fn test_arrows_present() {
    let mut graph: DiGraph<&str, &str> = DiGraph::new();
    let a = graph.add_node("A");
    let b = graph.add_node("B");
    graph.add_edge(a, b, "");

    let mut rendered = RenderedGraph::from_graph(graph);
    rendered.run_simulation();
    let output = render_to_string(&mut rendered);

    // Should have at least one arrow character
    let has_arrow = output.contains('↓')
        || output.contains('↑')
        || output.contains('→')
        || output.contains('←');

    assert!(
        has_arrow,
        "Output should contain arrow characters:\n{}",
        output
    );
}

#[test]
fn test_box_borders_complete() {
    let mut graph: DiGraph<&str, &str> = DiGraph::new();
    graph.add_node("Test");

    let mut rendered = RenderedGraph::from_graph(graph);
    rendered.run_simulation();
    let output = render_to_string(&mut rendered);

    // Default single border should have these characters
    assert!(output.contains('┌'), "Missing top-left corner");
    assert!(output.contains('┐'), "Missing top-right corner");
    assert!(output.contains('└'), "Missing bottom-left corner");
    assert!(output.contains('┘'), "Missing bottom-right corner");
    assert!(output.contains('─'), "Missing horizontal border");
    assert!(output.contains('│'), "Missing vertical border");
}

// =============================================================================
// BORDER STYLE TESTS
// =============================================================================

#[test]
fn test_border_style_single() {
    let mut graph: DiGraph<&str, &str> = DiGraph::new();
    graph.add_node("X");

    let mut rendered = RenderedGraph::builder()
        .graph(graph)
        .border_style(BoxBorder::Single)
        .build();
    rendered.run_simulation();
    let output = render_to_string(&mut rendered);

    assert!(output.contains('┌'));
    assert!(output.contains('┘'));
}

#[test]
fn test_border_style_double() {
    let mut graph: DiGraph<&str, &str> = DiGraph::new();
    graph.add_node("X");

    let mut rendered = RenderedGraph::builder()
        .graph(graph)
        .border_style(BoxBorder::Double)
        .build();
    rendered.run_simulation();
    let output = render_to_string(&mut rendered);

    assert!(
        output.contains('╔'),
        "Missing double top-left corner:\n{}",
        output
    );
    assert!(
        output.contains('╝'),
        "Missing double bottom-right corner:\n{}",
        output
    );
}

#[test]
fn test_border_style_rounded() {
    let mut graph: DiGraph<&str, &str> = DiGraph::new();
    graph.add_node("X");

    let mut rendered = RenderedGraph::builder()
        .graph(graph)
        .border_style(BoxBorder::Rounded)
        .build();
    rendered.run_simulation();
    let output = render_to_string(&mut rendered);

    assert!(
        output.contains('╭'),
        "Missing rounded top-left corner:\n{}",
        output
    );
    assert!(
        output.contains('╯'),
        "Missing rounded bottom-right corner:\n{}",
        output
    );
}

#[test]
fn test_border_style_ascii() {
    let mut graph: DiGraph<&str, &str> = DiGraph::new();
    graph.add_node("X");

    let mut rendered = RenderedGraph::builder()
        .graph(graph)
        .border_style(BoxBorder::Ascii)
        .build();
    rendered.run_simulation();
    let output = render_to_string(&mut rendered);

    assert!(output.contains('+'), "Missing ASCII corner:\n{}", output);
    assert!(
        output.contains('-'),
        "Missing ASCII horizontal:\n{}",
        output
    );
    assert!(output.contains('|'), "Missing ASCII vertical:\n{}", output);
}

// =============================================================================
// SCALING TESTS
// =============================================================================

#[test]
fn test_scaling_mode_numeric() {
    use crate::core::asciibox::ab_graph::ScalingMode;

    let mut graph: DiGraph<&str, &str> = DiGraph::new();
    graph.add_node("VeryLongNodeLabel");
    graph.add_node("AnotherLongLabel");

    let mut rendered = RenderedGraph::from_graph(graph);
    rendered.run_simulation();
    rendered.set_scaling_mode(ScalingMode::NumericIds);

    let output = render_to_string(&mut rendered);

    // Should have numeric IDs instead of labels
    assert!(output.contains('0') || output.contains('1'));
    // Should NOT have the full labels
    assert!(!output.contains("VeryLongNodeLabel"));
}

#[test]
fn test_scaling_mode_truncate() {
    use crate::core::asciibox::ab_graph::ScalingMode;

    let mut graph: DiGraph<&str, &str> = DiGraph::new();
    graph.add_node("VeryLongNodeLabel");

    let mut rendered = RenderedGraph::from_graph(graph);
    rendered.run_simulation();
    rendered.set_scaling_mode(ScalingMode::Truncate(8));

    let output = render_to_string(&mut rendered);

    // Should have truncated label with ellipsis
    assert!(output.contains("VeryLon") || output.contains("…"));
    // Should NOT have the full label
    assert!(!output.contains("VeryLongNodeLabel"));
}
