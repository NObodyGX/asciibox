use super::cell::{Arrow, Cell, Direct};
use super::maps::RenderBox;
use super::node::{AEdgeCell, ANode};
use std::cmp::{max, min};
use std::collections::HashMap;
use std::ops::Not;

#[derive(Debug, Clone)]
pub struct AGraph {
    pub nodes: HashMap<String, ANode>,
    pub w: usize,
    pub h: usize,

    members: HashMap<String, Cell>,
    edges: Vec<Arrow>,
    limit: usize,
    // 以 (x,y) 的形式来记录相应的 node 位置，用于 render
    canvas: Vec<Vec<String>>,
    edge_canvas: HashMap<String, Vec<AEdgeCell>>,
    emode: bool,
    rboard: HashMap<String, Vec<AEdgeCell>>,
}

impl AGraph {
    pub fn new(limit: usize, emode: bool) -> Self {
        Self {
            members: HashMap::new(),
            edges: Vec::new(),
            nodes: HashMap::new(),
            limit,
            w: 0,
            h: 0,
            canvas: Vec::new(),
            emode,
            edge_canvas: HashMap::new(),
            rboard: HashMap::new(),
        }
    }

    pub fn check_member(&self, id: &String) -> bool {
        if self.members.contains_key(id) {
            return true;
        }
        return false;
    }

    pub fn add_member(&mut self, id: &String, cell: &Cell) {
        if self.members.contains_key(id) {
            return;
        }
        self.members.insert(id.clone(), cell.clone());
    }

    pub fn add_edge(&mut self, edge: &Arrow) {
        if self.edges.contains(edge) {
            return;
        }
        self.edges.push(edge.clone());
    }

    pub fn merge(&mut self, graph: &AGraph) {
        for (id, cell) in graph.members.iter() {
            self.add_member(id, cell);
        }
        for edge in graph.edges.iter() {
            self.add_edge(edge);
        }
    }

    fn is_pos_exist_node(&self, x: usize, y: usize) -> bool {
        for (_id, node) in self.nodes.iter() {
            if node.x == x && node.y == y {
                return true;
            }
        }
        false
    }

    fn node_move(&mut self, id: &String, x: usize, y: usize, level: usize) {
        let node = self.nodes.get_mut(id).unwrap();
        node.x = x;
        node.y = y;
        node.level = level;
        node.locked = true;

        // 修改本 node 对应的位置
        for ec in node.r_edges.iter_mut() {
            ec.ox = x;
            ec.oy = y;
        }
        for ec in node.d_edges.iter_mut() {
            ec.ox = x;
            ec.oy = y;
        }

        // 修改相应的 node 对应的位置
        for (_id, node) in self.nodes.iter_mut() {
            for ec in node.r_edges.iter_mut() {
                if ec.id.eq(id) {
                    ec.x = x;
                    ec.y = y;
                }
            }
            for ec in node.d_edges.iter_mut() {
                if ec.id.eq(id) {
                    ec.x = x;
                    ec.y = y;
                }
            }
        }
    }

    fn try_move(&mut self, id: &String, x: usize, y: usize, level: usize) -> bool {
        if !self.is_pos_exist_node(x, y) {
            self.node_move(id, x, y, level);
            return true;
        }
        false
    }

    // 判断节点位置是否已经固定
    fn is_node_locked(&self, id: &String) -> bool {
        let node = self.nodes.get(id).unwrap();
        node.locked
    }

    // 是否有未固定的节点
    fn is_remain_unlocked(&self) -> bool {
        for (_name, node) in self.nodes.iter() {
            if !node.locked {
                return true;
            }
        }
        false
    }

    fn nodes_down(&mut self) {
        let l = self.nodes.len();
        let mut todos: Vec<String> = Vec::new();
        for (id, node) in self.nodes.clone().iter() {
            if node.y != l {
                todos.push(id.clone());
            }
        }

        for id in todos.iter() {
            let node = self.nodes.get(id).unwrap();
            self.node_move(id, node.x, node.y + 1, node.level);
        }
    }

