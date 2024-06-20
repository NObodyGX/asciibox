use super::node::{ADirect, AEdge, ANode, ASharp};
use crate::core::svgbob::parse::{parse_edge, parse_node};
use std::cmp::{max, min};
use std::collections::HashMap;

#[derive(Debug, Clone)]
struct AGraph {
    members: Vec<String>,
    edges: Vec<AEdge>,
    nodes: HashMap<String, ANode>,
}

impl AGraph {
    pub fn new() -> Self {
        Self {
            members: Vec::new(),
            edges: Vec::new(),
            nodes: HashMap::new(),
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

    fn node_move(&mut self, id: &String, x: usize, y: usize) {
        let node = self.nodes.get_mut(id).unwrap();
        node.x = x;
        node.y = y;
    }

    fn is_node_located(&self, id: &String) -> bool {
        let node = self.nodes.get(id).unwrap();
        node.x != self.nodes.len()
    }
    fn is_unlocated(&self) -> bool {
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

    fn align_node(&mut self, src: &String, dst: &String, direct: ADirect) {
        let l1 = self.is_node_located(src);
        let l2 = self.is_node_located(dst);
        if !l1 && !l2 {
            return;
        }
        if l1 {
            let x = self.nodes.get(src).unwrap().x;
            let y = self.nodes.get(src).unwrap().y;
            match direct {
                ADirect::Left | ADirect::Right => {
                    // src <-- dst
                    if !self.is_node_exist(x + 1, y) {
                        self.node_move(dst, x + 1, y)
                    }
                }
                ADirect::Up => {
                    // src --^ dst
                    if x == 0 {
                        self.nodes_down();
                    }
                    if !self.is_node_exist(max(x, 1) - 1, y) {
                        self.node_move(dst, max(x, 1) - 1, y)
                    }
                }
                ADirect::Down => {
                    // src --v dst
                    if !self.is_node_exist(x, y + 1) {
                        self.node_move(dst, x, y + 1);
                    }
                }
                _ => {}
            }
        } else {
            let x = self.nodes.get(dst).unwrap().x;
            let y = self.nodes.get(dst).unwrap().y;
            match direct {
                ADirect::Left | ADirect::Right => {
                    // src <-- dst
                    if x == 0 {
                        self.nodes_right();
                    }
                    if !self.is_node_exist(max(x, 1) - 1, y) {
                        self.node_move(src, max(x, 1) - 1, y)
                    }
                }
                ADirect::Up => {
                    // src --^ dst
                    if !self.is_node_exist(x, y + 1) {
                        self.node_move(src, x, y + 1);
                    }
                }
                ADirect::Down => {
                    // src --v dst
                    if x == 0 {
                        self.nodes_down();
                    }
                    if !self.is_node_exist(max(x, 1) - 1, y) {
                        self.node_move(src, max(x, 1) - 1, y)
                    }
                }
                _ => {}
            }
        }
    }

    pub fn align(&mut self) {
        let l = self.members.len();
        for id in self.members.iter() {
            self.nodes.insert(
                id.clone(),
                ANode::new(id.clone(), id.clone(), l, l, ASharp::Round),
            );
        }
        for cnt in 0..self.edges.len() {
            if !self.is_unlocated() {
                break;
            }
            for (i, edge) in self.edges.clone().iter().enumerate() {
                let src = &edge.src;
                let dst = &edge.dst;
                let dir = edge.direct.clone();
                if i == 0 && cnt == 0 {
                    self.node_move(src, 0, 0);
                }
                self.align_node(src, dst, dir);
            }
        }
    }
}

#[derive(Debug, Clone, Default, Copy)]
pub struct RenderBox {
    pub w: usize,
    pub w_left: usize,
    pub w_right: usize,
    pub h: usize,
    pub h_up: usize,
    pub h_down: usize,
    pub h_total: usize,
}

#[derive(Debug, Clone)]
pub struct AMap {
    // 记录所有 node 信息
    nodes: HashMap<String, ANode>,
    // 记录所有 id --> node.id 信息
    node_ids: HashMap<usize, String>,
    // 记录所有 edge 信息
    edges: Vec<AEdge>,
    // 以 (x,y) 的形式来记录相应的 node 位置，用于 render
    canvas: Vec<Vec<usize>>,
    // 以列表的形式来判断组
    graphs: Vec<AGraph>,
    // max w
    w: usize,
    // max h
    h: usize,
    // node 的序号生成起始值
    idx: usize,
    // 是否扩展 box 保证相同
    expand_mode: bool,
}

impl AMap {
    pub fn new(expand_mode: bool) -> Self {
        Self {
            nodes: HashMap::new(),
            node_ids: HashMap::new(),
            edges: Vec::new(),
            canvas: Vec::new(),
            graphs: Vec::new(),
            w: 0,
            h: 0,
            idx: 1,
            expand_mode,
        }
    }

    fn clear(&mut self) {
        self.edges = Vec::new();
        self.nodes = HashMap::new();
        self.node_ids = HashMap::new();
        self.graphs = Vec::new();
        self.w = 0;
        self.h = 0;
        self.idx = 1;
    }

    // 逐行解析出现的节点，如果有多个节点，这几个节点默认是一排的
    // 后续依据节点之间的联系会重排节点位置
    fn parse_line<'a>(&'a mut self, line: &'a str, y: usize) -> bool {
        let mut text: &str;
        let mut vtext: String;
        let mut direct: ADirect;
        let mut lid: String;
        let mut rid: String;
        let mut node: ANode;
        let mut x: usize = 0;
        let mut id: &str;
        let mut name: &str;
        let mut sharp: ASharp;
        let mut a_text: String;

        // 第一个 node
        (id, name, sharp, text) = parse_node(line);
        node = ANode::new(id.to_string(), name.to_string(), x, y, sharp);
        lid = node.id.clone();
        self.add_node(&node);
        loop {
            if text.len() < 3 {
                break;
            }
            x += 1;
            // edge
            (direct, a_text, vtext) = parse_edge(text);
            // node
            if vtext.len() <= 0 {
                break;
            }
            (id, name, sharp, text) = parse_node(vtext.as_str());
            node = ANode::new(id.to_string(), name.to_string(), x, y, sharp);
            rid = node.id.clone();
            self.add_node(&node);
            self.edges
                .push(AEdge::new(direct, lid, rid.clone(), a_text));
            lid = rid;
        }
        true
    }

    fn add_node(&mut self, node: &ANode) -> bool {
        if self.nodes.contains_key(&node.id) {
            return false;
        }
        let mut n_node = node.clone();
        n_node.idx = self.idx;
        self.idx += 1;
        self.w = max(n_node.x + 1, self.w);
        self.h = max(n_node.y + 1, self.h);
        self.node_ids.insert(n_node.idx.clone(), n_node.id.clone());
        self.nodes.insert(n_node.id.clone(), n_node);
        true
    }

    fn get_node_by_index(&self, idx: &usize) -> &ANode {
        let id = self.node_ids.get(idx).unwrap();
        self.nodes.get(id).unwrap()
    }

    fn clear_canvas(&mut self) {
        let w = self.w + 1;
        let h = self.h + 1;
        self.canvas = Vec::with_capacity(h);
        for _ih in 0..h {
            let mut a: Vec<usize> = Vec::with_capacity(w);
            for _ in 0..w {
                a.push(0);
            }
            self.canvas.push(a);
        }
    }

    // 将所有的 nodes 加入
    fn build_canvas(&mut self) {
        self.clear_canvas();
        for (_id, node) in self.nodes.iter() {
            let x = node.x;
            let y = node.y;
            match self.canvas.get_mut(y) {
                Some(v) => {
                    v[x] = node.idx;
                }
                None => {}
            }
        }
    }

    fn search_is_member(&self, id: &String) -> usize {
        for (i, graph) in self.graphs.iter().enumerate() {
            if graph.check_member(id) {
                return i;
            }
        }
        self.graphs.len()
    }

    fn add_into_graph(&mut self, mid1: &String, mid2: &String, edge: &AEdge) {
        let l = self.graphs.len();
        let lock1: usize = self.search_is_member(mid1);
        let lock2: usize = self.search_is_member(mid2);
        if lock1 == l && lock2 == l {
            let mut graph = AGraph::new();
            graph.add_member(mid1);
            graph.add_member(mid2);
            graph.add_edge(edge);
            self.graphs.push(graph);
            return;
        } else if lock1 == l {
            let graph = self.graphs.get_mut(lock2).unwrap();
            graph.add_member(mid1);
            graph.add_edge(edge);
        } else if lock2 == l {
            let graph = self.graphs.get_mut(lock1).unwrap();
            graph.add_member(mid2);
            graph.add_edge(edge);
        } else {
            let g1 = self.graphs.get(max(lock1, lock2)).unwrap().clone();
            let g2 = self.graphs.get_mut(min(lock1, lock2)).unwrap();
            g2.merge(&g1);
            g2.add_edge(edge);
            self.graphs.remove(max(lock1, lock2));
        }
    }

    fn build_board(&mut self) {
        let length = self.nodes.len();
        self.graphs = Vec::with_capacity(length);
        for edge in self.edges.clone().iter() {
            let src = &edge.src;
            let dst = &edge.dst;
            self.add_into_graph(src, dst, edge);
        }
        for graph in self.graphs.iter_mut() {
            graph.align()
        }
        self.w = 0;
        self.h = 0;
        for graph in self.graphs.iter() {
            for (id, node) in graph.nodes.iter() {
                let nnode = self.nodes.get_mut(id).unwrap();
                nnode.x = node.x;
                nnode.y = node.y;
                self.w = max(self.w, node.x + 1);
                self.h = max(self.h, node.y + 1);
            }
        }
    }

    fn debug_show_position(&self) {
        for (id, node) in self.nodes.iter() {
            println!("{}: ({}, {})", id, node.x, node.y);
        }
    }

    fn show(&self) -> String {
        self.debug_show_position();
        let mut rboxes: Vec<RenderBox> = Vec::new();
        for _ in 0..max(self.w + 1, self.h + 1) {
            rboxes.push(RenderBox::default());
        }
        let mut rw: usize = 0;
        let mut rh: usize = 0;
        // 先计算显示的长宽
        for (_id, node) in self.nodes.iter() {
            rw = max(rw, node.x + 1);
            rh = max(rh, node.y + 1);
            for (i, cbox) in rboxes.iter_mut().enumerate() {
                if i == node.x as usize {
                    cbox.w = max(cbox.w, node.content_w());
                    cbox.w_left = max(cbox.w_left, node.left_w());
                    cbox.w_right = max(cbox.w_right, node.right_w());
                }
                if i == node.y as usize {
                    cbox.h = max(cbox.h, node.content_h());
                    cbox.h_up = max(cbox.h_up, node.up_h());
                    cbox.h_down = max(cbox.h_down, node.down_h());
                    cbox.h_total = max(cbox.h_total, node.total_h());
                }
            }
        }
        // 开始逐行打印
        let mut content = String::new();
        for (y, items) in self.canvas.iter().enumerate() {
            let mut linestr: String = String::new();
            if y > rh {
                break;
            }
            let rbox = rboxes.get(y as usize).expect("error");
            let hu = rbox.h_up;
            let hc = rbox.h;
            let maxh = rbox.h_total;
            // 每行里按高度逐行计算
            for h in 0..maxh {
                // 开始逐列取 node 开始渲染
                for (x, idx) in items.iter().enumerate() {
                    if x >= rboxes.len() || x > rw {
                        break;
                    }
                    let rbox2 = rboxes.get(x as usize).expect("error");
                    let wl = rbox2.w_left;
                    let wr = rbox2.w_right;
                    let wc = rbox2.w; // content, when render total, need + 2
                    let wbc = wc + 2;
                    let maxw = wl + wr + wbc;
                    if *idx == 0 {
                        linestr.push_str(" ".repeat(maxw).as_str());
                        continue;
                    }
                    let node = self.get_node_by_index(idx);
                    let v;
                    if h < hu {
                        v = node.render_up(h, maxh, wbc, wl, wr);
                    } else if h < hu + hc {
                        let vv = node.render(
                            h as usize - hu as usize,
                            maxh,
                            wc,
                            wl,
                            wr,
                            self.expand_mode,
                        );
                        v = format!(
                            "{}{}{}",
                            " ".repeat(wl - node.left_w()),
                            vv,
                            " ".repeat(wr - node.right_w())
                        );
                    } else {
                        v = node.render_down(h - hu - hc, maxh, wc + 2, wl, wr)
                    }
                    linestr.push_str(v.as_str());
                }
                linestr = linestr.trim_end().to_string();
                linestr.push('\n');
            }
            content.push_str(linestr.trim_end());
            // trim_end 会清除最后的换行
            if linestr.trim_end().len() > 0 {
                content.push('\n');
            }
        }
        content
    }

    fn build_nodes(&mut self, content: &str) {
        let lines: Vec<&str> = content
            .split('\n')
            .filter(|&s| !s.trim().is_empty())
            .collect();
        let mut y: usize = 0;
        for line in lines.iter() {
            let aline = line.replace("\\n", "\n");
            if self.parse_line(aline.as_str(), y) {
                y += 1;
            }
        }
    }

    pub fn load_content(&mut self, content: &str) -> String {
        self.clear();
        self.build_nodes(content);
        self.build_board();
        self.build_canvas();
        println!("load content done.");
        let content = self.show();
        content
    }
}
