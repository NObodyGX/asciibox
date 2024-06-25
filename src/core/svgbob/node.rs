use super::cell::{Cell, Direct};

#[derive(Debug, Clone)]
pub struct AEdgeCell {
    // 目的所在位置(相对值)
    pub ox: i16,
    // 目的所在位置(相对值)
    pub oy: i16,
    // dst id
    pub id: String,
    // 方向
    pub direct: Direct,
}

impl AEdgeCell {
    #[must_use]
    pub fn new(id: String, ox: i16, oy: i16, direct: Direct) -> Self {
        Self { id, ox, oy, direct }
    }
}

#[derive(Debug, Clone)]
pub struct ANode {
    // 横坐标，对应水平行上的位置
    pub x: usize,
    // 纵坐标，对应垂直列上的位置
    pub y: usize,
    // 保留所在位置的级别，如果级别比其他的小，则保留位置，否则需要让出位置
    pub level: usize,
    // 位置是否已经固定
    pub locked: bool,
    pub r_edges: Vec<AEdgeCell>,
    pub d_edges: Vec<AEdgeCell>,
    cell: Cell,
}

impl ANode {
    #[must_use]
    pub fn new(cell: &Cell) -> Self {
        Self {
            x: 0,
            y: 0,
            level: 0,
            locked: false,
            cell: cell.clone(),
            r_edges: Vec::new(),
            d_edges: Vec::new(),
        }
    }

    pub fn w(&self) -> usize {
        return self.cell.total_w();
    }

    pub fn right(&self) -> usize {
        let w = match self.r_edges.len() {
            0 => 0,
            1 => 3,
            2 => 5,
            3 => 5,
            _ => 5,
        };
        return w;
    }
    pub fn h(&self) -> usize {
        return self.cell.total_h();
    }
}
