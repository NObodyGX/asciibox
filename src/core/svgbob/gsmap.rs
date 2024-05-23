use std::borrow::BorrowMut;
use std::ops::DerefMut;
use std::u8::MAX;

use nom::IResult;

use super::data::{GArrow, GDirect, GNode};
use super::parse::{parse_arrow, parse_node, valid_arrow_check, valid_node_check};

#[derive(Debug, Clone)]
pub struct GBoard {
    pub nodes: Vec<GNode>,
    pub w: i16,
    pub h: i16,
}

impl GBoard {
    pub fn new() -> Self {
        Self {
            nodes: Vec::new(),
            w: 0,
            h: 0,
        }
    }

    pub fn get(&mut self, id:&String) -> Option<&mut GNode> {
        if self.nodes.is_empty() {
            return None;
        }
        for node in self.nodes.iter_mut() {
            if node.id.eq(id) {
                return Some(node.borrow_mut());
            }
        }
        return None;
    }

    pub fn add(&mut self, id: String, name: String, x: i16, y: i16) {
        self.nodes.push(GNode::new(id, name, x, y));
    }

    pub fn sort(&mut self) {
        self.nodes.sort_by(|a, b| b.x.cmp(&a.x));
    }

    pub fn relocate(&mut self, spaing:i16) {
        // 主要是为了将
        self.w = 0;
        self.h = 0;
        let mut npos: Vec<String> = Vec::new();
        self.sort();
        for node in self.nodes.iter_mut() {
            let key = format!("{}&{}", node.x, node.y);
            if !npos.contains(&key) {
                npos.push(key);
                continue;
            }
            for i in (0..=100).filter(|x| x % spaing == 0) {
                let key = format!("{}&{}", node.x, node.y);
                if !npos.contains(&key) {
                    continue;
                }
                node.y = i;
                npos.push(key);
                if node.x > self.h {
                    self.h = node.x;
                }
                if node.y > self.w {
                    self.w = node.y;
                }
                break;
            }
        }
    }

    pub fn locate_arrow(&mut self, arrows: &Vec<GArrow>) -> Option<&str> {
        for arrow in arrows {
            let src = &arrow.src;
            let dst = &arrow.dst;
            if src.eq(dst) {
                continue;
            }
            let lnode = self.get(src)?.clone();
            let rnode = self.get(dst)?;
            let x = lnode.x;
            let y = lnode.y;
            match arrow.direct {
                GDirect::Left => {
                    rnode.x = x;
                    rnode.y = y+1;
                },
                GDirect::Right => {
                    rnode.x = x;
                    rnode.y = y-1;
                },
                GDirect::Up => {
                    rnode.x = x+1;
                    rnode.y = y;
                }
                GDirect::Down => {
                    rnode.x = x-1;
                    rnode.y = y;
                }
                _ => {

                }
            }

        }
        Some("")
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

    pub fn load_content(&mut self, content: &str) {
        let mut lines = content.lines();
        let mut linenum: i16 = 0;
        for line in &mut lines {
            match self.parse_line(line, linenum) {
                Ok(_) => {
                    linenum += 1;
                }
                Err(e) => {
                    println!("{:?}", e);
                    continue;
                }
            }
            println!("load content done.");
        }
        self.board_rebuild();
    }

    fn parse_line<'a>(&'a mut self, line: &'a str, linenum: i16) -> IResult<&str, &str> {
        let mut text: &str;
        let mut vtext: &str;
        let mut direct: GDirect;
        let mut grid: Vec<GNode> = Vec::new();
        let mut lid: String;
        let mut rid: String;
        let mut y: i16 = 0;

        (text, vtext) = valid_node_check(line)?;
        let (id, name) = parse_node(vtext)?;
        grid.push(GNode::new(id.to_string(), name.to_string(), linenum, y));
        lid = id.to_string();
        loop {
            y += 1;
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
            grid.push(GNode::new(id.to_string(), name.to_string(), linenum, y));
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
            self.board
                .add(id.to_string(), name.to_string(), node.x, node.y);
            node_ids.push(node.id.to_string());
        }
        Ok(("", ""))
    }

    fn board_rebuild(&mut self) {
        self.board.relocate(2);
        self.board.locate_arrow(&self.arrows);
    }
}
