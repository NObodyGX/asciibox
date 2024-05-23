use nom::IResult;

use super::data::{GArrow, GDirect, GNode};
use super::parse::{parse_arrow, parse_node, valid_arrow_check, valid_node_check};


#[derive(Debug, Clone)]
pub struct GBoard {
    pub nodes: Vec<GNode>,
    pub idw: i16,
    pub idh: i16,
}

impl GBoard {
    pub fn new() -> Self {
        Self {
            nodes: Vec::new(),
            idw: 0,
            idh: 0,
        }
    }

    pub fn add(&mut self, id:String, name:String, x:i16) {
        self.nodes.push(GNode::new(id, name, x));
    }

    pub fn relocate(&mut self) {
        // 主要是为了将
        self.idw = 0;
        self.idh = 0;
        let mut last_x: i16 = -32768;
        let mut last_y: i16 = -32768;
        self.nodes.sort_by(|a, b | b.x.cmp(&a.x));
        for node in self.nodes.iter_mut() {
            if node.x == last_x && node.y == last_y {
                node.x = self.idw;
                node.y = self.idh;
                self.idw += 1;
            }

            last_x = node.x;
            last_y = node.y;
        }
    }
}

#[derive(Debug)]
pub struct GSMap {
    board: GBoard,
    arrows: Vec<GArrow>,
}

impl GSMap {
    pub fn new() -> Self {
        Self {
            board: GBoard::new(),
            arrows: Vec::new(),
        }
    }

    pub fn load_content(&mut self, content: &str) -> IResult<&str, &str> {
        let mut lines = content.lines();
        let remain = "";
        let mut linenum: i16 = 0;
        for line in &mut lines {
            let _ = self.parse_line(line, linenum);
            linenum += 1;
            println!("+++++++++");
        }
        self.board_rebuild();
        Ok((remain, "ok"))
    }

    fn parse_line<'a>(&'a mut self, line: &'a str, linenum:i16) -> IResult<&str, &str> {
        let mut text: &str;
        let mut vtext: &str;
        let mut direct: GDirect;
        let mut grid: Vec<GNode> = Vec::new();
        let mut lid: String;
        let mut rid: String;

        (text, vtext) = valid_node_check(line)?;
        let (id, name) = parse_node(vtext)?;
        grid.push(GNode::new(id.to_string(), name.to_string(), linenum));
        lid = id.to_string();
        loop {
            if text.len() < 3 {
                break;
            }
            (text, vtext) = valid_arrow_check(text)?;
            direct = parse_arrow(vtext);
            if text.len() <= 0 {
                break;
            }
            (text, vtext) = valid_node_check(text)?;
            let (id, name) = parse_node(vtext)?;
            grid.push(GNode::new(id.to_string(), name.to_string(), linenum));
            rid = id.to_string();
            self.arrows
                .push(GArrow::new(direct, lid.clone(), rid.clone()));
            lid = rid;
        }

        let mut node_ids: Vec<String> = Vec::new();
        for node in grid.iter() {
            if node_ids.contains(&node.id) {
                continue;
            }
            self.board.add(id.to_string(), name.to_string(), node.x);
            node_ids.push(node.id.to_string());
        }
        println!("finished!");
        self.board_rebuild();
        Ok(("", ""))
    }

    fn board_rebuild(&mut self) {
        self.board.relocate();
    }
}
