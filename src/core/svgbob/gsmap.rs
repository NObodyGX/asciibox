use super::node::{GArrow, GDirect, GNode, GSharp};
use super::parse::{parse_arrow, parse_node, valid_arrow_check, valid_node_check};
use nom::IResult;
use std::cmp::max;
use std::ops::Not;

#[derive(Debug, Clone)]
pub struct GBoard {
    // 记录所有 node 信息
    pub nodes: Vec<GNode>,
    pub board: Vec<Vec<usize>>,
    pub w: usize,
    pub h: usize,
    idx: usize,
}

#[derive(Debug, Clone, Default, Copy)]
pub struct RenderBox {
    pub w: usize,
    pub w_left: usize,
    pub w_right: usize,
    pub w_total: usize,
    pub h: usize,
    pub h_up: usize,
    pub h_down: usize,
    pub h_total: usize,
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
        self.h = max(a_node.x + 1, self.h);
        self.w = max(a_node.y + 1, self.w);
        self.nodes.push(a_node);
        true
    }

    pub fn get_node(&mut self, id: &String) -> Option<&GNode> {
        if self.nodes.is_empty() {
            return None;
        }
        for node in self.nodes.iter() {
            if node.id.eq(id) {
                return Some(node);
            }
        }
        return None;
    }

    fn get_node_by_id(&self, idx: &usize) -> Option<&GNode> {
        if self.nodes.is_empty() {
            return None;
        }
        for node in self.nodes.iter() {
            if node.idx.eq(idx) {
                return Some(node);
            }
        }
        return None;
    }

    fn rebuild_borad(&mut self) {
        self.board = Vec::new();
        let h = self.h + 9;
        let w = self.w + 9;
        for _ih in 0..h {
            let mut a: Vec<usize> = Vec::new();
            for _ in 0..w {
                a.push(0);
            }
            self.board.push(a);
        }
    }

    fn add_nodes_into_board(&mut self) {
        for node in self.nodes.iter() {
            let x = node.x;
            let y = node.y;
            match self.board.get_mut(x as usize) {
                Some(v) => {
                    v[y as usize] = node.idx;
                }
                None => {}
            }
        }
    }

    // 将 arrow 加到 node 上
    fn add_arrow_to_node(&mut self, id: &String, arrow: &GArrow, pos: GDirect, render: bool) {
        for node in self.nodes.iter_mut() {
            if node.id.eq(id) {
                node.add_arrow(arrow, pos, render);
                break;
            }
        }
    }

    fn relocate_right(&mut self, id: &String, x: usize, y: usize) {
        for node in self.nodes.iter_mut() {
            if node.id.eq(id) {
                node.x = x;
                node.y = y;
                continue;
            }
            // TODO 所有关联节点都需要被调整
            if node.x == x && node.y >= y {
                node.y += 1;
            }
        }
    }
    fn relocate_down(&mut self, id: &String, x: usize, y: usize) {
        for node in self.nodes.iter_mut() {
            if node.id.eq(id) {
                node.x = x;
                node.y = y;
                continue;
            }
            // TODO 所有关联节点都需要被调整
            if node.x >= x && node.y >= y {
                node.x += 1;
            }
        }
    }

    pub fn load_arrows(&mut self, arrows: &Vec<GArrow>) -> Option<&str> {
        self.rebuild_borad();
        for arrow in arrows {
            let src = &arrow.src;
            let dst = &arrow.dst;
            if src.eq(dst) {
                continue;
            }
            let lnode = self.get_node(src)?;
            let x = lnode.x;
            let y = lnode.y;
            match arrow.direct {
                GDirect::Left | GDirect::Double => {
                    self.relocate_right(&dst, x, max(1, y) - 1);
                    self.add_arrow_to_node(src, arrow, GDirect::Left, true);
                    self.add_arrow_to_node(dst, arrow, GDirect::Left.not(), false);
                }
                GDirect::Right => {
                    self.relocate_right(&dst, x, y + 1);
                    self.add_arrow_to_node(src, arrow, GDirect::Right, true);
                    self.add_arrow_to_node(dst, arrow, GDirect::Right.not(), false);
                }
                GDirect::Up => {
                    self.relocate_down(&dst, max(x, 1) - 1, y);
                    self.add_arrow_to_node(src, arrow, GDirect::Up, true);
                    self.add_arrow_to_node(dst, arrow, GDirect::Up.not(), false);
                }
                GDirect::Down => {
                    self.relocate_down(&dst, x + 1, y);
                    self.add_arrow_to_node(src, arrow, GDirect::Down, true);
                    self.add_arrow_to_node(dst, arrow, GDirect::Down.not(), false);
                }
                _ => {}
            }
        }
        self.add_nodes_into_board();
        Some("")
    }

    pub fn show(&self) -> String {
        let mut rboxes: Vec<RenderBox> = Vec::new();
        for _ in 0..max(self.w + 6, self.h + 6) {
            rboxes.push(RenderBox::default());
        }
        // 先计算显示的长宽
        for node in self.nodes.iter() {
            for (i, cbox) in rboxes.iter_mut().enumerate() {
                if i == node.y as usize {
                    cbox.w = max(cbox.w, node.content_w());
                    cbox.w_left = max(cbox.w_left, node.left_w());
                    cbox.w_right = max(cbox.w_right, node.right_w());
                    cbox.w_total = max(cbox.w_total, node.total_w());
                }
                if i == node.x as usize {
                    cbox.h = max(cbox.w, node.content_h());
                    cbox.h_up = max(cbox.h_up, node.up_h());
                    cbox.h_down = max(cbox.h_down, node.down_h());
                    cbox.h_total = max(cbox.h_total, node.total_h());
                }
            }
        }
        // 开始逐行打印
        let mut content = String::new();
        for (x, items) in self.board.iter().enumerate() {
            let mut linestr: String = String::new();
            if x >= rboxes.len() {
                break;
            }
            let rbox = rboxes.get(x as usize).expect("error");
            let hu = rbox.h_up;
            let hc = rbox.h;
            let maxh = rbox.h_total;
            for h in 0..maxh {
                for (y, idx) in items.iter().enumerate() {
                    if y >= rboxes.len() {
                        break;
                    }
                    let rbox2 = rboxes.get(y as usize).expect("error");
                    let wl = rbox2.w_left;
                    let wr = rbox2.w_right;
                    let wc = rbox2.w;
                    let maxw = rbox2.w_total;
                    if idx.eq(&0) {
                        linestr.push_str(" ".repeat(maxw).as_str());
                        continue;
                    }
                    match self.get_node_by_id(idx) {
                        Some(node) => {
                            let v;
                            if h < hu {
                                v = node.render_up(h, maxh, wc, wl, wr);
                            } else if h < hu + hc {
                                let vv = node.render(h as usize - hu as usize, maxh, wc, wl, wr);
                                v = format!(
                                    "{}{}{}",
                                    " ".repeat(wl - node.left_w()),
                                    vv,
                                    " ".repeat(wr - node.right_w())
                                );
                            } else {
                                v = node.render_down(h - hu - hc, maxh, wc, wl, wr)
                            }
                            linestr.push_str(v.as_str());
                        }
                        None => {
                            linestr.push_str(" ".repeat(maxw).as_str());
                        }
                    }
                }
                linestr.push('\n');
            }
            content.push_str(linestr.trim_end());
            // trim_end 会清除最后的换行
            content.push('\n');
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
        let mut linenum: usize = 0;
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
        self.board.load_arrows(&self.arrows);
        let content = self.board.show();
        content
    }

    // 逐行解析出现的节点，如果有多个节点，这几个节点默认是一排的
    // 后续依据节点之间的联系会重排节点位置
    fn parse_line<'a>(&'a mut self, line: &'a str, linenum: usize) -> IResult<&str, &str> {
        let mut text: &str;
        let mut vtext: &str;
        let mut direct: GDirect;
        let mut lid: String;
        let mut rid: String;
        let mut node: GNode;
        let mut w: usize = 0;

        // 第一个 node
        (text, vtext) = valid_node_check(line)?;
        let (id, name) = parse_node(vtext)?;
        node = GNode::new(id.to_string(), name.to_string(), linenum, w, GSharp::Round);
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
            node = GNode::new(id.to_string(), name.to_string(), linenum, w, GSharp::Round);
            rid = node.id.clone();
            self.board.add_node(&node);
            self.arrows.push(GArrow::new(direct, lid, rid.clone()));
            lid = rid;
        }
        Ok(("", ""))
    }
}
