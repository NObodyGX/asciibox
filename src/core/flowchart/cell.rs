use crate::core::utils::cn_length;
use std::{fmt, ops::Not};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Direct {
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

impl ToString for Direct {
    fn to_string(&self) -> String {
        match self {
            Direct::None => String::from("none"),
            Direct::Double => String::from("double"),
            Direct::Left => String::from("left"),
            Direct::Right => String::from("right"),
            Direct::Up => String::from("up"),
            Direct::Down => String::from("down"),
            Direct::LeftUp => String::from("leftup"),
            Direct::LeftDown => String::from("leftdown"),
            Direct::RightUp => String::from("rightup"),
            Direct::RightDown => String::from("rightdown"),
        }
    }
}

impl Not for Direct {
    type Output = Self;

    fn not(self) -> Self::Output {
        match self {
            Direct::None => Direct::None,
            Direct::Double => Direct::Double,
            Direct::Left => Direct::Right,
            Direct::Right => Direct::Left,
            Direct::Up => Direct::Down,
            Direct::Down => Direct::Up,
            Direct::LeftUp => Direct::RightDown,
            Direct::LeftDown => Direct::RightUp,
            Direct::RightUp => Direct::LeftDown,
            Direct::RightDown => Direct::LeftUp,
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct RBox {
    pub w_left: usize,
    pub w_right: usize,
    pub h_up: usize,
    pub h_down: usize,
    pub left: Direct,
    pub right: Direct,
    pub up: Direct,
    pub down: Direct,
    pub left_down: Direct,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum ASharp {
    Round,  // 圆角
    Square, // 直角
    Circle, // 圆形
}

#[derive(Clone, Debug, Eq, Hash)]
pub struct Cell {
    // 节点 id
    pub id: String,
    // 节点展示内容原始值
    pub name: String,
    // 内容宽度
    pub w: usize,
    // 内容高度
    pub h: usize,
    // 具体每行内容
    words: Vec<String>,
    // 周围可用的箭头
    pub arrows: Vec<Arrow>,
    pub arrows_no_render: Vec<Arrow>,
    // render 用形状
    sharp: ASharp,
}

impl Cell {
    #[must_use]
    pub fn new(id: &str, name: &str) -> Self {
        let nid = String::from(id).trim().to_string();
        let nname: String = name.trim().to_string();
        let pwords: Vec<&str> = nname.split('\n').collect();
        let mut words = Vec::new();
        let h: usize = pwords.len() as usize;
        let mut w: usize = 0;
        for word in pwords {
            w = std::cmp::max(w, cn_length(word) as usize);
            words.push(word.to_string());
        }
        Self {
            id: nid,
            name: nname,
            w,
            h,
            words,
            arrows: Vec::new(),
            arrows_no_render: Vec::new(),
            sharp: ASharp::Round,
        }
    }

    pub fn set_sharp(&mut self, sharp: ASharp) {
        self.sharp = sharp;
    }

    // 绘制，i 行数，maxw 最大宽度(含边框), emode 是否是扩展模式
    pub fn do_render(&self, i: usize, maxw: usize, emode: bool) -> String {
        let cw = maxw - 2;
        let lb: usize = (cw - self.cw() + 1) / 2;
        let rb: usize = cw - self.cw() - lb;

        // 首行或者尾行
        if i == 0 || i == self.h + 1 {
            let spc = if self.sharp == ASharp::Square {
                "+"
            } else {
                if i == 0 {
                    "."
                } else {
                    "'"
                }
            };

            if emode {
                let cstr = "-".repeat(cw);
                return format!("{}{}{}", spc, cstr, spc);
            }
            let lstr = " ".repeat(lb);
            let rstr = " ".repeat(rb);
            let cstr = "-".repeat(self.cw());
            return format!("{}{}{}{}{}", lstr, spc, cstr, spc, rstr);
        }
        // 超出行
        else if i >= self.h + 2 {
            return format!("{}", " ".repeat(maxw));
        }
        // 内容行
        match self.words.get(i - 1) {
            Some(cword) => {
                let lbank = (self.cw() - cn_length(cword) + 1) / 2;
                let rbank = self.cw() - cn_length(cword) - lbank;
                if emode {
                    let lstr = " ".repeat(lb + lbank);
                    let rstr = " ".repeat(rb + rbank);
                    return format!("|{}{}{}|", lstr, cword, rstr);
                }
                let lstr = " ".repeat(lbank);
                let rstr = " ".repeat(rbank);
                return format!(
                    "{}|{}{}{}|{}",
                    " ".repeat(lb),
                    lstr,
                    cword,
                    rstr,
                    " ".repeat(rb)
                );
            }
            None => {
                return format!(
                    "{}|{}|{}",
                    " ".repeat(lb),
                    " ".repeat(self.cw()),
                    " ".repeat(rb)
                );
            }
        }
    }

    pub fn cw(&self) -> usize {
        return self.w + 2;
    }

    pub fn total_w(&self) -> usize {
        return self.cw() + 2;
    }

    pub fn ch(&self) -> usize {
        return self.h;
    }
    pub fn total_h(&self) -> usize {
        return self.ch() + 2;
    }
}

impl fmt::Display for Cell {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "GNode({})", self.id)
    }
}

impl PartialEq for Cell {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id && self.name == other.name
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Arrow {
    pub direct: Direct,
    pub src: String,
    pub dst: String,
    pub text: String,
}

impl Arrow {
    pub fn new(direct: Direct, from: String, to: String, text: String) -> Self {
        Self {
            direct,
            src: from,
            dst: to,
            text,
        }
    }
}

impl fmt::Display for Arrow {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "GArrow({} -{:?}- {})", self.src, self.direct, self.dst)
    }
}
