//! Extension trait for petgraph graphs.
//!
//! This module provides the [`AsciiGraphExt`] trait which adds a `.to_ascii()`
//! method to petgraph graph types, enabling easy ASCII visualization.

use std::collections::HashMap;
use std::fmt::Display;
use std::hash::Hash;

use petgraph::graph::DiGraph;
use petgraph::graphmap::DiGraphMap;
use petgraph::stable_graph::StableGraph;
use petgraph::visit::{EdgeRef, IntoEdgeReferences, IntoNodeReferences, NodeIndexable, NodeRef};

use crate::core::asciibox::ab_graph::graph::RenderedGraph;

/// Generic graph conversion helper that converts any graph type to DiGraph.
///
/// This function eliminates duplication in the `to_ascii` implementations by providing
/// a single conversion path for all graph types that support the necessary traits.
fn convert_to_digraph<'a, G, N, E, NId>(graph: G) -> DiGraph<N, E>
where
    G: IntoNodeReferences + IntoEdgeReferences + NodeIndexable,
    G::NodeWeight: Clone,
    G::EdgeWeight: Clone,
    N: From<G::NodeWeight>,
    E: From<G::EdgeWeight>,
    NId: Hash + Eq + From<G::NodeId>,
    G::NodeId: Copy,
{
    let mut digraph: DiGraph<N, E> = DiGraph::new();
    let mut node_map: HashMap<NId, petgraph::graph::NodeIndex> = HashMap::new();

    // Add all nodes
    for node_ref in graph.node_references() {
        let weight = N::from(node_ref.weight().clone());
        let new_idx = digraph.add_node(weight);
        node_map.insert(NId::from(node_ref.id()), new_idx);
    }

    // Add all edges
    for edge in graph.edge_references() {
        if let (Some(&src), Some(&tgt)) = (
            node_map.get(&NId::from(edge.source())),
            node_map.get(&NId::from(edge.target())),
        ) {
            digraph.add_edge(src, tgt, E::from(edge.weight().clone()));
        }
    }

    digraph
}

/// Extension trait that adds ASCII rendering capabilities to petgraph graphs.
///
/// Import this trait to use the `.to_ascii()` method on compatible petgraph graphs.
///
/// # Supported Graph Types
///
/// - `DiGraph<N, E>` - Directed graph
/// - `StableGraph<N, E>` - Stable directed graph (node removal doesn't invalidate indices)
/// - `DiGraphMap<N, E>` - Directed graph with hashable node type
///
/// # Example
///
/// ```rust
/// use petgraph::graph::DiGraph;
/// use ascii_petgraph::AsciiGraphExt;
///
/// let mut graph = DiGraph::new();
/// let a = graph.add_node("Start");
/// let b = graph.add_node("End");
/// graph.add_edge(a, b, "go");
///
/// let mut rendered = graph.to_ascii();
/// rendered.run_simulation();
/// ```
pub trait AsciiGraphExt<N, E> {
    /// Convert this graph to a renderable ASCII representation.
    ///
    /// This creates a [`RenderedGraph`] from the graph, which can then be
    /// configured, simulated, and rendered to a terminal.
    fn to_ascii(self) -> RenderedGraph<N, E>;
}

// Implementation for DiGraph (no conversion needed)
impl<N, E> AsciiGraphExt<N, E> for DiGraph<N, E>
where
    N: Display + Clone,
    E: Display + Clone,
{
    fn to_ascii(self) -> RenderedGraph<N, E> {
        RenderedGraph::from_graph(self)
    }
}

// Implementation for reference to DiGraph (clones the graph)
impl<N, E> AsciiGraphExt<N, E> for &DiGraph<N, E>
where
    N: Display + Clone,
    E: Display + Clone,
{
    fn to_ascii(self) -> RenderedGraph<N, E> {
        RenderedGraph::from_graph(self.clone())
    }
}

// Implementation for StableGraph
impl<N, E> AsciiGraphExt<N, E> for StableGraph<N, E>
where
    N: Display + Clone,
    E: Display + Clone,
{
    fn to_ascii(self) -> RenderedGraph<N, E> {
        let digraph = convert_to_digraph::<_, N, E, petgraph::stable_graph::NodeIndex>(&self);
        RenderedGraph::from_graph(digraph)
    }
}

// Implementation for reference to StableGraph
impl<N, E> AsciiGraphExt<N, E> for &StableGraph<N, E>
where
    N: Display + Clone,
    E: Display + Clone,
{
    fn to_ascii(self) -> RenderedGraph<N, E> {
        let digraph = convert_to_digraph::<_, N, E, petgraph::stable_graph::NodeIndex>(self);
        RenderedGraph::from_graph(digraph)
    }
}

// Implementation for DiGraphMap
impl<N, E> AsciiGraphExt<N, E> for DiGraphMap<N, E>
where
    N: Display + Clone + Copy + Ord + std::hash::Hash,
    E: Display + Clone,
{
    fn to_ascii(self) -> RenderedGraph<N, E> {
        let digraph = convert_to_digraph::<_, N, E, N>(&self);
        RenderedGraph::from_graph(digraph)
    }
}

// Implementation for reference to DiGraphMap
impl<N, E> AsciiGraphExt<N, E> for &DiGraphMap<N, E>
where
    N: Display + Clone + Copy + Ord + std::hash::Hash,
    E: Display + Clone,
{
    fn to_ascii(self) -> RenderedGraph<N, E> {
        let digraph = convert_to_digraph::<_, N, E, N>(self);
        RenderedGraph::from_graph(digraph)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_digraph_owned() {
        let mut graph: DiGraph<&str, &str> = DiGraph::new();
        let a = graph.add_node("A");
        let b = graph.add_node("B");
        graph.add_edge(a, b, "edge");

        let mut rendered = graph.to_ascii();
        rendered.run_simulation();

        assert!(rendered.is_stable());
    }

    #[test]
    fn test_digraph_ref() {
        let mut graph: DiGraph<&str, &str> = DiGraph::new();
        let a = graph.add_node("A");
        let b = graph.add_node("B");
        graph.add_edge(a, b, "edge");

        let mut rendered = (&graph).to_ascii();
        rendered.run_simulation();

        assert!(rendered.is_stable());
        // Original graph still usable
        assert_eq!(graph.node_count(), 2);
    }

    #[test]
    fn test_stable_graph() {
        let mut graph: StableGraph<&str, &str> = StableGraph::new();
        let a = graph.add_node("A");
        let b = graph.add_node("B");
        let c = graph.add_node("C");
        graph.add_edge(a, b, "1");
        graph.add_edge(b, c, "2");

        // Remove middle node - StableGraph handles this
        graph.remove_node(b);

        let mut rendered = graph.to_ascii();
        rendered.run_simulation();

        assert!(rendered.is_stable());
    }

    #[test]
    fn test_digraph_map() {
        let mut graph: DiGraphMap<&str, &str> = DiGraphMap::new();
        graph.add_edge("A", "B", "edge1");
        graph.add_edge("B", "C", "edge2");

        let mut rendered = graph.to_ascii();
        rendered.run_simulation();

        assert!(rendered.is_stable());
    }
}
