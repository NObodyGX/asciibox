use std::fmt;

#[derive(Debug)]
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

#[derive(Clone, Debug, Default, Eq, Hash)]
pub struct GNode {
    // 节点 id
    pub id: String,
    // 节点展示内容原始值
    pub name: String,
    // x 轴座标
    pub x: u16,
    // y 轴座标
    pub y: u16,
    // 内容宽度
    pub w: u16,
    // 内容高度
    pub h: u16,
    // 具体每行内容
    words: Vec<String>,
}

impl GNode {
    #[must_use]
    pub fn new(id: String, name: String, x: u16, y: u16) -> Self {
        let nid = id.trim().to_string();
        let pwords: Vec<&str> = name.split('\n').collect();
        let mut words = Vec::new();
        let h: u16 = pwords.len() as u16;
        let mut w: u16 = 0;
        for word in pwords {
            w = std::cmp::max(w, word.len() as u16);
            words.push(word.to_string());
        }

        Self {
            id: nid,
            name: name.trim().to_string(),
            x,
            y,
            w,
            h,
            words,
        }
    }

    pub fn show(&self, i: u16, _maxh: usize, maxw: usize) -> String {
        let lb: usize = (maxw - self.ww() + 1) / 2;
        let rb: usize = maxw - self.ww() - lb;

        if i == 0 {
            // 第一行
            return format!(
                "{}.{}.{}",
                " ".repeat(lb),
                "-".repeat(self.ww()),
                " ".repeat(rb)
            );
        } else if i == self.h + 1 {
            // 最后一行
            return format!(
                "{}'{}'{}",
                " ".repeat(lb),
                "-".repeat(self.ww()),
                " ".repeat(rb)
            );
        } else if i >= self.h + 2 {
            // 超出行
            return format!("{}", " ".repeat(maxw));
        } else {
            // 内容行
            match self.words.get(i as usize - 1) {
                Some(cword) => {
                    let lbank = (self.w as usize + 2 - cword.len() + 1) / 2;
                    let rbank = self.w as usize + 2 - cword.len() - lbank;
                    return format!(
                        "{}|{}{}{}|{}",
                        " ".repeat(lb),
                        " ".repeat(lbank),
                        cword,
                        " ".repeat(rbank),
                        " ".repeat(rb)
                    );
                }
                None => {
                    let ww: usize = self.w as usize + 2;
                    return format!("{}|{}|{}", " ".repeat(lb), " ".repeat(ww), " ".repeat(rb),);
                }
            }
        }
    }

    pub fn ww(&self) -> usize {
        return self.w as usize + 2;
    }

    pub fn hh(&self) -> usize {
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

#[derive(Debug)]
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
