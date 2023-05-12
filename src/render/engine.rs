use crate::game::Game;
use std::fmt::Arguments;
use std::io::{stdin, stdout, BufRead, Write};
use term_size;

pub trait Screen {
    fn render_to_buffer(&self, fb: &mut TextFrameBuffer, game: Option<&Game>) -> RenderResult<()>;

    fn render(&self, game: Option<&Game>) -> RenderResult<()> {
        let mut fb = TextFrameBuffer::new()?;
        self.render_to_buffer(&mut fb, game)?;
        println!("\x1B[2J{}", fb.to_string()); // "\x1B[2J" is clear
        Ok(())
    }

    fn take_input(prompt: Arguments) -> Option<String> {
        stdout().lock().write_fmt(prompt);
        let mut txt = "".to_string();
        stdin().lock().read_line(&mut txt).ok()?;
        Some(txt)
    }
}

#[derive(Debug)]
pub enum RenderError {
    TerminalDimensionsBad,
    DrawOutOfBounds((usize, usize), (usize, usize), (usize, usize)),
}

impl std::fmt::Display for RenderError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::TerminalDimensionsBad => write!(f, "Terminal dimensions missing or invalid!"),
            Self::DrawOutOfBounds((x, y), (w, h), (maxw, maxh)) => write!(
                f,
                "Drawing out of bounds [pos({x},{y}), dim({w},{h}), allowedDim({maxw},{maxh})]!"
            ),
        }
    }
}
impl std::error::Error for RenderError {}

pub type RenderResult<T> = Result<T, RenderError>;

const ANSI_ESCAPE: &str = "\x1B";
const ANSI_STYLE_RESET: &str = "\x1B[0m";

#[derive(Clone, Copy)]
pub enum TextColor {
    None,
    Red,
  // White,
  // Black,
}
impl TextColor {
    fn ansi_fg_id(&self) -> String {
        match self {
            Self::None => "",
            Self::Red => "31",
        }
        .to_string()
    }
    fn ansi_bg_id(&self) -> String {
        match self {
            Self::None => "",
            Self::Red => "41",
        }
        .to_string()
    }
}
#[derive(Clone, Copy)]
pub struct TextStyle {
    pub fg: TextColor,
    pub bg: TextColor,
}
impl TextStyle {
    pub fn fg_only(fg: TextColor) -> Self {
      Self { fg, bg: TextColor::None }
    }
    pub fn bg_only(bg: TextColor) -> Self {
      Self { fg: TextColor::None, bg }
    }
}
impl Default for TextStyle {
    fn default() -> Self {
        Self {
            fg: TextColor::None,
            bg: TextColor::None,
        }
    }
}
impl std::fmt::Display for TextStyle {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(// note: Replit will ignore the first one for whatever reason, TODO: solution that takes advantage of this
            f,
            "{ANSI_ESCAPE}[{}m{ANSI_ESCAPE}[{}m",
            self.bg.ansi_bg_id(),
            self.fg.ansi_fg_id(),
        )
    }
}

pub struct TextFrameBuffer {
    w: usize,
    h: usize,
    view: Vec<Vec<char>>, // Do you want to maybe make a type here instead? Eg. `type Vec2D<T> = Vec<Vec<T>>`? Then use that?
    style_view: Vec<Vec<TextStyle>>,
}

impl TextFrameBuffer {
    pub fn new() -> RenderResult<Self> {
        let (w, h) = term_size::dimensions().ok_or(RenderError::TerminalDimensionsBad)?;
        let h = h - 2;
        Ok(Self {
            view: vec![vec![' '; h]; w],
            style_view: vec![vec![TextStyle::default(); h]; w],
            w,
            h,
        })
    }

    pub fn text(&mut self, txt: &str, x: usize, y: usize) -> RenderResult<()> {
        self.check_bounds(x, y, txt.len(), 1)?;
        for (i, char) in txt.chars().into_iter().enumerate() {
            self.view[x + i][y] = char;
        }
        Ok(())
    }

    pub fn style_box(
        &mut self,
        style: TextStyle,
        xs: usize,
        ys: usize,
        w: usize,
        h: usize,
    ) -> RenderResult<()> {
        self.check_bounds(xs, ys, w, h)?;
        for x in xs..xs + w {
            for y in ys..ys + h {
                self.style_view[x][y] = style;
            }
        }
        Ok(())
    }

    fn check_bounds(&self, xs: usize, ys: usize, w: usize, h: usize) -> RenderResult<()> {
        if xs + w >= self.w || ys + h >= self.h {
            Err(RenderError::DrawOutOfBounds(
                (xs, ys),
                (w, h),
                (self.w, self.h),
            ))
        } else {
            Ok(())
        }
    }

    pub fn fill_box(
        &mut self,
        value: char,
        xs: usize,
        ys: usize,
        w: usize,
        h: usize,
    ) -> RenderResult<()> {
        self.check_bounds(xs, ys, w, h)?;
        for x in xs..xs + w {
            for y in ys..ys + h {
                self.view[x][y] = value;
            }
        }
        Ok(())
    }

    pub fn outline_box(
        &mut self,
        profile: BoxDrawingProfile,
        xs: usize,
        ys: usize,
        w: usize,
        h: usize,
    ) -> RenderResult<()> {
        self.check_bounds(xs, ys, w, h)?;

        let xe = xs + w - 1;
        let ye = ys + h - 1;

        let profile = profile.data();
        for x in xs + 1..xs + w - 1 {
            self.view[x][ys] = profile[1][0]; // top
            self.view[x][ye] = profile[1][2]; // bottom
        }
        for y in ys + 1..ys + h - 1 {
            self.view[xs][y] = profile[0][1]; // left
            self.view[xe][y] = profile[2][1]; // right
        }
        self.view[xs][ys] = profile[0][0]; // top left
        self.view[xe][ys] = profile[2][0]; // top right
        self.view[xs][ye] = profile[0][2]; // bottom left
        self.view[xe][ye] = profile[2][2]; // bottom right
        Ok(())
    }
}

pub enum BoxDrawingProfile {
    Normal,
}

impl BoxDrawingProfile {
    pub const SHADING: [char; 4] = [' ', '░', '▒', '▓'];

    fn data(&self) -> [[char; 3]; 3] {
        let raw = match self {
            Self::Normal => [
                ['┌', '─', '┐'], // comments
                ['│', ' ', '│'],   // preserve
                ['└', '─', '┘'], // formatting
            ],
        };

        let mut formatted = [[' '; 3]; 3];

        for x in 0..3 {
            for y in 0..3 {
                formatted[x][y] = raw[y][x];
            }
        }

        formatted
    }
}

impl std::string::ToString for TextFrameBuffer {
    fn to_string(&self) -> String {
        println!("selfdim: {},{}", self.w, self.h);
        let mut txt = "".to_string();
        for y in 0..self.h {
            for x in 0..self.w {
                txt += format!(
                    "{}{}{}",
                    self.style_view[x][y], self.view[x][y], ANSI_STYLE_RESET
                )
                .as_str();
            }
            txt += &"\n";
        }
        txt
    }
}

pub trait RenderableElement {
    const W: usize;
    const H: usize;
    fn render_size(&self) -> (usize, usize) {
        (Self::W, Self::H)
    }
    fn render(&self, fb: &mut TextFrameBuffer, x: usize, y: usize) -> RenderResult<()>;
}
