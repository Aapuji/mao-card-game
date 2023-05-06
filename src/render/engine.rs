use term_size;

pub trait Screen {}

impl dyn Screen {}

pub struct TestScreen {}

impl Screen for TestScreen {}

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

pub struct TextFrameBuffer {
    w: usize,
    h: usize,
    view: Vec<Vec<char>>,
}
impl TextFrameBuffer {
    pub fn new() -> RenderResult<Self> {
        let (w, h) = term_size::dimensions().ok_or(RenderError::TerminalDimensionsBad)?;
        println!("DIM: {w}, {h}");
        Ok(Self {
            view: vec![vec![' '; h - 1]; w],
            w,
            h: h - 1,
        })
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
            println!("{y}");
            for x in 0..self.w {
                txt += &self.view[x][y].to_string();
            }
            txt += &"\n";
        }
        txt
    }
}
