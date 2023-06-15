/* Created by Skylar Huber
 *
 * This file is responsible for rendering images made from text,
 * such as the ones used at the title screen, or the ones used to
 * render big text splashes.
 *
 * All images in `src/render/images` were made by Skylar.
 */

use super::ansi::ANSIColor;
use super::engine::RenderableElement;
use std::collections::HashMap;

/// Represents a text-based-ascii-art-image thing
///
/// Image imports work by calling `Img::from_str(include_str!("path/to/file"))`
/// The image files should be formatted with several likes of text that is the
/// ascii art for the image, then on the first empty line, the program looks for
/// colors, formatted as long lists of RRGGBB color hex codes which match up to
/// the text chars.
pub struct Img {
    /// Lines of text making up the image.
    text: Vec<String>,
    /// 2D Vector of colors for each part of the image.
    color: Vec<Vec<ANSIColor>>,
}

impl RenderableElement for Img {
    // unknown size, minimum is zero tho.
    const H: usize = 0;
    const W: usize = 0;

    fn render(
        &self,
        fb: &mut super::engine::TextFrameBuffer,
        x0: usize,
        y0: usize,
    ) -> super::engine::RenderResult<()> {
        for (y_off, line) in self.text.iter().enumerate() {
            let y = y_off + y0;
            if y >= fb.height() {
                break;
            }
            for (x_off, chr) in line.chars().enumerate() {
                let x = x_off + x0;
                if x >= fb.width() {
                    break;
                }
                fb.char(chr, x, y)?;
                if y_off < self.color.len() && x_off < self.color[y_off].len() {
                    fb.style_fg_box(self.color[y_off][x_off], x, y, 1, 1)?;
                }
            }
        }
        Ok(())
    }
}

impl Img {
    /// Parse the text-image file format.
    pub fn from_str(src: &str) -> Img {
        let mut text = vec![];
        let mut color = vec![];
        let src = src.trim_end();

        let mut colormode = false;
        for line in src.lines() {
            let line = line;
            if line.is_empty() {
                // Switch to capturing color on the first empty newline
                colormode = true;
                continue;
            }
            if colormode {
                // If capturing color, interpret as hex color codes: `RRGGBB`
                let mut building_line = vec![];
                let line: Vec<_> = line.chars().collect();
                for i in 0..line.len() / 6 {
                    let i = i * 6;
                    let v = Vec::<u8>::from_hex(&line[i..i + 6]);
                    building_line.push(ANSIColor::RGB(v[0], v[1], v[2]));
                }
                color.push(building_line);
            } else {
                // If capturing text, simply copy the line over.
                text.push(line.to_string());
            }
        }

        Self { text, color }
    }
    /// Stringifies this image, independent of the renderer.
    pub fn to_raw_string(&self) -> String {
        let mut str: String = String::new();

        for text in &self.text {
            str += format!("{text}\n").as_str();
        }

        str += "\n";

        for color in &self.color {
            for color in color {
                str += &match color {
                    ANSIColor::RGB(r, g, b) => (&[*r, *g, *b][..]).as_hex(),
                    _ => panic!("no"),
                }
            }
            str += "\n";
        }

        str
    }
    pub fn max_width(&self) -> usize {
        self.text
            .iter()
            .map(|v| v.chars().count())
            .max()
            .unwrap_or_default()
    }
    pub fn height(&self) -> usize {
        self.text.len()
    }
}

trait AsHex {
    fn as_hex(&self) -> String;
}
trait FromHex {
    fn from_hex(slice: &[char]) -> Self;
}

impl AsHex for &[u8] {
    fn as_hex(&self) -> String {
        const LUT: &str = "0123456789ABCDEF";
        let lut: Vec<String> = LUT.chars().map(|v| v.to_string()).collect();
        let mut str = String::new();

        for byte in *self {
            str += &lut[(byte >> 4) as usize];
            str += &lut[(byte & 0xf) as usize];
        }

        str
    }
}
impl FromHex for Vec<u8> {
    fn from_hex(slice: &[char]) -> Self {
        const LUT: &str = "0123456789ABCDEF";
        let mut lut: HashMap<char, u8> = HashMap::new();
        for (i, v) in LUT.chars().enumerate() {
            lut.insert(v, i as u8);
        }
        let mut out = vec![];

        for i in 0..slice.len() / 2 {
            let i = i * 2;
            out.push((lut[&slice[i + 0]] << 4) + (lut[&slice[i + 1]]));
        }

        out
    }
}
