use super::cell::{ACell, ADirect, AEdge, ASharp};
use super::graph::AGraph;
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
    cells: HashMap<String, ACell>,
    // 记录所有 edge 信息
    edges: Vec<AEdge>,
    // 以列表的形式来判断组
    graphs: Vec<AGraph>,
    // max w
    w: usize,
    // max h
    h: usize,
    // 是否扩展 box 保证相同
    expand_mode: bool,
}

impl AMap {
    pub fn new(expand_mode: bool) -> Self {
        Self {
            cells: HashMap::new(),
            edges: Vec::new(),
            graphs: Vec::new(),
            w: 0,
            h: 0,
            expand_mode,
        }
    }

    fn clear(&mut self) {
        self.edges = Vec::new();
        self.cells = HashMap::new();
        self.graphs = Vec::new();
        self.w = 0;
        self.h = 0;
    }

    // 从输入内容里解析 node 和 edge
    // 这里会做一点预处理
    // TODO: 考虑直接支持多行文本
    fn build_cells(&mut self, content: &str) {
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
        let mut node: ACell;
        let mut id: &str;
        let mut name: &str;
        let mut sharp: ASharp;
        let mut a_text: String;

        // 第一个 node
        (id, name, sharp, text) = parse_node(line);
        node = ACell::new(id, name.to_string(), 0, 0);
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
            node = ACell::new(id, name.to_string(), 0, 0);
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
    fn add_node(&mut self, node: &ACell) -> bool {
        if self.cells.contains_key(&node.id) {
            return false;
        }
        self.cells.insert(node.id.clone(), node.clone());
        true
    }

    fn search_is_member(&self, id: &String) -> usize {
        for (i, graph) in self.graphs.iter().enumerate() {
            if graph.check_member(id) {
                return i;
            }
        }
        self.graphs.len()
    }

    // 添加互相联系的节点
    fn add_into_graph(&mut self, sid: &String, did: &String, edge: &AEdge) {
        let l = self.graphs.len();
        let slock: usize = self.search_is_member(sid);
        let dlock: usize = self.search_is_member(did);
        let src = self.cells.get(sid).unwrap();
        let dst = self.cells.get(did).unwrap();
        // 都不在 graph 中
        if slock == l && dlock == l {
            let mut graph = AGraph::new(999, self.expand_mode);
            graph.add_member(sid, src);
            graph.add_member(did, dst);
            graph.add_edge(edge);
            self.graphs.push(graph);
            return;
        }
        // dst 在
        else if slock == l {
            let graph = self.graphs.get_mut(dlock).unwrap();
            graph.add_member(sid, src);
            graph.add_edge(edge);
        }
        // src 在
        else if dlock == l {
            let graph = self.graphs.get_mut(slock).unwrap();
            graph.add_member(did, dst);
            graph.add_edge(edge);
        }
        // 各自都在，合并 graph
        else {
            let g1 = self.graphs.get(max(slock, dlock)).unwrap().clone();
            let g2 = self.graphs.get_mut(min(slock, dlock)).unwrap();
            g2.merge(&g1);
            g2.add_edge(edge);
            self.graphs.remove(max(slock, dlock));
        }
    }

    // 添加孤儿的节点
    fn add_orphan_graph(&mut self) {
        for (id, cell) in self.cells.iter() {
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
            let mut graph = AGraph::new(1, self.expand_mode);
            graph.add_member(id, cell);
            self.graphs.push(graph);
        }
    }

    fn build_render_box(&self) -> Vec<RenderNode> {
        let mut rboxes: Vec<RenderNode> = Vec::new();
        for _ in 0..max(self.w + 1, self.h + 1) {
            rboxes.push(RenderNode::default());
        }
        // 先计算显示的长宽
        for graph in self.graphs.iter() {
            for (_id, node) in graph.nodes.iter() {
                for (i, cbox) in rboxes.iter_mut().enumerate() {
                    if i == node.x as usize {
                        cbox.w = max(cbox.w, node.w());
                    }
                    if i == node.y as usize {
                        cbox.h = max(cbox.h, node.h());
                    }
                }
            }
        }
        rboxes
    }

    // 重排 nodes 之间的位置
    fn build_board(&mut self) {
        let length = self.cells.len();
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
            self.h += graph.h;
        }

        for graph in self.graphs.iter_mut() {
            graph.build_canvas();
        }
    }

    fn render(&self) -> String {
        // 绘制分为两个部分
        // 第一部分：绘制节点的上 edge 部分（包括上节点的下edge部分）
        // 第二部分：绘制节点和节点的左右 edge 部分
        let rbox: Vec<RenderNode> = self.build_render_box();
        let mut content = String::new();
        for graph in self.graphs.iter() {
            content.push_str(graph.render(&rbox).trim_end());
            content.push('\n');
        }
        content
    }

    pub fn load_content(&mut self, content: &str) -> String {
        self.clear();
        self.build_cells(content);
        self.build_board();
        println!("load content done.");
        let content = self.render();
        content
    }
}
