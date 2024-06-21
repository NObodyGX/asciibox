use super::graph::AGraph;
use super::node::{ADirect, AEdge, ANode, ASharp};
use super::parse::{parse_edge, parse_node};
use std::cmp::{max, min};
use std::collections::HashMap;

#[derive(Debug, Clone, Default, Copy)]
pub struct RenderNode {
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

    // 从输入内容里解析 node 和 edge
    // 这里会做一点预处理
    // TODO: 考虑直接支持多行文本
    fn build_nodes(&mut self, content: &str) {
        let lines: Vec<&str> = content
            .split('\n')
            .filter(|&s| !s.trim().is_empty())
            .collect();
        for line in lines.iter() {
            let aline = line.replace("\\n", "\n").replace("\t", " ");
            self.parse_line(aline.as_str());
        }
    }

    // 逐行解析出现的节点
    // 后续依据节点之间的关系重排节点位置
    fn parse_line<'a>(&'a mut self, line: &'a str) -> bool {
        let mut text: &str;
        let mut vtext: String;
        let mut direct: ADirect;
        let mut lid: String;
        let mut rid: String;
        let mut node: ANode;
        let mut id: &str;
        let mut name: &str;
        let mut sharp: ASharp;
        let mut a_text: String;

        // 第一个 node
        (id, name, sharp, text) = parse_node(line);
        node = ANode::new(id, name.to_string(), 0, 0);
        node.set_sharp(sharp);
        lid = node.id.clone();
        self.add_node(&node);
        loop {
            if text.len() < 3 {
                break;
            }
            // edge
            (direct, a_text, vtext) = parse_edge(text);
            // node
            if vtext.len() <= 0 {
                break;
            }
            (id, name, sharp, text) = parse_node(vtext.as_str());
            if id.len() == 0 {
                break;
            }
            node = ANode::new(id, name.to_string(), 0, 0);
            node.set_sharp(sharp);
            rid = node.id.clone();
            self.add_node(&node);
            self.edges
                .push(AEdge::new(direct, lid, rid.clone(), a_text));
            lid = rid;
        }
        true
    }

    // 将 node 加入到 graph 中
    fn add_node(&mut self, node: &ANode) -> bool {
        if self.nodes.contains_key(&node.id) {
            return false;
        }
        let mut n_node = node.clone();
        n_node.idx = self.idx;
        self.idx += 1;
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
            let mut graph = AGraph::new(999);
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

    fn add_orphan_graph(&mut self) {
        for (id, _node) in self.nodes.iter() {
            let mut flag = false;
            for graph in self.graphs.iter() {
                if graph.check_member(id) {
                    flag = true;
                    break;
                }
            }
            if flag {
                continue;
            }
            let mut graph = AGraph::new(1);
            graph.add_member(id);
            self.graphs.push(graph);
        }
    }

    // 重排 nodes 之间的位置
    fn build_board(&mut self) {
        let length = self.nodes.len();
        self.graphs = Vec::with_capacity(length);
        // 先添加集合体
        for edge in self.edges.clone().iter() {
            let src = &edge.src;
            let dst = &edge.dst;
            self.add_into_graph(src, dst, edge);
        }
        // 再添加一个孤儿
        self.add_orphan_graph();
        for graph in self.graphs.iter_mut() {
            graph.assign_seats()
        }

        self.w = 0;
        self.h = 0;
        for graph in self.graphs.iter() {
            self.w = max(self.w, graph.w);
            for (id, node) in graph.nodes.iter() {
                let nnode = self.nodes.get_mut(id).unwrap();
                nnode.x = node.x;
                nnode.y = self.h + node.y;
            }
            self.h += graph.h;
        }
    }

    fn __show_position(&self) {
        for (id, node) in self.nodes.iter() {
            println!("{}: ({}, {})", id, node.x, node.y);
        }
    }

    fn build_render_nodes(&self) -> Vec<RenderNode> {
        let mut rboxes: Vec<RenderNode> = Vec::new();
        for _ in 0..max(self.w + 1, self.h + 1) {
            rboxes.push(RenderNode::default());
        }
        // 先计算显示的长宽
        for (_id, node) in self.nodes.iter() {
            for (i, cbox) in rboxes.iter_mut().enumerate() {
                if i == node.x as usize {
                    cbox.w = max(cbox.w, node.total_w());
                }
                if i == node.y as usize {
                    cbox.h = max(cbox.h, node.total_h());
                }
            }
        }
        rboxes
    }

    fn render_edge_up(&self, y: usize, rbox: &Vec<RenderNode>) -> String {
        for x in 0..self.w + 1 {
            let _maxw = rbox.get(x).unwrap().w;
            let _rid = self.canvas.get(y).unwrap().get(x).unwrap();
        }
        "".to_string()
    }

    fn render_node_with_edge(&self, y: usize, rbox: &Vec<RenderNode>) -> String {
        let mut content = String::new();
        let emode = self.expand_mode;
        let maxh = rbox.get(y).unwrap().h;

        for h in 0..maxh + 1 {
            let mut line = String::new();
            for x in 0..self.w + 1 {
                // // render_edge_left
                // let maxlw = rbox.get(x).unwrap().w_left;
                // let maxrw = if x == 0 {
                //     0
                // } else {
                //     rbox.get(x - 1).unwrap().w_right
                // };
                // let maxw = max(maxlw, maxrw);
                // // todo render
                // render_node
                let maxw = rbox.get(x).unwrap().w;
                let nid = self.canvas.get(y).unwrap().get(x).unwrap();
                if nid == &0 {
                    line.push_str(" ".repeat(maxw).as_str());
                } else {
                    let node = self.get_node_by_index(nid);
                    line.push_str(node.render(h, maxw, emode).trim_end());
                }
            }
            content.push_str(line.trim_end());
            content.push('\n');
        }

        content
    }

    fn render(&self) -> String {
        // 绘制分为两个部分
        // 第一部分：绘制节点的上 edge 部分（包括上节点的下edge部分）
        // 第二部分：绘制节点和节点的左右 edge 部分
        self.__show_position();
        let rbox: Vec<RenderNode> = self.build_render_nodes();
        let mut content = String::new();
        for graph in self.graphs.iter() {
            content.push_str(graph.render().trim_end());
            content.push('\n');
        }
        for y in 0..self.h + 1 {
            let u_letters = self.render_edge_up(y, &rbox);
            let c_letters = self.render_node_with_edge(y, &rbox);
            content.push_str(u_letters.trim_end());
            content.push_str(c_letters.trim_end());
            content.push('\n');
        }
        content
    }

    pub fn load_content(&mut self, content: &str) -> String {
        self.clear();
        self.build_nodes(content);
        self.build_board();
        self.build_canvas();
        println!("load content done.");
        let content = self.render();
        content
    }
}
