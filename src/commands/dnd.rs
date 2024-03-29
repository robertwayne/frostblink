use inputbot::KeybdKey::LControlKey;
use tracing::debug;

use crate::{app::App, commands};

use super::Command;

#[derive(Debug, Default)]
pub struct DoNotDisturb;

impl Command for DoNotDisturb {
    fn run(&self, app: &mut App) {
        app.bindings.dnd.bind(|| {
            if LControlKey.is_pressed() {
                debug!("Toggling DND...");
                commands::send("/dnd");
            };
        });
    }
}
