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
    UpLeft,
    UpRight,
    DownLeft,
    DownRight,
}

#[derive(Clone, Debug, Default, Eq, Hash)]
pub struct GNode {
    pub id: String,
    pub name: String,
    pub x: u16,
    pub y: u16,
    pub w: u16,
    pub h: u16,
    words: Vec<String>,
}

impl GNode {
    #[must_use]
    pub fn new(id: String, name: String, x:u16, y:u16) -> Self {
        let nid = id.trim().to_string();
        let pwords: Vec<&str> = nid.split('\n').collect();
        let mut words = Vec::new();
        let h: u16 = pwords.len() as u16;
        let mut w: u16 = 0;
        for word in pwords {
            w = std::cmp::max(w, word.len() as u16);
            words.push(word.to_string());
        }
        
        Self {
            id: id.trim().to_string(),
            name: name.trim().to_string(),
            x,
            y,
            w,
            h,
            words
        }
    }

    pub fn show(&self, i:u16) -> String {
        if i == 0  {
            let ww: usize = self.w as usize + 2;
            return format!(".{}.", "-".repeat(ww + 2));
        }
        else if i == self.h - 1 {
            let ww: usize = self.w as usize + 2;
            return format!(".{}.", "-".repeat(ww));
        }
        else {
            match self.words.get(i as usize) {
                Some(cword) => {
                    let lbank = (self.w as usize + 2 - cword.len() + 1) / 2;
                    let rbank = self.w as usize + 2 - cword.len() - lbank;
                    return format!("|{}{}{}|", " ".repeat(lbank), cword, " ".repeat(rbank));
                }
                None => {
                    let ww: usize = self.w as usize + 2;
                    return format!("|{}|", " ".repeat(ww));
                }
            }
        }
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
    pub fn new(direct:GDirect, from: String, to: String) -> Self {
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
