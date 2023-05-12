use std::fmt::{Debug, Display};

// Credit to Robert on stack overflow
// Gotten Ansi Codes from his reply on stack overflow:
// https://stackoverflow.com/questions/4842424/list-of-ansi-color-escape-sequences

// Additionally, more ansi codes were gotten from LeonDoesCode in a Replit Blog post:
// https://replit.com/talk/learn/ANSI-Escape-Codes-in-Python/22803

pub enum Ansi {
    Reset,
    Bold,
    Underline,
    Black,
    Red,
    Green,
    Yellow,
    Blue,
    Magenta,
    Cyan,
    White,
}

impl Display for Ansi {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::Reset => "\x1b[0m",
                Self::Bold => "\x1b[1m",
                Self::Underline => "\x1b[4m",
                Self::Black => "\x1b[30m",
                Self::Red => "\x1b[31m",
                Self::Green => "\x1b[32m",
                Self::Yellow => "\x1b[33m",
                Self::Blue => "\x1b[34m",
                Self::Magenta => "\x1b[35m",
                Self::Cyan => "\x1b[36m",
                Self::White => "\x1b[37m",
            }
        )
    }
}

impl Debug for Ansi {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self)
    }
}
