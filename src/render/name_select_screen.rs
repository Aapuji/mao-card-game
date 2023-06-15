/* Created by Skylar Huber
 * 
 * This file is responsible for rendering the screen that
 * asks users for their names.
 */

use super::engine::{RenderResult, Screen, TextFrameBuffer};

#[derive(Debug)]
pub struct NameSetScreen<'a> {
    names: &'a [String],
}

impl<'a> Screen for NameSetScreen<'a> {
    fn render_to_buffer(
        &self,
        fb: &mut TextFrameBuffer,
        _: Option<&crate::game::Game>,
    ) -> RenderResult<()> {
        for (i, ln) in [
            r#"____ _  _ ___ ____ ____    _  _ ____ _  _ ____ ____ "#,
            r#"|___ |\ |  |  |___ |__/    |\ | |__| |\/| |___ [__  "#,
            r#"|___ | \|  |  |___ |  \    | \| |  | |  | |___ ___] "#,
            r#"                                                    "#,
        ]
        .iter()
        .enumerate()
        {
            fb.text(&ln[..fb.width().min(ln.len())], 0, i)?;
        }

        if self.names.len() < 2 {
            fb.text("Add at least 2 players...", 1, 5)?;
        }

        for (i, name) in self.names.iter().enumerate() {
            fb.text("- ", 0, i + 7)?;
            fb.text_wrapped(name, 2, i + 7, fb.width() - 7)?;
        }

        fb.set_input_prompt(
            "Enter next player's name (max 50 chars), leave empty to begin game".to_string(),
        );

        Ok(())
    }
}

pub fn select_names() -> RenderResult<Vec<String>> {
    let mut players = vec![];

    loop {
        let new_name = (NameSetScreen {
            names: &players[..],
        })
        .render_then_input(None)?
        .trim()
        .to_string();

        if new_name.is_empty() {
            if players.len() >= 2 {
                break;
            }
        } else {
            players.push(new_name);
        }
    }

    Ok(players)
}
