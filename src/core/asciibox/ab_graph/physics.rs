//! Force-directed physics simulation for graph layout.

use petgraph::graph::{DiGraph, NodeIndex};
use petgraph::visit::EdgeRef;

/// Physics simulation parameters.
#[derive(Debug, Clone)]
pub struct PhysicsConfig {
    /// Spring constant for edge attraction (Hooke's law).
    pub spring_constant: f64,
    /// Ideal spring length (rest length of edges).
    pub spring_length: f64,
    /// Repulsion constant between nodes (Coulomb's law).
    pub repulsion_constant: f64,
    /// Downward gravity force.
    pub gravity: f64,
    /// Velocity damping factor (0.0 - 1.0).
    pub damping: f64,
    /// Time step for simulation.
    pub dt: f64,
    /// Velocity threshold for convergence.
    pub velocity_threshold: f64,
    /// Maximum iterations for simulation.
    pub max_iterations: usize,
}

impl Default for PhysicsConfig {
    fn default() -> Self {
        Self {
            spring_constant: 0.05,
            spring_length: 150.0,
            repulsion_constant: 10000.0,
            gravity: 0.3,
            damping: 0.85,
            dt: 1.0,
            velocity_threshold: 0.1,
            max_iterations: 1000,
        }
    }
}

/// 2D vector for physics calculations.
#[derive(Debug, Clone, Copy, Default)]
pub struct Vec2 {
    pub x: f64,
    pub y: f64,
}

impl Vec2 {
    pub fn new(x: f64, y: f64) -> Self {
        Self { x, y }
    }

    pub fn length(self) -> f64 {
        (self.x * self.x + self.y * self.y).sqrt()
    }

    pub fn normalized(self) -> Self {
        let len = self.length();
        if len > 0.0 {
            Self {
                x: self.x / len,
                y: self.y / len,
            }
        } else {
            Self::default()
        }
    }
}

impl std::ops::Add for Vec2 {
    type Output = Self;
    fn add(self, rhs: Self) -> Self {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl std::ops::AddAssign for Vec2 {
    fn add_assign(&mut self, rhs: Self) {
        self.x += rhs.x;
        self.y += rhs.y;
    }
}

impl std::ops::Sub for Vec2 {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self {
        Self {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}

impl std::ops::Mul<f64> for Vec2 {
    type Output = Self;
    fn mul(self, rhs: f64) -> Self {
        Self {
            x: self.x * rhs,
            y: self.y * rhs,
        }
    }
}

/// Node state for physics simulation.
#[derive(Debug, Clone)]
pub struct NodePhysics {
    pub position: Vec2,
    pub velocity: Vec2,
}

impl NodePhysics {
    pub fn new(x: f64, y: f64) -> Self {
        Self {
            position: Vec2::new(x, y),
            velocity: Vec2::default(),
        }
    }
}

/// Physics simulation engine.
pub struct PhysicsEngine {
    pub config: PhysicsConfig,
    pub nodes: Vec<NodePhysics>,
    iteration: usize,
}

impl PhysicsEngine {
    /// Create a new physics engine with initial node positions.
    pub fn new<N, E>(graph: &DiGraph<N, E>, config: PhysicsConfig) -> Self {
        let node_count = graph.node_count();
        let mut nodes = Vec::with_capacity(node_count);

        // Initialize nodes in a grid pattern
        let cols = (node_count as f64).sqrt().ceil() as usize;
        for (i, _) in graph.node_indices().enumerate() {
            let row = i / cols;
            let col = i % cols;
            let x = col as f64 * config.spring_length;
            let y = row as f64 * config.spring_length;
            nodes.push(NodePhysics::new(x, y));
        }

        Self {
            config,
            nodes,
            iteration: 0,
        }
    }

    /// Perform one simulation step.
    pub fn tick<N, E>(&mut self, graph: &DiGraph<N, E>) {
        let node_count = self.nodes.len();
        if node_count == 0 {
            return;
        }

        // Calculate forces
        let mut forces: Vec<Vec2> = vec![Vec2::default(); node_count];

        // Repulsion between all pairs of nodes
        for i in 0..node_count {
            for j in (i + 1)..node_count {
                let delta = self.nodes[i].position - self.nodes[j].position;
                let dist = delta.length().max(1.0);
                let force_mag = self.config.repulsion_constant / (dist * dist);
                let force = delta.normalized() * force_mag;
                forces[i] += force;
                forces[j] = forces[j] - force;
            }
        }

        // Spring forces along edges
        for edge in graph.edge_references() {
            let source = edge.source().index();
            let target = edge.target().index();

            let delta = self.nodes[target].position - self.nodes[source].position;
            let dist = delta.length();
            let displacement = dist - self.config.spring_length;
            let force_mag = self.config.spring_constant * displacement;
            let force = delta.normalized() * force_mag;

            forces[source] += force;
            forces[target] = forces[target] - force;
        }

        // Gravity (downward force)
        for force in &mut forces {
            force.y += self.config.gravity;
        }

        // Apply forces and update positions
        for (i, node) in self.nodes.iter_mut().enumerate() {
            node.velocity = (node.velocity + forces[i] * self.config.dt) * self.config.damping;
            node.position += node.velocity * self.config.dt;
        }

        self.iteration += 1;
    }

    /// Check if simulation has converged (velocities below threshold).
    pub fn is_stable(&self) -> bool {
        if self.iteration >= self.config.max_iterations {
            return true;
        }
        // Need at least a few iterations before checking stability
        if self.iteration < 10 {
            return false;
        }
        self.nodes
            .iter()
            .all(|n| n.velocity.length() < self.config.velocity_threshold)
    }

    /// Run simulation until stable or max iterations reached.
    pub fn run<N, E>(&mut self, graph: &DiGraph<N, E>) {
        while !self.is_stable() {
            self.tick(graph);
        }
    }

    /// Get the current iteration count.
    pub fn iterations(&self) -> usize {
        self.iteration
    }

    /// Get position of a node by index.
    pub fn position(&self, node: NodeIndex) -> Vec2 {
        self.nodes[node.index()].position
    }

    /// Normalize positions to start from (0, 0).
    pub fn normalize_positions(&mut self) {
        if self.nodes.is_empty() {
            return;
        }

        let min_x = self.nodes.iter().map(|n| n.position.x).fold(f64::INFINITY, f64::min);
        let min_y = self.nodes.iter().map(|n| n.position.y).fold(f64::INFINITY, f64::min);

        for node in &mut self.nodes {
            node.position.x -= min_x;
            node.position.y -= min_y;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_vec2_operations() {
        let a = Vec2::new(3.0, 4.0);
        assert!((a.length() - 5.0).abs() < 0.001);

        let b = Vec2::new(1.0, 1.0);
        let c = a + b;
        assert!((c.x - 4.0).abs() < 0.001);
        assert!((c.y - 5.0).abs() < 0.001);
    }

    #[test]
    fn test_physics_convergence() {
        let mut graph: DiGraph<&str, &str> = DiGraph::new();
        let a = graph.add_node("A");
        let b = graph.add_node("B");
        graph.add_edge(a, b, "edge");

        let mut engine = PhysicsEngine::new(&graph, PhysicsConfig::default());
        engine.run(&graph);

        assert!(engine.is_stable());
    }
}
