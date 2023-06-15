/* Created by Skylar Huber
 *
 * Rendering engine code. Responsible for providing all the functions needed
 * to draw things on the screen such as rectangles, boxes, and text.
 */

use crate::game::Game;
use crate::render::ansi::{ANSIColor, ANSIStyle, ANSI_STYLE_RESET};
use core::fmt::Debug;
use std::io::{stdin, BufRead};
use term_size;

/// Render over the entire screen.
fn print_framebuffer(fb: TextFrameBuffer) {
    println!("\n\n\n\n\n\n\n{}", fb.to_string());
    // println!("\x1B[2J{}", fb.to_string()); // "\x1B[2J" is clear
}

fn about_to_render<T: Debug>(screen: &T) {
    println!("About to render `{screen:?}`");
}
fn about_to_display<T: Debug>(_screen: &T) {
    println!("About to display");
}

/// Trait representing any full-terminal-covering renderer.
pub trait Screen: Debug {
    /// Takes and subsequently populates a TextFrameBuffer with visual
    /// elements to be printed to the screen.
    fn render_to_buffer(&self, fb: &mut TextFrameBuffer, game: Option<&Game>) -> RenderResult<()>;

    fn render(&self, game: Option<&Game>) -> RenderResult<()> {
        about_to_render(&self);
        let mut fb = TextFrameBuffer::new()?;
        self.render_to_buffer(&mut fb, game)?;
        fb.set_input_prompt("".to_string());
        about_to_display(&self);
        print_framebuffer(fb);
        Ok(())
    }

    /// Like render, but a prompt is shown and stdin is read.
    fn render_then_input(&self, game: Option<&Game>) -> RenderResult<String> {
        about_to_render(&self);
        let mut fb = TextFrameBuffer::new()?;
        self.render_to_buffer(&mut fb, game)?;
        about_to_display(&self);
        print_framebuffer(fb);

        let mut txt = "".to_string();
        stdin()
            .lock()
            .read_line(&mut txt)
            .ok()
            .ok_or(RenderError::InputFailed)?;
        Ok(txt)
    }

    /// Calls render_then_input, and voids the response.
    fn render_then_wait(&self, game: Option<&Game>) -> RenderResult<()> {
        self.render_then_input(game)?;
        Ok(())
    }
}

#[derive(Debug)]
pub enum RenderError {
    TerminalDimensionsBad,
    DrawOutOfBounds((usize, usize), (usize, usize), (usize, usize)),
    InputFailed,
}

impl std::fmt::Display for RenderError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::TerminalDimensionsBad => write!(f, "Terminal dimensions missing or invalid!"),
            Self::DrawOutOfBounds((x, y), (w, h), (maxw, maxh)) => write!(
                f,
                "Drawing out of bounds [pos({x},{y}), dim({w},{h}), allowedDim({maxw},{maxh})]!"
            ),
            Self::InputFailed => write!(f, "Failed to receive stdin input!"),
        }
    }
}
impl std::error::Error for RenderError {}

pub type RenderResult<T> = Result<T, RenderError>;

/// An FrabeBuffer like object which is a 2-dimensional vec of
/// `char` to be drawn to the screen and `TextStyle` to apply
/// at each point
pub struct TextFrameBuffer {
    w: usize,
    h: usize,
    view: Vec<Vec<char>>, // Do you want to maybe make a type here instead? Eg. `type Vec2D<T> = Vec<Vec<T>>`? Then use that? // no i dont
    style_view: Vec<Vec<ANSIStyle>>,
    input_prompt: String,
}

impl TextFrameBuffer {
    pub fn width(&self) -> usize {
        self.w
    }
    pub fn height(&self) -> usize {
        self.h
    }
    /// Creates a new empty `TextFrameBuffer` with the terminal's current dimensions.
    pub fn new() -> RenderResult<Self> {
        let (w, h) = term_size::dimensions().ok_or(RenderError::TerminalDimensionsBad)?;
        let h = h - 3;
        Ok(Self {
            view: vec![vec![' '; h]; w],
            style_view: vec![vec![ANSIStyle::default(); h]; w],
            w,
            h,
            input_prompt: "☵ press enter to continue ☵".to_string(),
        })
    }

    /// Sets the prompt that the user sees when Screen::render_then_input is called.
    pub fn set_input_prompt(&mut self, txt: String) {
        self.input_prompt = txt;
    }

    /// Draw a **single line** of text (`"\n"` will break do not use)
    pub fn text(&mut self, txt: &str, x: usize, y: usize) -> RenderResult<()> {
        self.check_bounds(x, y, txt.len(), 1)?;
        for (i, char) in txt.chars().into_iter().enumerate() {
            self.view[x + i][y] = char;
        }
        Ok(())
    }
    /// Wrapped text
    pub fn text_wrapped(&mut self, txt: &str, x: usize, y: usize, w: usize) -> RenderResult<()> {
        let lines = textwrap::wrap(txt, w);
        self.check_bounds(x, y, w, lines.len())?;
        for (j, line) in lines.into_iter().enumerate() {
            for (i, char) in line.chars().into_iter().enumerate() {
                self.view[x + i][y + j] = char;
            }
        }
        Ok(())
    }
    /// Write a single char to the screen.
    pub fn char(&mut self, char: char, x: usize, y: usize) -> RenderResult<()> {
        self.check_bounds(x, y, 1, 1)?;
        self.view[x][y] = char;
        Ok(())
    }

