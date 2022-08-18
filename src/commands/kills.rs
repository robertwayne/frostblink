use std::{io::BufRead, sync::Arc};

use inputbot::KeybdKey::LControlKey;
use tracing::debug;

use crate::{app::App, commands};

use super::Command;

#[derive(Debug, Default)]
pub struct Kills;

impl Command for Kills {
    fn run(&self, app: &mut App) {
        let path = Arc::clone(&app.game_log_path);

        app.bindings.kills.bind(move || {
            // Because we use modifiers elsewhere, we need to ensure that they
            // aren't active before we use the `send` function.
            if LControlKey.is_pressed() {
                debug!("Getting kills...");
                commands::send("/kills");

                let p = path.lock().unwrap();
                if let Ok(file) = std::fs::File::open(&*p) {
                    let reader = std::io::BufReader::new(file);
                    let mut lines = reader.lines();
                    let mut last_line = String::new();

                    while let Some(Ok(line)) = lines.next() {
                        if line.contains("You have killed") {
                            last_line = line;
                        }
                    }

                    tracing::debug!("{}", last_line);
                }
            };
        });
    }
}
