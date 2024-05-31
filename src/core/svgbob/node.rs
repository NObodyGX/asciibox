use std::fmt;

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

#[derive(Clone, Debug, Default, Eq, PartialEq, Hash)]
pub struct GNBox {
    pub l_arrow_w: usize,
    pub r_arrow_w: usize,
}

impl GNBox {
    pub fn new() -> Self {
        Self {
            l_arrow_w: 0,
            r_arrow_w: 0,
        }
    }
}

#[derive(Clone, Debug, Default, Eq, Hash)]
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
            w = std::cmp::max(w, word.len() as u16);
            words.push(word.to_string());
        }

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
            mbox: GNBox::new(),
        }
    }

    pub fn add_arrow(&mut self, arrow: &GArrow, enable_render: bool) {
        if !enable_render {
            self.arrows_no_render.push(arrow.clone());
            return;
        }
        self.arrows.push(arrow.clone());
        match arrow.direct {
            GDirect::Left => {
                self.mbox.l_arrow_w = 3;
            }
            GDirect::Right => {
                self.mbox.r_arrow_w = 3;
            }
            _ => {}
        }
    }

    fn render_arrow(&self, i: u16) -> (String, String) {
        let mut lcontent = String::new();
        let mut rcontent = String::new();
        if self.mbox.l_arrow_w > 0 {
            if i == (self.h + 1) / 2 {
                let v = format!("<{}", "-".repeat(self.mbox.l_arrow_w - 1));
                lcontent.push_str(v.as_str());
            } else {
                lcontent.push_str(" ".repeat(self.mbox.l_arrow_w).as_str());
            }
        }
        if self.mbox.r_arrow_w > 0 {
            if i == (self.h + 1) / 2 {
                let v = format!("{}>", "-".repeat(self.mbox.r_arrow_w - 1));
                rcontent.push_str(v.as_str());
            } else {
                rcontent.push_str(" ".repeat(self.mbox.r_arrow_w).as_str());
            }
        }
        (lcontent, rcontent)
    }

    pub fn render(&self, i: u16, _maxh: usize, maxw: usize) -> String {
        let lb: usize = (maxw - self.total_w() + 1) / 2;
        let rb: usize = maxw - self.total_w() - lb;

        if i == 0 || i == self.h + 1 {
            let spc = if i == 0 { "." } else { "'" };
            let lstr = " ".repeat(lb + self.mbox.l_arrow_w);
            let rstr = " ".repeat(rb + self.mbox.r_arrow_w);
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
                let lbank = (self.w as usize + 2 - cword.len() + 1) / 2;
                let rbank = self.w as usize + 2 - cword.len() - lbank;
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

    pub fn centent_w(&self) -> usize {
        return self.w as usize + 2;
    }

    pub fn total_w(&self) -> usize {
        return self.mbox.l_arrow_w + self.w as usize + 2 + self.mbox.r_arrow_w;
    }

    pub fn total_h(&self) -> usize {
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