    /// Set the foreground color in a rectangular area.
    pub fn style_fg_box(
        &mut self,
        color: ANSIColor,
        xs: usize,
        ys: usize,
        w: usize,
        h: usize,
    ) -> RenderResult<()> {
        self.check_bounds(xs, ys, w, h)?;
        for x in xs..xs + w {
            for y in ys..ys + h {
                self.style_view[x][y].set_fg(color);
            }
        }
        Ok(())
    }

    /// Set the background color in a rectangular area.
    pub fn style_bg_box(
        &mut self,
        color: ANSIColor,
        xs: usize,
        ys: usize,
        w: usize,
        h: usize,
    ) -> RenderResult<()> {
        self.check_bounds(xs, ys, w, h)?;
        for x in xs..xs + w {
            for y in ys..ys + h {
                self.style_view[x][y].set_bg(color);
            }
        }
        Ok(())
    }

    /// Clear color in a rectangular area.
    pub fn style_clear_color_box(
        &mut self,
        xs: usize,
        ys: usize,
        w: usize,
        h: usize,
    ) -> RenderResult<()> {
        self.check_bounds(xs, ys, w, h)?;
        for x in xs..xs + w {
            for y in ys..ys + h {
                self.style_view[x][y].clear_color();
            }
        }
        Ok(())
    }

    /// Set or clear bolding in a rectangular area.
    pub fn style_bold_box(
        &mut self,
        is_bold: bool,
        xs: usize,
        ys: usize,
        w: usize,
        h: usize,
    ) -> RenderResult<()> {
        self.check_bounds(xs, ys, w, h)?;
        for x in xs..xs + w {
            for y in ys..ys + h {
                self.style_view[x][y].set_bold(is_bold);
            }
        }
        Ok(())
    }

    /// Set or clear underlining in a rectangular area.
    pub fn style_underline_box(
        &mut self,
        is_bold: bool,
        xs: usize,
        ys: usize,
        w: usize,
        h: usize,
    ) -> RenderResult<()> {
        self.check_bounds(xs, ys, w, h)?;
        for x in xs..xs + w {
            for y in ys..ys + h {
                self.style_view[x][y].set_underline(is_bold);
            }
        }
        Ok(())
    }

    pub fn style_char(&mut self, style: ANSIStyle, x: usize, y: usize) -> RenderResult<()> {
        self.check_bounds(x, y, 1, 1)?;
        self.style_view[x][y] = style;
        Ok(())
    }

    /// Helper function that checks if a box region is safe to draw in without indexing out of bounds.
    fn check_bounds(&self, xs: usize, ys: usize, w: usize, h: usize) -> RenderResult<()> {
        if xs + w > self.w || ys + h > self.h {
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

    /// Uses a `BoxDrawingProfile` to outline a box with [unicode box drawing characters](https://en.wikipedia.org/wiki/Box-drawing_character)
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

/// Represents neat box shaped arrangements of [unicode box drawing characters](https://en.wikipedia.org/wiki/Box-drawing_character)
pub enum BoxDrawingProfile {
    Normal,
}

impl BoxDrawingProfile {
    pub const SHADING: [char; 4] = [' ', '░', '▒', '▓'];

    fn data(&self) -> [[char; 3]; 3] {
        let raw = match self {
            Self::Normal => [
                ['┌', '─', '┐'], // comments
                ['│', ' ', '│'], // preserve
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
    // This is the function that actually turns a `TextFrameBuffer` into a frame.
    fn to_string(&self) -> String {
        let w = self.w;
        let h = self.h;
        dbg!((w, h));
        let mut txt = "".to_string();
        for y in 0..h {
            for x in 0..w {
                let style = self.style_view[x][y];
                let char = self.view[x][y];
                let style_release = if style.has_style() {
                    if x != w - 1 {
                        if style == self.style_view[x + 1][y] {
                            ""
                        } else {
                            ANSI_STYLE_RESET
                        }
                    } else {
                        ANSI_STYLE_RESET
                    }
                } else {
                    ""
                };
                txt += format!("{style}{char}{style_release}",).as_str();
            }
            txt += &"\n";
        }
        txt += self.input_prompt.as_str();
        txt
    }
}

/// A trait representing an object that can be rendered to a `TextFrameBuffer`, such as a card.
pub trait RenderableElement {
    const W: usize;
    const H: usize;
    fn render(&self, fb: &mut TextFrameBuffer, x: usize, y: usize) -> RenderResult<()>;
}
