use super::node::{GArrow, GDirect, GNode};
use super::parse::{parse_arrow, parse_node, valid_arrow_check, valid_node_check};
use nom::IResult;
use std::borrow::BorrowMut;

#[derive(Debug, Clone)]
pub struct GBoard {
    pub nodes: Vec<GNode>,
    pub w: u16,
    pub h: u16,
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

    // pub fn sort(&mut self) {
    //     self.nodes.sort_by(|a, b| b.x.cmp(&a.x));
    // }

    // pub fn trim(&mut self) {
    //     let mut x: u16 = 0;
    //     let mut y: u16 = 0;
    //     for node in self.nodes.iter() {
    //         x = max(x, node.x);
    //         y = max(y, node.y);
    //     }
    //     self.w = x;
    //     self.h = y;
    //     let mut vv: Vec<u16> = Vec::new();
    //     for node in self.nodes.iter() {
    //         if vv.contains(&node.x) {
    //             continue;
    //         }
    //         vv.push(node.x);
    //     }
    // }

    pub fn relocate(&mut self, id: &String, x: u16, y: u16, mode: &GDirect) {
        let mut flag = false;
        for node in self.nodes.iter_mut() {
            if !node.id.eq(id) {
                continue;
            }
            // if node.x == x && node.y == y {
            //     return;
            // }
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
        let mut w = 0;
        let mut h = 0;
        for node in self.nodes.iter() {
            w = std::cmp::max(w, node.y + 1);
            h = std::cmp::max(h, node.x + 1);
        }
        self.h = h;
        self.w = w;
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
                    self.relocate(&dst, x, y - 1, &arrow.direct);
                }
                GDirect::Right => {
                    self.relocate(&dst, x, y + 1, &arrow.direct);
                }
                _ => {}
            }
        }
        Some("")
    }

    pub fn show(&self) {
        // cal width and height

        let mut w_val: Vec<usize> = Vec::new();
        let mut h_val: Vec<usize> = Vec::new();
        for _ in 0..self.nodes.len() {
            w_val.push(0);
            h_val.push(0);
        }
        for node in self.nodes.iter() {
            w_val[node.x as usize] = std::cmp::max(w_val[node.x as usize], node.ww());
            h_val[node.y as usize] = std::cmp::max(h_val[node.y as usize], node.hh());
        }
        // 逐行打印
        for x in 0..self.h {
            let mut content: String = String::new();
            for h in 0..h_val[x as usize] {
                for node in self.nodes.iter() {
                    if node.x != x {
                        continue;
                    }
                    content.push_str(
                        node.show(h as u16, h_val[node.y as usize], w_val[node.x as usize])
                            .as_str(),
                    );
                }
                content.push('\n');
            }
            println!("{}", content);
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

    pub fn load_content(&mut self, content: &str) {
        // let mut lines = content.lines();
        let mut lines: Vec<&str> = content.split('\n').filter(|&s| !s.is_empty()).collect();
        let mut linenum: u16 = 0;
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
        self.show();
    }

    fn parse_line<'a>(&'a mut self, line: &'a str, linenum: u16) -> IResult<&str, &str> {
        let mut text: &str;
        let mut vtext: &str;
        let mut direct: GDirect;
        let mut grid: Vec<GNode> = Vec::new();
        let mut lid: String;
        let mut rid: String;
        let mut h: u16 = 0;
        let mut w: u16 = 0;

        (text, vtext) = valid_node_check(line)?;
        let (id, name) = parse_node(vtext)?;
        grid.push(GNode::new(id.to_string(), name.to_string(), linenum, h));
        lid = id.to_string();
        w = 0;
        loop {
            h += 1;
            w += 1;
            if text.len() < 3 {
                break;
            }
            (text, vtext) = valid_arrow_check(text)?;
            direct = parse_arrow(vtext);
            if text.len() <= 0 {
                break;
            }
            w += 1;
            (text, vtext) = valid_node_check(text)?;
            let (id, name) = parse_node(vtext)?;
            grid.push(GNode::new(id.to_string(), name.to_string(), linenum, h));
            rid = id.to_string();
            self.arrows.push(GArrow::new(
                direct,
                lid.clone().trim().to_string(),
                rid.clone().trim().to_string(),
            ));
            lid = rid;
        }
        self.board.w = w;
        self.board.h = h;
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

    fn show(&self) {
        self.board.show();
    }
}