    fn nodes_right(&mut self) {
        let l = self.nodes.len();
        let mut todos: Vec<String> = Vec::new();
        for (id, node) in self.nodes.iter_mut() {
            if node.x != l {
                todos.push(id.clone());
            }
        }
        for id in todos.iter() {
            let node = self.nodes.get(id).unwrap();
            self.node_move(id, node.x + 1, node.y, node.level);
        }
    }

    // 将 edge 添加到 node 上，为了渲染方便，只保留 right/down 两边的结构

    fn add_edge_node(
        &mut self,
        src: &String,
        dst: &String,
        dir: Direct,
        x: usize,
        y: usize,
        neg: bool,
    ) {
        let flag = match dir {
            Direct::Right | Direct::Left => !neg,
            Direct::Up => neg,
            Direct::Down => !neg,
            _ => false,
        };

        let (si, di) = if flag { (src, dst) } else { (dst, src) };
        let node = self.nodes.get_mut(si).unwrap();
        let edge = AEdgeCell::new(di.clone(), x, y, node.x, node.y, dir.clone());

        match dir {
            Direct::Right | Direct::Left => {
                node.r_edges.push(edge);
            }
            Direct::Up | Direct::Down => {
                node.d_edges.push(edge);
            }
            _ => {}
        }
    }

    // 固定 src 和 dst 的位置
    fn assign_node_seat(&mut self, src: &String, dst: &String, direct: &Direct) {
        let l1 = self.is_node_locked(src);
        let l2 = self.is_node_locked(dst);
        if !l1 && !l2 {
            return;
        }
        let (src, dst, dir, neg) = if l1 {
            (src, dst, direct.clone(), false)
        } else {
            (dst, src, direct.clone().not(), true)
        };

        let x = self.nodes.get(src).unwrap().x;
        let y = self.nodes.get(src).unwrap().y;
        match dir {
            Direct::Left | Direct::Right => {
                if x == 0 && neg {
                    self.nodes_right();
                }
                let nx = if !neg { x + 1 } else { max(x, 1) - 1 };
                for i in 0..self.limit {
                    if !self.try_move(dst, nx, y + i, 1 + i * 2) {
                        continue;
                    }
                    self.add_edge_node(src, dst, dir, nx, y + i, neg);
                    break;
                }
            }
            Direct::Up => {
                // src --^ dst
                if y == 0 && !neg {
                    self.nodes_down();
                }
                let ny = if !neg { max(y, 1) - 1 } else { y + 1 };
                for i in 0..self.limit {
                    if !self.try_move(dst, x + i, ny, 1 + i * 2) {
                        continue;
                    }
                    self.add_edge_node(src, dst, dir, x + i, ny, neg);
                    break;
                }
            }
            Direct::Down => {
                // src --v dst
                if y == 0 && neg {
                    self.nodes_down();
                }
                let ny = if !neg { y + 1 } else { max(y, 1) - 1 };
                for i in 0..self.limit {
                    if !self.try_move(dst, x + i, ny, 1 + i * 2) {
                        continue;
                    }
                    self.add_edge_node(src, dst, dir, x + i, ny, neg);
                    break;
                }
            }
            _ => {}
        }
    }

    fn fit_wh(&mut self) {
        for (_id, node) in self.nodes.iter() {
            self.w = max(self.w, node.x + 1);
            self.h = max(self.h, node.y + 1);
        }
    }

    // 分配所有的节点位置
    pub fn assign_seats(&mut self) {
        let l = self.members.len();
        if l == 1 {
            for (id, cell) in self.members.iter() {
                self.nodes.insert(id.clone(), ANode::new(cell));
            }
            self.fit_wh();
            return;
        }
        // 生成所有节点
        for (id, cell) in self.members.iter() {
            self.nodes.insert(id.clone(), ANode::new(cell));
        }
        // 根据 edge 依次排列节点的位置
        for cnt in 0..self.edges.len() {
            if !self.is_remain_unlocked() {
                break;
            }
            for (i, edge) in self.edges.clone().iter().enumerate() {
                let src = &edge.src;
                let dst = &edge.dst;
                if i == 0 && cnt == 0 {
                    self.node_move(src, 0, 0, 1);
                }
                self.assign_node_seat(src, dst, &edge.direct);
            }
        }
        self.fit_wh();
    }

