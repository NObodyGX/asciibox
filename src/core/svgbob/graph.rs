use super::cell::{ACell, ADirect, AEdge};
use super::maps::RenderBox;
use std::cmp::{max, min};
use std::collections::HashMap;
use std::ops::Not;

#[derive(Debug, Clone)]
pub struct AEdgeCell {
    // 目的所在位置(相对值)
    pub ox: i16,
    // 目的所在位置(相对值)
    pub oy: i16,
    // dst id
    pub id: String,
    // 方向
    pub direct: ADirect,
}

impl AEdgeCell {
    #[must_use]
    pub fn new(id: String, ox: i16, oy: i16, direct: ADirect) -> Self {
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
    level: usize,
    // 位置是否已经固定
    locked: bool,
    l_edges: Vec<AEdgeCell>,
    r_edges: Vec<AEdgeCell>,
    u_edges: Vec<AEdgeCell>,
    d_edges: Vec<AEdgeCell>,
    cell: ACell,
}

impl ANode {
    #[must_use]
    pub fn new(cell: &ACell) -> Self {
        Self {
            x: 0,
            y: 0,
            level: 0,
            locked: false,
            cell: cell.clone(),
            l_edges: Vec::new(),
            r_edges: Vec::new(),
            u_edges: Vec::new(),
            d_edges: Vec::new(),
        }
    }

