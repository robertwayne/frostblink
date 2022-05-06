use std::io::BufRead;

use inputbot::KeybdKey::*;
use tracing::debug;

use crate::{app::App, commands, GAME_LOG_PATH};

use super::Command;

#[derive(Debug, Default)]
pub struct Kills;

impl Command for Kills {
    fn run(&self, app: &mut App) {
        Numrow6Key.bind(|| {
            // Because we use modifiers elsewhere, we need to ensure that they aren't active
            // before we use the `send` function.
            if !LShiftKey.is_pressed() {
                debug!("Getting kills...");
                commands::send("/kills");

                let file = std::fs::File::open(GAME_LOG_PATH).unwrap();
                let reader = std::io::BufReader::new(file);
                let mut lines = reader.lines();
                let mut last_line = String::new();

                while let Some(Ok(line)) = lines.next() {
                    if line.contains("You have killed") {
                        last_line = line;
                    }
                }

                tracing::debug!("{}", last_line);
            };
        });
    }
}