    fn do_render_down_arrow(&self, i: usize, x: usize, y: usize, rbox: &Vec<RenderBox>) -> String {
        let mut content = String::new();

        let maxh = rbox.get(y).unwrap().down;
        let maxw = rbox.get(x).unwrap().w;
        let cid = self.canvas.get(y).unwrap().get(x).unwrap();

        if cid.is_empty() {
            return " ".repeat(maxw);
        }

        let node = self.nodes.get(cid).unwrap();
        let mut adir = Direct::None;
        let mut is_left = false;
        let mut is_right = false;
        // 判断有几个需要绘制的
        for ec in node.d_edges.iter() {
            if ec.x == x {
                adir = if ec.y > y { Direct::Down } else { Direct::Up };
            }
            if ec.x > x && ec.y != y {
                is_right = true;
            }
            if ec.x < x && ec.y != y {
                is_left = true;
            }
        }
        let lb: usize = maxw / 2;
        let rb: usize = maxw - 1 - lb;

        if !(is_left || is_right) {
            if adir == Direct::None {
                content.push_str(" ".repeat(maxw).as_str());
                return content;
            }
            if i == 0 {
                let seq = if adir == Direct::Up { '^' } else { '|' };
                let a = format!("{}{}{}", " ".repeat(lb), seq, " ".repeat(rb));
                content.push_str(a.as_str());
            } else if i == maxh - 1 {
                let seq = if adir == Direct::Down { 'v' } else { '|' };
                let a = format!("{}{}{}", " ".repeat(lb), seq, " ".repeat(rb));
                content.push_str(a.as_str());
            } else {
                let a = format!("{}|{}", " ".repeat(lb), " ".repeat(rb));
                content.push_str(a.as_str());
            }
        }
        content
    }

    fn do_render_down_arrow_right(
        &self,
        _i: usize,
        x: usize,
        y: usize,
        rbox: &Vec<RenderBox>,
    ) -> String {
        let mut content = String::new();
        let maxw = rbox.get(x).unwrap().right;
        // 注意，由于信息是存放在 (x, y+1) 的右侧，所以判断需要这个格子
        let bid = self.get_bid(x, y + 1);

        let line = match self.rboard.get(&bid) {
            Some(v) => {
                let mut is_over = false;
                let l: usize = (maxw - 1) / 2;
                let r: usize = maxw - l;
                for ec in v.iter() {
                    if ec.y >= y {
                        is_over = true;
                    }
                }

                if is_over {
                    format!("{}|{}", " ".repeat(l), " ".repeat(r - 1))
                } else {
                    " ".repeat(maxw)
                }
            }
            None => " ".repeat(maxw),
        };
        content.push_str(line.as_str());
        content
    }

    fn render_edge_down(&self, y: usize, rbox: &Vec<RenderBox>) -> String {
        let mut content = String::new();
        let maxh = rbox.get(y).unwrap().down;

        for i in 0..maxh {
            let mut line = String::new();
            for x in 0..self.w + 1 {
                line.push_str(self.do_render_down_arrow(i, x, y, rbox).as_str());
                line.push_str(&self.do_render_down_arrow_right(i, x, y, rbox).as_str());
            }
            content.push_str(line.trim_end());
            content.push('\n');
        }

        content
    }

    fn do_render_cell(&self, i: usize, x: usize, y: usize, rbox: &Vec<RenderBox>) -> String {
        let mut content = String::new();
        let maxw = rbox.get(x).unwrap().w;
        let cid = self.canvas.get(y).unwrap().get(x).unwrap();
        if cid.is_empty() {
            content.push_str(" ".repeat(maxw).as_str());
        } else {
            let cell = self.members.get(cid).unwrap();
            content.push_str(cell.do_render(i, maxw, self.emode).trim_end());
        }
        content
    }