    pub fn w(&self) -> usize {
        return self.cell.total_w();
    }
    pub fn left(&self) -> usize {
        let w = match self.l_edges.len() {
            0 => 0,
            1 => 3,
            2 => 5,
            3 => 5,
            _ => 5,
        };
        return w;
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

#[derive(Debug, Clone)]
pub struct AGraph {
    pub nodes: HashMap<String, ANode>,
    pub w: usize,
    pub h: usize,

    members: HashMap<String, ACell>,
    edges: Vec<AEdge>,
    limit: usize,
    // 以 (x,y) 的形式来记录相应的 node 位置，用于 render
    canvas: Vec<Vec<String>>,
    emode: bool,
}

impl AGraph {
    pub fn new(limit: usize, emode: bool) -> Self {
        Self {
            members: HashMap::new(),
            edges: Vec::new(),
            nodes: HashMap::new(),
            limit,
            w: 0,
            h: 0,
            canvas: Vec::new(),
            emode,
        }
    }

    pub fn check_member(&self, id: &String) -> bool {
        if self.members.contains_key(id) {
            return true;
        }
        return false;
    }

    pub fn add_member(&mut self, id: &String, cell: &ACell) {
        if self.members.contains_key(id) {
            return;
        }
        self.members.insert(id.clone(), cell.clone());
    }

    pub fn add_edge(&mut self, edge: &AEdge) {
        if self.edges.contains(edge) {
            return;
        }
        self.edges.push(edge.clone());
    }

    pub fn merge(&mut self, graph: &AGraph) {
        for (id, cell) in graph.members.iter() {
            self.add_member(id, cell);
        }
        for edge in graph.edges.iter() {
            self.add_edge(edge);
        }
    }

    fn is_pos_exist_node(&self, x: usize, y: usize) -> bool {
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
        node.locked = true;
    }

    fn try_move(&mut self, id: &String, x: usize, y: usize, level: usize) -> bool {
        if !self.is_pos_exist_node(x, y) {
            self.node_move(id, x, y, level);
            return true;
        }
        false
    }

    // 判断节点位置是否已经固定
    fn is_node_locked(&self, id: &String) -> bool {
        let node = self.nodes.get(id).unwrap();
        node.locked
    }

    // 是否有未固定的节点
    fn is_remain_unlocked(&self) -> bool {
        for (_name, node) in self.nodes.iter() {
            if !node.locked {
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

    fn add_edge_node(
        &mut self,
        src: &String,
        dst: &String,
        dir: ADirect,
        ox: i16,
        oy: i16,
        neg: bool,
    ) {
        let node = self.nodes.get_mut(src).unwrap();
        let edge = AEdgeCell::new(dst.clone(), ox, oy, dir.clone());
        match dir {
            ADirect::Right | ADirect::Left => {
                if neg {
                    node.l_edges.push(edge);
                } else {
                    node.r_edges.push(edge);
                }
            }
            ADirect::Up => {
                if neg {
                    node.d_edges.push(edge);
                } else {
                    node.u_edges.push(edge);
                }
            }
            ADirect::Down => {
                if neg {
                    node.u_edges.push(edge);
                } else {
                    node.d_edges.push(edge);
                }
            }
            _ => {}
        }
    }

    // 固定 src 和 dst 的位置
    fn assign_node_seat(&mut self, src: &String, dst: &String, direct: &ADirect) {
        let l1 = self.is_node_locked(src);
        let l2 = self.is_node_locked(dst);
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
                for i in 0..self.limit {
                    if !self.try_move(dst, nx, y + i, 1 + i * 2) {
                        continue;
                    }
                    self.add_edge_node(src, dst, dir, nx as i16 - x as i16, i as i16, neg);
                    break;
                }
            }
            ADirect::Up => {
                // src --^ dst
                if y == 0 && !neg {
                    self.nodes_down();
                }
                let ny = if !neg { max(y, 1) - 1 } else { y + 1 };
                for i in 0..self.limit {
                    if !self.try_move(dst, x + i, ny, 1 + i * 2) {
                        continue;
                    }
                    self.add_edge_node(src, dst, dir, i as i16, ny as i16 - y as i16, neg);
                    break;
                }
            }
            ADirect::Down => {
                // src --v dst
                if y == 0 && neg {
                    self.nodes_down();
                }
                let ny = if !neg { y + 1 } else { max(y, 1) - 1 };
                for i in 0..self.limit {
                    if !self.try_move(dst, x + i, ny, 1 + i * 2) {
                        continue;
                    }
                    self.add_edge_node(src, dst, dir, i as i16, ny as i16 - y as i16, neg);
                    break;
                }
            }
            _ => {}
        }
    }

    fn fit_wh(&mut self) {
        for (_id, node) in self.nodes.iter() {
            self.w = max(self.w, node.x + 1);
            self.h = max(self.h, node.y + 1);
        }
    }

    // 分配所有的节点位置
    pub fn assign_seats(&mut self) {
        let l = self.members.len();
        if l == 1 {
            for (id, cell) in self.members.iter() {
                self.nodes.insert(id.clone(), ANode::new(cell));
            }
            self.fit_wh();
            return;
        }
        // 生成所有节点
        for (id, cell) in self.members.iter() {
            self.nodes.insert(id.clone(), ANode::new(cell));
        }
        // 根据 edge 依次排列节点的位置
        for cnt in 0..self.edges.len() {
            if !self.is_remain_unlocked() {
                break;
            }
            for (i, edge) in self.edges.clone().iter().enumerate() {
                let src = &edge.src;
                let dst = &edge.dst;
                if i == 0 && cnt == 0 {
                    self.node_move(src, 0, 0, 1);
                }
                self.assign_node_seat(src, dst, &edge.direct);
            }
        }
        self.fit_wh();
    }

    fn render_edge_up(&self, y: usize, rbox: &Vec<RenderBox>) -> String {
        for x in 0..self.w + 1 {
            let _maxw = rbox.get(x).unwrap().w;
            let _cid = self.canvas.get(y).unwrap().get(x).unwrap();
        }
        "".to_string()
    }

    fn do_render_cell(&self, i: usize, x: usize, y: usize, rbox: &Vec<RenderBox>) -> String {
        let mut content = String::new();
        let maxw = rbox.get(x).unwrap().w;
        let cid = self.canvas.get(y).unwrap().get(x).unwrap();
        if cid.is_empty() {
            content.push_str(" ".repeat(maxw).as_str());
        } else {
            let cell = self.members.get(cid).unwrap();
            content.push_str(cell.do_render(i, maxw, self.emode).trim_end());
        }
        content
    }

    fn do_render_right(&self, i: usize, x: usize, y: usize, rbox: &Vec<RenderBox>) -> String {
        // 这里应该和 cell 一样，也是需要找到这个的最大宽度
        let mut content = String::new();

        let maxh = rbox.get(y).unwrap().h;
        let maxw = max(
            rbox.get(x).unwrap().right,
            rbox.get(min(rbox.len() - 1, x + 1)).unwrap().left,
        );

        let cid = self.canvas.get(y).unwrap().get(x).unwrap();
        // TODO 这里是不合理的
        if cid.is_empty() {
            content.push_str(" ".repeat(maxw).as_str());
            return content;
        }

        let node = self.nodes.get(cid).unwrap();

        let udis = ((maxh - 1) / 2 - 1) / 2;
        let ddis = ((maxh + 1) / 2 + 1) / 2;
        let mut flag: bool = false;
        // 判断上节点
        if i == udis {
            // 右侧
            for ec in node.r_edges.iter() {
                if ec.ox > 0 && ec.oy < 0 {
                    content.push_str("-".repeat((maxw + 1) / 2).as_str());
                    content.push('\'');
                    content.push_str(" ".repeat((maxw - 1) / 2).as_str());
                    flag = true;
                    break;
                }
            }
            // 左侧
            for (_nid, aode) in self.nodes.iter() {
                if flag {
                    break;
                }
                if !(aode.x > x && aode.y > y) {
                    continue;
                }
                for ec in aode.l_edges.iter() {
                    if ec.id.eq(cid) {
                        content.push_str("-".repeat((maxw - 1) / 2).as_str());
                        content.push('\'');
                        content.push_str(" ".repeat((maxw + 1) / 2).as_str());
                        flag = true;
                        break;
                    }
                }
            }
        }
        // 判断中节点
        else if i == maxh / 2 {
            // 右侧
            for ec in node.r_edges.iter() {
                if ec.ox > 0 && ec.oy == 0 {
                    if ec.direct == ADirect::Left {
                        content.push('<');
                        content.push_str("-".repeat(maxw - 1).as_str());
                    } else {
                        content.push_str("-".repeat(maxw - 1).as_str());
                        content.push('>');
                    }
                    flag = true;
                    break;
                }
            }
            // 左侧
            for (_nid, aode) in self.nodes.iter() {
                if flag {
                    break;
                }
                if aode.x <= x || aode.y != y {
                    continue;
                }
                for ec in aode.l_edges.iter() {
                    if ec.id.eq(cid) {
                        if ec.direct == ADirect::Left {
                            content.push('<');
                            content.push_str("-".repeat(maxw - 1).as_str());
                        } else {
                            content.push_str("-".repeat(maxw - 1).as_str());
                            content.push('>');
                        }
                        flag = true;
                        break;
                    }
                }
            }
        }
        // 判断下节点
        else if i == maxh - ddis {
            // 右侧
            for ec in node.r_edges.iter() {
                if ec.ox > 0 && ec.oy > 0 {
                    content.push_str("-".repeat((maxw - 1) / 2).as_str());
                    content.push('.');
                    content.push_str(" ".repeat((maxw + 1) / 2).as_str());
                    flag = true;
                    break;
                }
            }
            // 左侧
            for (_nid, aode) in self.nodes.iter() {
                if flag {
                    break;
                }
                if aode.x <= x || aode.y <= y {
                    continue;
                }
                for ec in aode.l_edges.iter() {
                    if ec.id.eq(cid) {
                        content.push_str("-".repeat((maxw - 1) / 2).as_str());
                        content.push('.');
                        content.push_str(" ".repeat((maxw + 1) / 2).as_str());
                        flag = true;
                        break;
                    }
                }
            }
        } else {
            content.push_str(" ".repeat(maxw).as_str());
            return content;
        }
        if content.len() < maxw {
            content.push_str(" ".repeat(maxw - content.len()).as_str());
        }

        content
    }

    fn render_cell_with_edge(&self, y: usize, rbox: &Vec<RenderBox>) -> String {
        let mut content = String::new();
        let maxh = rbox.get(y).unwrap().h;

        for i in 0..maxh + 1 {
            let mut line = String::new();
            for x in 0..self.w + 1 {
                line.push_str(self.do_render_cell(i, x, y, rbox).as_str());
                line.push_str(self.do_render_right(i, x, y, rbox).as_str());
            }
            content.push_str(line.trim_end());
            content.push('\n');
        }

        content
    }

    fn print_members(&self) {
        println!("graph");
        for (name, cell) in self.nodes.iter() {
            println!("{}: ({}, {})", name, cell.x, cell.y);
        }
        println!("graph end!!!")
    }

    // 绘制本graph
    pub fn render(&self, rbox: &Vec<RenderBox>) -> String {
        self.print_members();
        // 绘制分为两个部分
        // 第一部分：绘制节点的上 edge 及上节点的下 edge
        // 第二部分：绘制节点和节点的左右 edge 部分
        let mut content = String::new();
        for y in 0..self.h + 1 {
            let u_letters = self.render_edge_up(y, &rbox);
            let c_letters = self.render_cell_with_edge(y, &rbox);
            content.push_str(u_letters.trim_end());
            content.push_str(c_letters.trim_end());
            content.push('\n');
        }

        content
    }

    pub fn build_canvas(&mut self) {
        // clear canvas
        let w = self.w + 1;
        let h = self.h + 1;
        self.canvas = Vec::with_capacity(h);
        for _ih in 0..h {
            let mut a: Vec<String> = Vec::with_capacity(w);
            for _ in 0..w {
                a.push("".to_string());
            }
            self.canvas.push(a);
        }

        // fill canvas
        for (id, node) in self.nodes.iter() {
            let x = node.x;
            let y = node.y;
            match self.canvas.get_mut(y) {
                Some(v) => {
                    v[x] = id.clone();
                }
                None => {}
            }
        }
    }
}
