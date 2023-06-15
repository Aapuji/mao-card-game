/* Created by Skylar Huber
 * 
 * Manages color formatting for the render engine, which can be drawn to the scren.
 */
use std::fmt::{Debug, Display};

/// Escape symbol for terminal style and color: https://en.wikipedia.org/wiki/ANSI_escape_code
const ANSI_ESCAPE: &str = "\x1B";
pub const ANSI_STYLE_RESET: &str = "\x1B[0m";

/// Represents a combined style for a single `char` in the renerer.
///
/// Contains:
/// - color (either foreground, background, or neither)
/// - if it's underlined
/// - if it's bold
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct ANSIStyle {
    color: ANSIColor,
    is_fg: bool,
    is_bg: bool,
    is_underline: bool,
    is_bold: bool,
}
impl ANSIStyle {
    pub fn fg(color: ANSIColor) -> Self {
        Self {
            color,
            is_fg: true,
            is_bg: false,
            is_underline: false,
            is_bold: false,
        }
    }
    pub fn bg(color: ANSIColor) -> Self {
        Self {
            color,
            is_fg: true,
            is_bg: false,
            is_underline: false,
            is_bold: false,
        }
    }

    pub fn set_underline(&mut self, is_underline: bool) {
        self.is_underline = is_underline;
    }
    pub fn set_bold(&mut self, is_bold: bool) {
        self.is_bold = is_bold;
    }
    pub fn set_fg(&mut self, color: ANSIColor) {
        self.is_fg = true;
        self.is_bg = false;
        self.color = color;
    }
    pub fn set_bg(&mut self, color: ANSIColor) {
        self.is_fg = false;
        self.is_bg = true;
        self.color = color;
    }
    pub fn clear_color(&mut self) {
        self.is_fg = false;
        self.is_bg = false;
    }
    pub fn clear(&mut self) {
        self.clear_color();
        self.set_bold(false);
        self.set_underline(false);
    }
    pub fn has_style(&self) -> bool {
        self.is_fg || self.is_bg || self.is_bold || self.is_underline
    }
}
impl Default for ANSIStyle {
    fn default() -> Self {
        Self {
            color: ANSIColor::Default,
            is_fg: false,
            is_bg: false,
            is_underline: false,
            is_bold: false,
        }
    }
}
impl std::fmt::Display for ANSIStyle {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.is_fg {
            write!(f, "{}", self.color.to_string(true))?;
        }
        if self.is_bg {
            write!(f, "{}", self.color.to_string(false))?;
        }
        if self.is_bold {
            write!(f, "{}", ANSIModifier::Bold)?;
        }
        if self.is_underline {
            write!(f, "{}", ANSIModifier::Underline)?;
        }
        Ok(())
    }
}

/// Represents all different kinds of text styles that the console can render.
#[derive(Debug)]
pub enum ANSIModifier {
    Reset,
    Bold,
    Underline,
}

impl Display for ANSIModifier {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let id = match self {
            Self::Reset => "0",
            Self::Bold => "1",
            Self::Underline => "4",
        };
        write!(f, "{ANSI_ESCAPE}[{id}m")
    }
}

/// Represents all different kinds of colors that the console can render.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ANSIColor {
    RGB(u8, u8, u8),
    Default,
    Black,
    LightBlack, // I shit you not that is what it's called in the standard
    Red,
    LightRed,
    Green,
    LightGreen,
    Yellow,
    LightYellow,
    Blue,
    LightBlue,
    Magenta,
    LightMagenta,
    Cyan,
    LightCyan,
    White,
    LightWhite,
}

impl ANSIColor {
    pub fn fg(&self) -> String {
        self.to_string(true)
    }
    pub fn bg(&self) -> String {
        self.to_string(false)
    }

    /// Gets the full ANSI code for this color.
    fn to_string(&self, is_fg: bool) -> String {
        let id = match (is_fg, self) {
            (_, Self::RGB(r, g, b)) => {
                // FG: ESC[38;2;⟨r⟩;⟨g⟩;⟨b⟩m
                // BG: ESC[48;2;⟨r⟩;⟨g⟩;⟨b⟩m
                format!(
                    "{};2;{};{};{}",
                    if is_fg { "38" } else { "48" },
                    r.to_string(),
                    g.to_string(),
                    b.to_string()
                )
            }

            (true, Self::Default) => "39".to_string(),
            (false, Self::Default) => "49".to_string(),

            (true, Self::Black) => "30".to_string(),
            (false, Self::Black) => "40".to_string(),
            (true, Self::LightBlack) => "90".to_string(),
            (false, Self::LightBlack) => "100".to_string(),

            (true, Self::Red) => "31".to_string(),
            (false, Self::Red) => "41".to_string(),
            (true, Self::LightRed) => "91".to_string(),
            (false, Self::LightRed) => "101".to_string(),

            (true, Self::Green) => "32".to_string(),
            (false, Self::Green) => "42".to_string(),
            (true, Self::LightGreen) => "92".to_string(),
            (false, Self::LightGreen) => "102".to_string(),

            (true, Self::Yellow) => "33".to_string(),
            (false, Self::Yellow) => "43".to_string(),
            (true, Self::LightYellow) => "93".to_string(),
            (false, Self::LightYellow) => "103".to_string(),

            (true, Self::Blue) => "34".to_string(),
            (false, Self::Blue) => "44".to_string(),
            (true, Self::LightBlue) => "94".to_string(),
            (false, Self::LightBlue) => "104".to_string(),

            (true, Self::Magenta) => "35".to_string(),
            (false, Self::Magenta) => "45".to_string(),
            (true, Self::LightMagenta) => "95".to_string(),
            (false, Self::LightMagenta) => "105".to_string(),

            (true, Self::Cyan) => "36".to_string(),
            (false, Self::Cyan) => "46".to_string(),
            (true, Self::LightCyan) => "96".to_string(),
            (false, Self::LightCyan) => "106".to_string(),

            (true, Self::White) => "37".to_string(),
            (false, Self::White) => "47".to_string(),
            (true, Self::LightWhite) => "97".to_string(),
            (false, Self::LightWhite) => "107".to_string(),
        };
        format!("{ANSI_ESCAPE}[{id}m")
    }
}