    fn inner_render_right_arrow(
        &self,
        i: usize,
        x: usize,
        y: usize,
        rbox: &Vec<RenderBox>,
    ) -> String {
        let mut content = String::new();

        let maxh = rbox.get(y).unwrap().h;
        let maxw = rbox.get(x).unwrap().right;

        let bid = self.get_bid(x, y);
        let line = match self.rboard.get(&bid) {
            Some(v) => {
                let mut adir = Direct::None;
                let mut is_over = false;
                let mut adown = false;
                let l: usize = (maxw - 1) / 2;
                let r: usize = maxw - l;
                for ec in v.iter() {
                    // todo, 需要区分开
                    if ec.y == y {
                        adir = ec.direct.clone();
                        if ec.oy < y {
                            adown = true;
                        } else {
                            adown = false;
                        }
                    }
                    if ec.y > y {
                        is_over = true;
                    }
                    if ec.y < y {
                        is_over = true;
                    }
                }
                if i == maxh / 2 {
                    match adir {
                        Direct::None => {
                            format!("{}|{}", " ".repeat(l), " ".repeat(r - 1))
                        }
                        // 判断是否结束
                        Direct::Left | Direct::Right => {
                            let seq = if adir == Direct::Left { '<' } else { '>' };
                            if is_over {
                                format!("{}+{}{}", " ".repeat(l), "-".repeat(r - 2), seq)
                            } else {
                                format!("{}'{}{}", " ".repeat(l), "-".repeat(r - 2), seq)
                            }
                        }
                        _ => {
                            format!("{}|{}", " ".repeat(l), " ".repeat(r - 1))
                        }
                    }
                } else if i < maxh / 2 {
                    if is_over {
                        format!("{}|{}", " ".repeat(l), " ".repeat(r - 1))
                    } else if adown && adir != Direct::None {
                        format!("{}|{}", " ".repeat(l), " ".repeat(r - 1))
                    } else {
                        " ".repeat(maxw)
                    }
                } else {
                    if is_over {
                        format!("{}|{}", " ".repeat(l), " ".repeat(r - 1))
                    } else if !adown && adir != Direct::None {
                        format!("{}|{}", " ".repeat(l), " ".repeat(r - 1))
                    } else {
                        " ".repeat(maxw)
                    }
                }
            }
            None => " ".repeat(maxw),
        };

        content.push_str(line.as_str());
        return content;
    }

    fn do_render_right_arrow(&self, i: usize, x: usize, y: usize, rbox: &Vec<RenderBox>) -> String {
        // 这里应该和 cell 一样，也是需要找到这个的最大宽度
        let mut content = String::new();

        let maxh = rbox.get(y).unwrap().h;
        let maxw = rbox.get(x).unwrap().right;

        let cid = self.canvas.get(y).unwrap().get(x).unwrap();
        if cid.is_empty() {
            return self.inner_render_right_arrow(i, x, y, rbox);
        }

        let node = self.nodes.get(cid).unwrap();

        let udis = ((maxh - 1) / 2 - 1) / 2;
        let ddis = ((maxh + 1) / 2 + 1) / 2;
        // 判断上节点
        if i == udis {
            // 右侧
            for ec in node.r_edges.iter() {
                if ec.x > x && ec.y < y {
                    content.push_str("-".repeat((maxw + 1) / 2).as_str());
                    content.push('\'');
                    content.push_str(" ".repeat((maxw - 1) / 2).as_str());
                    break;
                }
            }
        }
        // 判断中节点
        else if i == maxh / 2 {
            // 右侧
            for ec in node.r_edges.iter() {
                if ec.x > x && ec.y == y {
                    if ec.direct == Direct::Left {
                        content.push('<');
                        content.push_str("-".repeat(maxw - 1).as_str());
                    } else {
                        content.push_str("-".repeat(maxw - 1).as_str());
                        content.push('>');
                    }
                    break;
                }
            }
        }
        // 判断下节点
        else if i == maxh - ddis {
            // 右侧
            for ec in node.r_edges.iter() {
                if ec.x > x && ec.y > y {
                    content.push_str("-".repeat((maxw - 1) / 2).as_str());
                    content.push('.');
                    content.push_str(" ".repeat((maxw) / 2).as_str());
                    break;
                }
            }
        } else {
            content.push_str(" ".repeat(maxw).as_str());
            return content;
        }
        if content.len() < maxw {
            return self.inner_render_right_arrow(i, x, y, rbox);
        }

        content
    }

