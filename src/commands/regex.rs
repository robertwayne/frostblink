use inputbot::KeybdKey::LControlKey;
use tracing::debug;

use crate::{app::App, commands};

use super::Command;

#[derive(Debug, Default)]
pub struct Regex;

impl Command for Regex {
    fn run(&self, app: &mut App) {
        app.bindings.regex.bind(|| {
            if LControlKey.is_pressed() {
                debug!("Pasting regex...");
                commands::paste("g-g-g|nne");
            };
        });
    }
}
