use super::node::{GArrow, GDirect, GNode, GSharp};
use super::parse::{parse_arrow, parse_node, valid_arrow_check, valid_node_check};
use nom::IResult;
use std::cmp::max;
use std::collections::HashMap;
use std::ops::Not;

#[derive(Debug, Clone)]
pub struct GSMap {
    // 记录所有 node 信息
    nodes: HashMap<String, GNode>,
    // 记录所有 id --> node.id 信息
    node_ids: HashMap<usize, String>,
    // 记录所有 arrow 信息
    arrows: Vec<GArrow>,
    // 以 grid 的形式来记录相应的 node 位置，用于 render
    canvas: Vec<Vec<usize>>,
    // max w
    w: usize,
    // max h
    h: usize,
    // node 的序号生成起始值
    idx: usize,
    // 是否扩展 box 保证相同
    expand_mode: bool,
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

impl GSMap {
    pub fn new(expand_mode: bool) -> Self {
        Self {
            nodes: HashMap::new(),
            node_ids: HashMap::new(),
            arrows: Vec::new(),
            canvas: Vec::new(),
            w: 0,
            h: 0,
            idx: 1,
            expand_mode,
        }
    }

    pub fn load_content(&mut self, content: &str) -> String {
        self.clear();
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
        self.load_arrows();
        let content = self.show();
        content
    }

    fn clear(&mut self) {
        self.arrows = Vec::new();
        self.nodes = HashMap::new();
        self.node_ids = HashMap::new();
        self.w = 0;
        self.h = 0;
        self.idx = 1;
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
        self.add_node(&node);
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
            self.add_node(&node);
            self.arrows.push(GArrow::new(direct, lid, rid.clone()));
            lid = rid;
        }
        Ok(("", ""))
    }

    pub fn add_node(&mut self, node: &GNode) -> bool {
        if self.nodes.contains_key(&node.id) {
            return false;
        }
        let mut n_node = node.clone();
        n_node.idx = self.idx;
        self.idx += 1;
        self.h = max(n_node.x + 1, self.h);
        self.w = max(n_node.y + 1, self.w);
        self.node_ids.insert(n_node.idx.clone(), n_node.id.clone());
        self.nodes.insert(n_node.id.clone(), n_node);
        true
    }

    fn get_node_by_index(&self, idx: &usize) -> &GNode {
        let id = self.node_ids.get(idx).unwrap();
        self.nodes.get(id).unwrap()
    }

    fn clear_canvas(&mut self) {
        self.canvas = Vec::new();
        let h = self.h + 9;
        let w = self.w + 9;
        for _ih in 0..h {
            let mut a: Vec<usize> = Vec::new();
            for _ in 0..w {
                a.push(0);
            }
            self.canvas.push(a);
        }
    }

    // 将所有的 nodes 加入到里
    fn add_nodes_into_board(&mut self) {
        for (_id, node) in self.nodes.iter() {
            let x = node.x;
            let y = node.y;
            match self.canvas.get_mut(x as usize) {
                Some(v) => {
                    v[y as usize] = node.idx;
                }
                None => {}
            }
        }
    }

    // 将 arrow 加到 node 上
    fn add_arrow_to_node(&mut self, id: &String, arrow: &GArrow, pos: GDirect, render: bool) {
        let node = self.nodes.get_mut(id).unwrap();
        node.add_arrow(arrow, pos, render);
    }

    fn get_node_relationship(&self, id: &String) -> Vec<String> {
        let mut result: Vec<String> = Vec::new();
        let node = self.nodes.get(id).unwrap();
        for arrow in node.arrows.iter() {
            if !result.contains(&arrow.src) {
                result.push(arrow.src.clone());
            }
            if !result.contains(&arrow.dst) {
                result.push(arrow.dst.clone());
            }
        }
        for arrow in node.arrows_no_render.iter() {
            if !result.contains(&arrow.src) {
                result.push(arrow.src.clone());
            }
            if !result.contains(&arrow.dst) {
                result.push(arrow.dst.clone());
            }
        }
        result
    }