    fn render_cell_with_edge(&self, y: usize, rbox: &Vec<RenderBox>) -> String {
        let mut content = String::new();
        let maxh = rbox.get(y).unwrap().h;

        for i in 0..maxh {
            let mut line = String::new();
            for x in 0..self.w + 1 {
                line.push_str(self.do_render_cell(i, x, y, rbox).as_str());
                line.push_str(self.do_render_right_arrow(i, x, y, rbox).as_str());
            }
            content.push_str(line.trim_end());
            content.push('\n');
        }

        content
    }

    fn print_members(&self) {
        log::info!("graph");
        for (name, cell) in self.nodes.iter() {
            log::info!("{}: ({}, {})", name, cell.x, cell.y);
        }
        log::info!("graph end!!!")
    }

    // 绘制本graph
    pub fn render(&self, rbox: &Vec<RenderBox>) -> String {
        self.print_members();
        // 绘制分为两个部分
        // 第一部分：绘制节点的上 edge 及上节点的下 edge
        // 第二部分：绘制节点和节点的左右 edge 部分
        let mut content = String::new();
        for y in 0..self.h + 1 {
            let c_letters = self.render_cell_with_edge(y, &rbox);
            let u_letters = self.render_edge_down(y, &rbox);

            content.push_str(c_letters.trim_end());
            content.push('\n');
            if u_letters.trim_end().len() > 0 {
                content.push_str(u_letters.trim_end());
                content.push('\n');
            }
        }

        content
    }

    pub fn build_canvas(&mut self) {
        // clear canvas
        let w = self.w + 1;
        let h = self.h + 1;
        self.canvas = Vec::with_capacity(h);
        self.edge_canvas = HashMap::new();
        for _ih in 0..h {
            let mut a: Vec<String> = Vec::with_capacity(w);
            for _ in 0..w {
                a.push("".to_string());
            }
            self.canvas.push(a);
        }

        // fill canvas
        for (id, node) in self.nodes.iter() {
            let x = node.x;
            let y = node.y;
            match self.canvas.get_mut(y) {
                Some(v) => {
                    v[x] = id.clone();
                }
                None => {}
            }
        }

        self.rboard = HashMap::new();
        for (_id, node) in self.nodes.iter() {
            for ec in node.r_edges.iter() {
                if !ec.need_record() {
                    continue;
                }
                // 如果是斜着的，暂时额外安排
                if ec.y != node.y && ec.x != node.x {
                    for y in min(ec.y, node.y)..=max(ec.y, node.y) {
                        for x in min(ec.x, node.x)..=max(ec.x, node.x) {
                            if x == node.x && y == node.y {
                                continue;
                            }
                            if x == ec.x && y == ec.y {
                                continue;
                            }
                            if !(x == node.x || y == ec.y) {
                                continue;
                            }
                            let bid = self.get_bid(x, y);
                            match self.rboard.get_mut(&bid) {
                                Some(v) => {
                                    v.push(ec.clone());
                                }
                                None => {
                                    let mut array: Vec<AEdgeCell> = Vec::new();
                                    array.push(ec.clone());
                                    self.rboard.insert(bid, array);
                                }
                            }
                        }
                    }
                }
            }
        }
    }

    fn get_bid(&self, x: usize, y: usize) -> String {
        format!("{}#{}", x, y)
    }
}
