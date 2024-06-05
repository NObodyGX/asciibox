use crate::core::utils::cn_length;
use std::{fmt, ops::Not};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum GDirect {
    None,
    Double,
    Left,
    Right,
    Up,
    Down,
    LeftUp,
    LeftDown,
    RightUp,
    RightDown,
    // UpLeft,
    // UpRight,
    // DownLeft,
    // DownRight,
}

impl ToString for GDirect {
    fn to_string(&self) -> String {
        match self {
            GDirect::None => String::from("none"),
            GDirect::Double => String::from("double"),
            GDirect::Left => String::from("left"),
            GDirect::Right => String::from("right"),
            GDirect::Up => String::from("up"),
            GDirect::Down => String::from("down"),
            GDirect::LeftUp => String::from("leftup"),
            GDirect::LeftDown => String::from("leftdown"),
            GDirect::RightUp => String::from("rightup"),
            GDirect::RightDown => String::from("rightdown"),
        }
    }
}

impl Not for GDirect {
    type Output = Self;

    fn not(self) -> Self::Output {
        match self {
            GDirect::None => GDirect::None,
            GDirect::Double => GDirect::Double,
            GDirect::Left => GDirect::Right,
            GDirect::Right => GDirect::Left,
            GDirect::Up => GDirect::Down,
            GDirect::Down => GDirect::Up,
            GDirect::LeftUp => GDirect::LeftDown,
            GDirect::LeftDown => GDirect::LeftUp,
            GDirect::RightUp => GDirect::RightDown,
            GDirect::RightDown => GDirect::RightUp,
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct GNBox {
    pub w_left: usize,
    pub w_right: usize,
    pub h_up: usize,
    pub h_down: usize,
    pub left: GDirect,
    pub right: GDirect,
    pub up: GDirect,
    pub down: GDirect,
}

impl GNBox {
    pub fn new() -> Self {
        Self {
            left: GDirect::None,
            right: GDirect::None,
            up: GDirect::None,
            down: GDirect::None,
            w_left: 0,
            w_right: 0,
            h_up: 0,
            h_down: 0,
        }
    }
}

#[derive(Clone, Debug, Eq, Hash)]
pub struct GNode {
    // 节点排序用序号
    pub idx: u16,
    // 节点 id
    pub id: String,
    // 节点展示内容原始值
    pub name: String,
    // 行坐标，依次对应 map 的每一行
    pub x: u16,
    // 列坐标，依次对应每行里面的顺序
    pub y: u16,
    // 内容宽度
    pub w: u16,
    // 内容高度
    pub h: u16,
    // 具体每行内容
    words: Vec<String>,
    // 周围可用的箭头
    pub arrows: Vec<GArrow>,
    pub arrows_no_render: Vec<GArrow>,
    mbox: GNBox,
}

impl GNode {
    #[must_use]
    pub fn new(id: String, name: String, x: u16, y: u16) -> Self {
        let nid: String = id.trim().to_string();
        let nname: String = name.trim().to_string();
        let pwords: Vec<&str> = nname.split('\n').collect();
        let mut words = Vec::new();
        let h: u16 = pwords.len() as u16;
        let mut w: u16 = 0;
        for word in pwords {
            w = std::cmp::max(w, cn_length(word) as u16);
            words.push(word.to_string());
        }
        let mbox = GNBox::new();

        Self {
            id: nid,
            name: nname,
            x,
            y,
            w,
            h,
            words,
            arrows: Vec::new(),
            arrows_no_render: Vec::new(),
            idx: 0,
            mbox,
        }
    }

    /// 向 node 添加 arrow
    /// - arrow: 要添加的 GArrow
    /// - direct: 要添加的方向
    /// - enable_render: 是否需要被绘制
    pub fn add_arrow(&mut self, arrow: &GArrow, direct: GDirect, enable_render: bool) {
        if !enable_render {
            self.arrows_no_render.push(arrow.clone());
            return;
        }
        self.arrows.push(arrow.clone());
        match direct {
            GDirect::Left => {
                self.mbox.left = arrow.direct.clone();
                self.mbox.w_left = std::cmp::max(
                    self.mbox.w_left,
                    if arrow.direct == GDirect::Double {
                        4
                    } else {
                        3
                    },
                );
            }
            GDirect::Right => {
                self.mbox.right = arrow.direct.clone();
                self.mbox.w_right = std::cmp::max(
                    self.mbox.w_right,
                    if arrow.direct == GDirect::Double {
                        4
                    } else {
                        3
                    },
                );
            }
            GDirect::Up => {
                self.mbox.up = arrow.direct.clone();
                self.mbox.h_up = std::cmp::max(self.mbox.h_up, 3);
            }
            GDirect::Down => {
                self.mbox.down = arrow.direct.clone();
                self.mbox.h_down = std::cmp::max(self.mbox.h_down, 3);
            }
            _ => {}
        }
    }

    fn render_arrow(&self, i: u16) -> (String, String) {
        let mut lcontent = String::new();
        let mut rcontent = String::new();

        if self.mbox.w_left > 0 {
            let v = if i != (self.h + 1) / 2 {
                " ".repeat(self.mbox.w_left)
            } else {
                match self.mbox.left {
                    GDirect::Left => {
                        format!("<{}", "-".repeat(self.mbox.w_left - 1))
                    }
                    GDirect::Right => {
                        format!("{}>", "-".repeat(self.mbox.w_left - 1))
                    }
                    GDirect::Double => {
                        format!("<{}>", "-".repeat(self.mbox.w_left - 2))
                    }
                    _ => " ".repeat(self.mbox.w_left),
                }
            };
            lcontent.push_str(v.as_str());
        }

        if self.mbox.w_right > 0 {
            let v = if i != (self.h + 1) / 2 {
                " ".repeat(self.mbox.w_right)
            } else {
                match self.mbox.right {
                    GDirect::Left => {
                        format!("<{}", "-".repeat(self.mbox.w_right - 1))
                    }
                    GDirect::Right => {
                        format!("{}>", "-".repeat(self.mbox.w_right - 1))
                    }
                    GDirect::Double => {
                        format!("<{}>", "-".repeat(self.mbox.w_right - 2))
                    }
                    _ => " ".repeat(self.mbox.w_right),
                }
            };
            rcontent.push_str(v.as_str());
        }
        (lcontent, rcontent)
    }

    pub fn render(&self, i: u16, _maxh: usize, maxw: usize) -> String {
        let lb: usize = (maxw - self.total_w() + 1) / 2;
        let rb: usize = maxw - self.total_w() - lb;

        if i == 0 || i == self.h + 1 {
            let spc = if i == 0 { "." } else { "'" };
            let lstr = " ".repeat(lb + self.mbox.w_left);
            let rstr = " ".repeat(rb + self.mbox.w_right);
            let cstr = "-".repeat(self.centent_w());
            return format!("{}{}{}{}{}", lstr, spc, cstr, spc, rstr);
        } else if i >= self.h + 2 {
            // 超出行
            return format!("{}", " ".repeat(maxw));
        }
        // 内容行
        match self.words.get(i as usize - 1) {
            Some(cword) => {
                let (lastr, rastr) = self.render_arrow(i);
                let lbank = (self.w as usize + 2 - cn_length(cword) + 1) / 2;
                let rbank = self.w as usize + 2 - cn_length(cword) - lbank;
                let lstr = " ".repeat(lbank);
                let rstr = " ".repeat(rbank);
                return format!(
                    "{}{}|{}{}{}|{}{}",
                    " ".repeat(lb),
                    lastr,
                    lstr,
                    cword,
                    rstr,
                    rastr,
                    " ".repeat(rb)
                );
            }
            None => {
                let ww: usize = self.w as usize + 2;
                return format!("{}|{}|{}", " ".repeat(lb), " ".repeat(ww), " ".repeat(rb),);
            }
        }
    }

    pub fn render_up(&self, i: u16, _maxh: usize, maxw: usize) -> String {
        if self.mbox.h_up <= 0 {
            return String::new();
        }
        let lb: usize = (maxw + 1) / 2;
        let rb: usize = maxw - 1 - lb;
        if i == 0 {
            return format!("{}^{}", " ".repeat(lb), " ".repeat(rb),);
        } else if i <= self.mbox.h_up as u16 - 1 {
            return format!("{}|{}", " ".repeat(lb), " ".repeat(rb),);
        }
        return format!("{}", " ".repeat(maxw));
    }

    pub fn render_down(&self, i: u16, _maxh: usize, maxw: usize) -> String {
        if self.mbox.h_down <= 0 {
            return String::new();
        }
        let lb: usize = (maxw + 1) / 2;
        let rb: usize = maxw - 1 - lb;
        if i == self.mbox.h_down as u16 - 1 {
            return format!("{}v{}", " ".repeat(lb), " ".repeat(rb),);
        } else if i < self.mbox.h_down as u16 - 1 {
            return format!("{}|{}", " ".repeat(lb), " ".repeat(rb),);
        }
        return format!("{}", " ".repeat(maxw));
    }

    pub fn centent_w(&self) -> usize {
        return self.w as usize + 2;
    }

    pub fn total_w(&self) -> usize {
        return self.mbox.w_left + self.w as usize + 2 + self.mbox.w_right;
    }

    pub fn total_h(&self) -> usize {
        return self.mbox.h_up + self.h as usize + 2 + self.mbox.h_down;
    }

    pub fn up_h(&self) -> usize {
        return self.mbox.h_up;
    }

    pub fn down_h(&self) -> usize {
        return self.mbox.h_down;
    }

    pub fn content_h(&self) -> usize {
        return self.h as usize + 2;
    }
}

impl fmt::Display for GNode {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "GNode({})", self.id)
    }
}

impl PartialEq for GNode {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id && self.name == other.name
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct GArrow {
    pub direct: GDirect,
    pub src: String,
    pub dst: String,
}

impl GArrow {
    pub fn new(direct: GDirect, from: String, to: String) -> Self {
        Self {
            direct,
            src: from,
            dst: to,
        }
    }
}

impl fmt::Display for GArrow {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "GArrow({} -{:?}- {})", self.src, self.direct, self.dst)
    }
}
