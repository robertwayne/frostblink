use inputbot::KeybdKey::*;
use tracing::debug;

use crate::{app::App, commands};

use super::Command;

#[derive(Debug, Default)]
pub struct DoNotDisturb;

impl Command for DoNotDisturb {
    fn run(&self, _app: &mut App) {
        F2Key.bind(|| {
            // Because we use modifiers elsewhere, we need to ensure that they aren't active
            // before we use the `send` function.
            if !LShiftKey.is_pressed() {
                debug!("Toggling DND...");
                commands::send("/dnd");
            };
        });
    }
}
