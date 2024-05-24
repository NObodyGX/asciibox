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
    pub x: i16,
    pub y: i16,
    pub w: u16,
    pub h: u16,
}

impl GNode {
    #[must_use]
    pub fn new(id: String, name: String, x:i16, y:i16) -> Self {
        Self {
            id: id.trim().to_string(),
            name: name.trim().to_string(),
            x,
            y,
            w: 0,
            h: 0,
        }
    }

    pub fn walk(&mut self, x_off:i16, y_off:i16) {
        self.x += x_off;
        self.y += y_off;
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
