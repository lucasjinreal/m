use std::default::Default;

use buffer::Mark;

#[derive(Copy, Clone, Debug)]
pub enum Kind {
    Char,
    Line(Anchor),
    Word(Anchor),
    Selection(Anchor),
}

impl Default for Kind {
    fn default() -> Kind {
        Kind::Char
    }
}

#[derive(Copy, Clone, Debug)]
pub enum Anchor {
    Start, // First index within TextObject
    End,   // Last index within TextObject
    Same,  // Same as index within current TextObject of the same Kind
}

impl Default for Anchor {
    fn default() -> Anchor {
        Anchor::Same
    }
}

#[derive(Copy, Clone, Debug)]
pub enum Offset {
    Absolute(usize),
    Backward(usize, Mark),
    Forward(usize, Mark),
}

impl Default for Offset {
    fn default() -> Offset {
        Offset::Forward(0, Mark::Cursor(0))
    }
}

#[derive(Copy, Clone, Debug)]
pub struct TextObject {
    pub kind: Kind,
    pub offset: Offset,
}

impl Default for TextObject {
    fn default() -> TextObject {
        TextObject {
            kind: Default::default(),
            offset: Default::default(),
        }
    }
}
