use nom::IResult;
use std::borrow::BorrowMut;
use std::cmp::max;
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

    pub fn get(&mut self, id: &String) -> Option<&mut GNode> {
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

    pub fn sort(&mut self) {
        self.nodes.sort_by(|a, b| b.x.cmp(&a.x));
    }

    pub fn trim(&mut self) {
        let mut x: i16 = 0;
        let mut y: i16 = 0;
        for node in self.nodes.iter() {
            x = max(x, node.x);
            y = max(y, node.y);
        }
        self.w = x;
        self.h = y;
        let mut vv: Vec<i16> = Vec::new();
        for node in self.nodes.iter() {
            if vv.contains(&node.x) {
                continue;
            }
            vv.push(node.x);
        }
    }

    pub fn relocate(&mut self, id: &String, x: i16, y: i16, mode: &GDirect) {
        let mut sx: i16 = 0;
        let mut sy: i16 = 0;
        let mut flag = false;
        for node in self.nodes.iter_mut() {
            if !node.id.eq(id) {
                continue;
            }
            if node.x == x && node.y == y {
                return;
            }
            node.x = x;
            node.y = y;
            flag = true;
        }
        if !flag {
            return;
        }
        for node in self.nodes.iter_mut() {
            if node.id.eq(id) {
                continue;
            }
            match mode {
                GDirect::Left => {
                    if node.x == x && node.y >= y {
                        node.y += 1;
                    }
                }
                GDirect::Right => {
                    if node.x == x && node.y >= y {
                        node.y += 1;
                    }
                }
                _ => {}
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
            let x = lnode.x;
            let y = lnode.y;
            match arrow.direct {
                GDirect::Left => {
                    self.relocate(&dst, x, y + 1, &arrow.direct);
                }
                _ => {}
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
        // let mut lines = content.lines();
        let mut lines: Vec<&str> = content.split('\n').filter(|&s| !s.is_empty()).collect();
        let mut linenum: i16 = 0;
        for line in lines.iter_mut() {
            match self.parse_line(line, linenum) {
                Ok(_) => {
                    linenum += 1;
                }
                Err(e) => {
                    println!("{:?}", e);
                    continue;
                }
            }
        }
        println!("load content done.");
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
        for node in self.board.nodes.iter() {
            node_ids.push(node.id.to_string());
        }
        for node in grid.iter() {
            if node_ids.contains(&node.id) {
                continue;
            }
            self.board.nodes.push(node.clone());
            node_ids.push(node.id.to_string());
        }
        Ok(("", ""))
    }

    fn board_rebuild(&mut self) {
        self.board.locate_arrow(&self.arrows);
    }

    fn print(&self) {}
}