    fn move_node_to(&mut self, id: &String, x: usize, y: usize) {
        let node = self.nodes.get_mut(id).unwrap();
        node.x = x;
        node.y = y;
    }

    fn move_node(&mut self, id: &String, offx: usize, offy: usize) {
        let node = self.nodes.get_mut(id).unwrap();
        node.x += offx;
        node.y += offy;
    }

    fn search_node_relationship(&self, id: &String) -> Vec<String> {
        let mut done_vec: Vec<String> = Vec::new();
        let mut todo_vec: Vec<String> = Vec::new();
        done_vec.push(id.clone());
        todo_vec.push(id.clone());
        while todo_vec.len() > 0 {
            let nid = todo_vec.pop().unwrap();
            let rnodes = self.get_node_relationship(&nid);
            for rid in rnodes.iter() {
                if !done_vec.contains(rid) {
                    done_vec.push(rid.clone());
                    todo_vec.push(rid.clone());
                }
            }
        }
        done_vec
    }

    fn relocate_right(&mut self, id: &String, x: usize, y: usize) {
        let mut moved_ids: Vec<String> = Vec::new();
        self.move_node_to(id, x, y);
        moved_ids.push(id.clone());
        let rids: Vec<String> = self.search_node_relationship(id);
        // 暂时不调整自身的关联节点
        // 调整所有的关联节点
        // 1. 调整所有在当前行插入节点所在位置右侧的节点
        // 2. 调整 1 中所有节点上下对应的节点
        // 3. 调整 2 中所有上下对应节点的左右节点
        for (_id, node) in self.nodes.iter_mut() {
            if moved_ids.contains(&node.id) {
                continue;
            }
            // 这里调整的节点可能和 id 没有联系
            if node.x == x && node.y >= y {
                node.y += 1;
                moved_ids.push(node.id.clone());
            }
        }
        for nid in rids.iter() {
            if moved_ids.contains(nid) {
                continue;
            }
            self.move_node(nid, 0, 1);
        }
    }

    fn relocate_down(&mut self, id: &String, x: usize, y: usize) {
        let mut moved_ids: Vec<String> = Vec::new();
        self.move_node_to(id, x, y);
        moved_ids.push(id.clone());
        let rids: Vec<String> = self.search_node_relationship(id);
        for (_id, node) in self.nodes.iter_mut() {
            if moved_ids.contains(&node.id) {
                continue;
            }
            // 这里调整的节点可能和 id 没有联系
            if node.x >= x && node.y == y {
                node.y += 1;
                moved_ids.push(node.id.clone());
            }
        }
        for nid in rids.iter() {
            if moved_ids.contains(nid) {
                continue;
            }
            self.move_node(nid, 1, 0);
        }
    }

    fn move_nodes_up(&mut self) {
        for (_id, node) in self.nodes.iter_mut() {
            node.x += 1;
        }
    }

    fn check_floating(&self, src: &String, dst: &String) -> (usize, usize) {
        let mut lindex = 0;
        let mut rindex = 0;
        for (_id, node) in self.nodes.iter() {
            if node.id.eq(src) {
                lindex = node.floating;
            }
            if node.id.eq(dst) {
                rindex = node.floating;
            }
        }
        (lindex, rindex)
    }

    fn set_node_floating(&mut self, id: &String) {
        let node = self.nodes.get_mut(id).unwrap();
        node.floating = 1;
    }

