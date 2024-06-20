use super::node::{ADirect, AEdge};
use std::cmp::max;
use std::collections::HashMap;
use std::ops::Not;

#[derive(Debug, Clone)]
pub struct AGraphNode {
    // 横坐标，对应水平行上的位置
    pub x: usize,
    // 纵坐标，对应垂直列上的位置
    pub y: usize,
    // 保留所在位置的级别，如果级别比其他的小，则保留位置，否则需要让出位置
    level: usize,
}

impl AGraphNode {
    #[must_use]
    pub fn new(hold: usize) -> Self {
        Self {
            x: hold,
            y: hold,
            level: 0,
        }
    }
}

#[derive(Debug, Clone)]
pub struct AGraph {
    members: Vec<String>,
    edges: Vec<AEdge>,
    pub nodes: HashMap<String, AGraphNode>,
    limit: usize,
}

impl AGraph {
    pub fn new(limit: usize) -> Self {
        Self {
            members: Vec::new(),
            edges: Vec::new(),
            nodes: HashMap::new(),
            limit,
        }
    }

    pub fn check_member(&self, id: &String) -> bool {
        if self.members.contains(id) {
            return true;
        }
        return false;
    }

    pub fn add_member(&mut self, id: &String) {
        if self.members.contains(id) {
            return;
        }
        self.members.push(id.clone());
    }

    pub fn add_edge(&mut self, edge: &AEdge) {
        if self.edges.contains(edge) {
            return;
        }
        self.edges.push(edge.clone());
    }

    pub fn merge(&mut self, graph: &AGraph) {
        for id in graph.members.iter() {
            if !self.members.contains(id) {
                self.members.push(id.clone());
            }
        }
        for edge in graph.edges.iter() {
            if !self.edges.contains(edge) {
                self.edges.push(edge.clone());
            }
        }
    }

    fn is_node_exist(&self, x: usize, y: usize) -> bool {
        for (_id, node) in self.nodes.iter() {
            if node.x == x && node.y == y {
                return true;
            }
        }
        false
    }

    fn node_move(&mut self, id: &String, x: usize, y: usize, level: usize) {
        let node = self.nodes.get_mut(id).unwrap();
        node.x = x;
        node.y = y;
        node.level = level;
    }

    fn try_move(&mut self, id: &String, x: usize, y: usize, level: usize) -> bool {
        if !self.is_node_exist(x, y) {
            self.node_move(id, x, y, level);
            return true;
        }
        false
    }

    fn is_node_located(&self, id: &String) -> bool {
        let node = self.nodes.get(id).unwrap();
        node.x != self.nodes.len()
    }
    fn is_remain_unseated(&self) -> bool {
        for (_name, node) in self.nodes.iter() {
            if node.x == self.nodes.len() {
                return true;
            }
        }
        false
    }

    fn nodes_down(&mut self) {
        let l = self.nodes.len();
        for (_id, node) in self.nodes.iter_mut() {
            if node.y != l {
                node.y += 1;
            }
        }
    }

    fn nodes_right(&mut self) {
        let l = self.nodes.len();
        for (_id, node) in self.nodes.iter_mut() {
            if node.x != l {
                node.x += 1;
            }
        }
    }

    fn assign_node_seat(&mut self, src: &String, dst: &String, direct: ADirect) {
        let l1 = self.is_node_located(src);
        let l2 = self.is_node_located(dst);
        if !l1 && !l2 {
            return;
        }
        let (src, dst, dir, neg) = if l1 {
            (src, dst, direct.clone(), false)
        } else {
            (dst, src, direct.clone().not(), true)
        };

        let x = self.nodes.get(src).unwrap().x;
        let y = self.nodes.get(src).unwrap().y;
        match dir {
            ADirect::Left | ADirect::Right => {
                if x == 0 && neg {
                    self.nodes_right();
                }
                let nx = if !neg { x + 1 } else { max(x, 1) - 1 };
                if !self.try_move(dst, nx, y, 1) {
                    for i in 1..self.limit {
                        if self.try_move(dst, nx, y + i, 1 + i) {
                            break;
                        }
                    }
                }
            }
            ADirect::Up => {
                // src --^ dst
                if y == 0 && !neg {
                    self.nodes_down();
                }
                let ny = if !neg { max(y, 1) - 1 } else { y + 1 };
                if !self.try_move(dst, x, ny, 1) {
                    for i in 1..self.limit {
                        if self.try_move(dst, x + i, ny, 1 + i * 2) {
                            break;
                        }
                    }
                }
            }
            ADirect::Down => {
                // src --v dst
                if y == 0 && neg {
                    self.nodes_down();
                }
                let ny = if !neg { y + 1 } else { max(y, 1) - 1 };
                if !self.try_move(dst, x, ny, 1) {
                    for i in 1..self.limit {
                        if self.try_move(dst, x + i, ny, 1 + i * 2) {
                            break;
                        }
                    }
                }
            }
            _ => {}
        }
    }

    //
    pub fn assign_seats(&mut self) {
        let l = self.members.len();
        for id in self.members.iter() {
            self.nodes.insert(id.clone(), AGraphNode::new(l));
        }
        for cnt in 0..self.edges.len() {
            if !self.is_remain_unseated() {
                break;
            }
            for (i, edge) in self.edges.clone().iter().enumerate() {
                let src = &edge.src;
                let dst = &edge.dst;
                let direct = edge.direct.clone();
                if i == 0 && cnt == 0 {
                    self.node_move(src, 0, 0, 1);
                }
                self.assign_node_seat(src, dst, direct);
            }
        }
    }
}
