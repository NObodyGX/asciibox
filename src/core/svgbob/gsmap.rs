use super::node::{GArrow, GDirect, GNode};
use super::parse::{parse_arrow, parse_node, valid_arrow_check, valid_node_check};
use nom::IResult;
use std::borrow::BorrowMut;

#[derive(Debug, Clone)]
pub struct GBoard {
    pub nodes: Vec<GNode>,
    pub board: Vec<Vec<u8>>,
    pub w: u16,
    pub h: u16,
    idx: u8,
}

impl GBoard {
    pub fn new() -> Self {
        Self {
            nodes: Vec::new(),
            board: Vec::new(),
            w: 0,
            h: 0,
            idx: 1,
        }
    }

    pub fn add_node(&mut self, node: &GNode) -> bool {
        for x in self.nodes.iter() {
            if x.id.eq(&node.id) {
                return false;
            }
        }
        let mut a_node = node.clone();
        a_node.idx = self.idx;
        self.idx += 1;
        self.h = std::cmp::max(a_node.x + 1, self.h);
        self.w = std::cmp::max(a_node.y + 1, self.w);
        self.nodes.push(a_node);
        true
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

    pub fn show(&self) -> String {
        // cal width and height
        let mut content = String::new();
        let mut w_val: Vec<usize> = Vec::new(); // 每行 cell 的宽度
        let mut h_val: Vec<usize> = Vec::new(); // 每行的高度
        for _ in 0..self.nodes.len() {
            w_val.push(0);
            h_val.push(0);
        }
        for node in self.nodes.iter() {
            w_val[node.y as usize] = std::cmp::max(w_val[node.y as usize], node.ww());
            h_val[node.x as usize] = std::cmp::max(h_val[node.x as usize], node.hh());
        }
        // 逐行打印
        for x in 0..self.h {
            let mut linestr: String = String::new();
            for h in 0..h_val[x as usize] {
                for node in self.nodes.iter() {
                    if node.x != x {
                        continue;
                    }
                    linestr.push_str(
                        node.show(h as u16, h_val[x as usize], w_val[node.y as usize])
                            .as_str(),
                    );
                }
                linestr.push('\n');
            }
            println!("{}", linestr);
            content.push_str(linestr.clone().as_str());
        }
        content
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

    pub fn load_content(&mut self, content: &str) -> String {
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
        let content = self.board.show();
        content
    }

    // 逐行解析出现的节点，如果有多个节点，这几个节点默认是一排的
    // 后续依据节点之间的联系会重排节点位置
    fn parse_line<'a>(&'a mut self, line: &'a str, linenum: u16) -> IResult<&str, &str> {
        let mut text: &str;
        let mut vtext: &str;
        let mut direct: GDirect;
        let mut lid: String;
        let mut rid: String;
        let mut node: GNode;
        let mut w: u16 = 0;

        // 第一个 node
        (text, vtext) = valid_node_check(line)?;
        let (id, name) = parse_node(vtext)?;
        node = GNode::new(id.to_string(), name.to_string(), linenum, w);
        lid = node.id.clone();
        self.board.add_node(&node);
        loop {
            w += 1;
            if text.len() < 3 {
                break;
            }
            // 再接着 arrow
            (text, vtext) = valid_arrow_check(text)?;
            direct = parse_arrow(vtext);
            if text.len() <= 0 {
                break;
            }
            w += 1;
            // 再接着 node
            (text, vtext) = valid_node_check(text)?;
            let (id, name) = parse_node(vtext)?;
            node = GNode::new(id.to_string(), name.to_string(), linenum, w);
            rid = node.id.clone();
            self.board.add_node(&node);
            self.arrows.push(GArrow::new(
                direct,
                lid,
                rid.clone(),
            ));
            lid = rid;
        }
        Ok(("", ""))
    }

    fn board_rebuild(&mut self) {
        self.board.locate_arrow(&self.arrows);
    }
}