    pub fn load_arrows(&mut self) {
        self.clear_canvas();
        for arrow in self.arrows.clone().iter() {
            let mut src = &arrow.src;
            let mut dst = &arrow.dst;
            if src.eq(dst) {
                continue;
            }
            let mut rev: bool = false;
            match self.check_floating(src, dst) {
                (0, 0) => {
                    self.set_node_floating(src);
                    self.set_node_floating(dst);
                }
                (1, 0) => {
                    self.set_node_floating(dst);
                }
                (0, 1) => {
                    let tmp = src;
                    src = dst;
                    dst = tmp;
                    rev = true;
                    self.set_node_floating(dst);
                }
                (_, _) => {
                    // TODO
                    return;
                }
            }
            let ndirect = if !rev {
                arrow.direct.clone()
            } else {
                arrow.direct.clone().not()
            };
            let node = self.nodes.get(src).unwrap();
            let x = node.x;
            let y = node.y;
            match ndirect {
                GDirect::Left => {
                    // src <-- dst
                    self.relocate_right(&dst, x, y + 1);
                    self.add_arrow_to_node(src, arrow, GDirect::Right, true);
                    self.add_arrow_to_node(dst, arrow, GDirect::Right.not(), false);
                }
                GDirect::Right | GDirect::Double => {
                    // src --> dst
                    self.relocate_right(&dst, x, y + 1);
                    self.add_arrow_to_node(src, arrow, GDirect::Right, true);
                    self.add_arrow_to_node(dst, arrow, GDirect::Right.not(), false);
                }
                GDirect::Up => {
                    // src --^ dst
                    if x == 0 {
                        self.move_nodes_up();
                        self.move_node_to(&dst, x, y);
                    } else {
                        self.relocate_down(&dst, x - 1, y);
                    }
                    self.add_arrow_to_node(src, arrow, GDirect::Up, true);
                    self.add_arrow_to_node(dst, arrow, GDirect::Up.not(), false);
                }
                GDirect::Down => {
                    // src --v dst
                    self.relocate_down(&dst, x + 1, y);
                    self.add_arrow_to_node(src, arrow, GDirect::Down, true);
                    self.add_arrow_to_node(dst, arrow, GDirect::Down.not(), false);
                }
                GDirect::LeftDown => {
                    self.relocate_down(&dst, x + 1, max(1, y) - 1);
                    self.add_arrow_to_node(src, arrow, GDirect::LeftDown, true);
                    self.add_arrow_to_node(dst, arrow, GDirect::LeftDown.not(), false);
                }
                _ => {}
            }
        }
        self.add_nodes_into_board();
    }

    fn debug_show_position(&self) {
        for (id, node) in self.nodes.iter() {
            println!("{}: ({}, {})", id, node.x, node.y);
        }
    }

    fn show(&self) -> String {
        self.debug_show_position();
        let mut rboxes: Vec<RenderBox> = Vec::new();
        for _ in 0..max(self.w + 6, self.h + 6) {
            rboxes.push(RenderBox::default());
        }
        let mut rw: usize = 0;
        let mut rh: usize = 0;
        // 先计算显示的长宽
        for (_id, node) in self.nodes.iter() {
            rw = max(rw, node.y + 1);
            rh = max(rh, node.x + 1);
            for (i, cbox) in rboxes.iter_mut().enumerate() {
                if i == node.y as usize {
                    cbox.w = max(cbox.w, node.content_w());
                    cbox.w_left = max(cbox.w_left, node.left_w());
                    cbox.w_right = max(cbox.w_right, node.right_w());
                }
                if i == node.x as usize {
                    cbox.h = max(cbox.h, node.content_h());
                    cbox.h_up = max(cbox.h_up, node.up_h());
                    cbox.h_down = max(cbox.h_down, node.down_h());
                    cbox.h_total = max(cbox.h_total, node.total_h());
                }
            }
        }
        // 开始逐行打印
        let mut content = String::new();
        for (x, items) in self.canvas.iter().enumerate() {
            let mut linestr: String = String::new();
            if x > rh {
                break;
            }
            let rbox = rboxes.get(x as usize).expect("error");
            let hu = rbox.h_up;
            let hc = rbox.h;
            let maxh = rbox.h_total;
            // 每行里按高度逐行计算
            for h in 0..maxh {
                // 开始逐列取 node 开始渲染
                for (y, idx) in items.iter().enumerate() {
                    if y >= rboxes.len() || y > rh {
                        break;
                    }
                    let rbox2 = rboxes.get(y as usize).expect("error");
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
}
