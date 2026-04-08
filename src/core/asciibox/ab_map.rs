use std::collections::HashMap;

use ascii_dag::Graph;

use crate::{
    core::asciibox::{
        ab_cell::{ASharp, Direct},
        ab_parse::{parse_edge, parse_node},
    },
    utils::cn_length,
};

#[derive(Clone, Default)]
pub struct AsciiBoxMap<'a> {
    pub graph: Graph<'a>,
    pub node_index: usize,
    pub node_id_map: HashMap<String, usize>,
}

impl AsciiBoxMap<'_> {
    pub fn load_content(content: &str) -> Self {
        let mut map = AsciiBoxMap {
            graph: Graph::new(),
            node_index: 0,
            node_id_map: HashMap::new(),
        };
        let lines: Vec<&str> = content
            .split('\n')
            .filter(|&s| !s.trim().is_empty())
            .collect();
        for line in lines.iter() {
            let aline = line.replace("\\n", "\n").replace("\t", " ");
            map.parse_line(aline.as_str());
        }
        return map;
    }

    fn parse_line<'a>(&'a mut self, line_content: &'a str) -> bool {
        let mut text: String;
        let mut id: String;
        let mut name: String;
        let mut _sharp: ASharp;

        let mut direct: Direct;
        let mut _a_text: String;
        let mut vtext: String;
        let mut src_node_id: usize;
        let mut dst_node_id: usize;
        (id, name, _sharp, text) = parse_node(line_content);
        src_node_id = self.add_node(&id, &name);
        loop {
            if text.len() < 3 {
                break;
            }
            // edge
            (direct, _a_text, vtext) = parse_edge(text.trim());
            // node
            if vtext.len() <= 0 {
                break;
            }
            (id, name, _sharp, text) = parse_node(vtext.as_str());
            if id.len() == 0 {
                break;
            }
            dst_node_id = self.add_node(&id, &name);
            // TODO: add arrow label
            match direct {
                Direct::None => {}
                Direct::Double => {
                    self.graph.add_edge(src_node_id, dst_node_id, None);
                    self.graph.add_edge(dst_node_id, src_node_id, None);
                }
                Direct::Left | Direct::LeftUp | Direct::LeftDown => {
                    self.graph.add_edge(dst_node_id, src_node_id, None);
                }
                Direct::Right | Direct::RightUp | Direct::RightDown => {
                    self.graph.add_edge(src_node_id, dst_node_id, None);
                }
                Direct::Up => {
                    self.graph.add_edge(dst_node_id, src_node_id, None);
                }
                Direct::Down => {
                    self.graph.add_edge(src_node_id, dst_node_id, None);
                }
            }
            src_node_id = dst_node_id;
        }
        true
    }

    fn add_node(&mut self, id: &String, name: &String) -> usize {
        if self.node_id_map.contains_key(id) {
            return *self.node_id_map.get(id).unwrap();
        }
        self.node_index += 1;
        let node_id = self.node_index;
        self.node_id_map.insert(name.clone(), node_id);
        let n: &'static str = Box::leak(name.clone().into_boxed_str());
        self.graph.add_node_with_size(
            node_id,
            n,
            cn_length(name) + 4,
            name.matches('\n').count() + 3,
        );
        return node_id;
    }

    pub fn show(&self) {
        println!("{}", self.graph.render());
    }

    pub fn show_layout(&self) {
        let ir = self.graph.compute_layout();
        println!("Width: {}, Height: {}", ir.width(), ir.height());
        for node in ir.nodes() {
            println!("{} at ({}, {})", node.label, node.x, node.y);
        }
        for edge in ir.edges() {
            println!(
                "{} at ({}, {}) -> ({}, {})",
                edge.label.unwrap_or_default(),
                edge.from_x,
                edge.from_y,
                edge.to_x,
                edge.to_y
            )
        }
    }

    pub fn render_to_svgbob(&self) {}
}

mod test {
    use crate::core::AsciiBoxMap;

    #[test]
    fn test_abc() {
        let b = AsciiBoxMap::load_content("a-->c\na-->b\na-->d\nb-->g1");
        b.show_layout();
        b.show();
    }
}
