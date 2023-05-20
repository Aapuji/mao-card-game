use super::engine::{RenderResult, Screen, TextFrameBuffer};
use crate::game::Game;
use rand::seq::SliceRandom;

#[derive(Debug)]
pub struct TitleScreen {}

impl Screen for TitleScreen {
    fn render_to_buffer(&self, fb: &mut TextFrameBuffer, _game: Option<&Game>) -> RenderResult<()> {
        let g = graphic();
        for (y, line) in g.into_iter().enumerate() {
            if y >= fb.height() {
                break;
            }
            for (x, char) in line.chars().enumerate() {
                if x >= fb.width() {
                    break;
                }
                fb.char(char, x, y)?;
            }
        }
        fb.text_wrapped(splash(), 5, 5, fb.width() - 10)?;
        // fb.style_box(TextStyle::fg_only(TextColor::Red), 10, 10, 12, 6)?;

        Ok(())
    }
}

fn splash() -> &'static str {
    &[
        "The only card game about being confused.",
        "Wikipedia says this game is about Mao Zedong's rule being chaotic.",
        "Gluten free",
        "猫\u{200D}猫\u{200D}猫\u{200D}猫\u{200D} :3c",
        "Forgot the rules? No problem, you weren't supposed to know them anyway.",
        "[Bottom Text]",
    ][..]
        .choose(&mut rand::thread_rng())
        .unwrap()
}

fn graphic() -> Vec<String> {
    //*
    vec![
        r#"                     "#,
        r#"    _  _ ____ ____   "#,
        r#"    |\/| |__| |  |   "#,
        r#"    |  | |  | |__|     The card game™"#,
        r#"⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀"#,
        r#"⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⣀⡀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⡀⡤⠆⠆⡏⠁⠀⠀⠀⠀⠀⠀⠀"#,
        r#"⠀⡀⣀⡄⡤⠤⠤⠆⠖⠋⠏⠋⠍⠁⠟⡆⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⡀⡤⠄⠖⠋⠅⠁⠅⠁⡽⠁⠀⠀⠀⠀⠀⠀⠀⠀"#,
        r#"⡏⠁⠅⠇⡗⡅⠅⠁⠅⠁⠅⠁⠅⠁⠅⠉⠧⡀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠠⠏⡅⡧⠅⠅⠁⠅⠁⠅⢡⠇⠀⠀⠀⠀⠀⠀⠀⠀⠀"#,
        r#"⠉⠧⡅⠁⠇⠇⠅⠁⠅⠁⠅⠁⠅⠁⠅⠁⠅⠷⡄⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠇⡅⠇⠅⠁⠅⠁⠅⠁⠅⠇⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀"#,
        r#"⠀⠀⠻⡇⠅⠁⠅⠁⠅⠁⣷⠧⣅⡁⠅⠁⠅⠁⠟⡇⠀⠀⠀⠀⠀⠀⠀⠀⡏⠁⠅⠁⠅⠁⣥⡅⠅⠁⡏⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀"#,
        r#"⠀⠀⠀⠉⡇⠁⠅⠁⠅⠁⣿⠇⠀⠉⠧⣅⡅⠁⠅⠉⠧⡄⠀⠀⠀⠀⠀⡜⠅⠁⡗⠋⠏⠁⡏⠁⠅⡝⠁⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀"#,
        r#"⠀⠀⠀⠀⠉⠧⡅⠁⠅⠁⠟⠧⣄⠀⠀⣿⠇⠁⠅⠁⠅⠷⡆⠀⠀⠀⡰⠁⠅⢿⠅⠀⡠⠏⠅⠁⡵⠃⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀"#,
        r#"⠀⠀⠀⠀⠀⠀⠳⡅⠅⠁⠅⠁⠍⠉⠧⡿⠅⠁⠅⠁⠅⠁⠍⡇⠀⠠⠇⠁⠅⠉⣧⠎⠅⠁⠅⠡⠇⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀"#,
        r#"⠀⠀⠀⠀⠀⠀⠀⠙⡇⠁⠅⠁⠅⠁⠅⠁⠅⠁⠅⠁⡇⠁⠇⠍⠧⡇⠅⠁⠅⠁⠅⠁⡇⠁⠅⡏⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀"#,
        r#"⠀⠀⠀⠀⠀⠀⠀⠀⠉⢧⡅⠁⠅⠁⠅⠁⠅⠁⠅⠁⠇⠋⠅⠁⡅⡛⡇⠁⠅⠁⠅⡇⠅⠁⡏⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀"#,
        r#"⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠷⡅⠅⠁⡅⣁⡅⡥⠥⠇⠗⠓⠋⡏⠏⠉⠅⠁⠅⠁⠅⡃⡧⠕⠁⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀"#,
        r#"⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠉⠋⠉⠉⠁⠁⠀⠀⠀⠀⠀⡜⠁⡅⡁⡥⠅⠗⠋⠉⠁⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀"#,
        r#"⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠋⠋⠁⠁⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀"#,
        r#"⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀"#,
    ]
    // */
    .into_iter()
    .map(|v| v.to_string())
    .collect()
}
