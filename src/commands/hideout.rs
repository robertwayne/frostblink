use inputbot::KeybdKey::*;
use tracing::debug;

use crate::{app::App, commands};

use super::Command;

#[derive(Debug, Default)]
pub struct Hideout;

impl Command for Hideout {
    fn run(&self, _app: &mut App) {
        Numrow7Key.bind(|| {
            // Because we use modifiers elsewhere, we need to ensure that they aren't active
            // before we use the `send` function.
            if !LShiftKey.is_pressed() {
                debug!("Moving to hideout...");
                commands::send("/hideout");
            };
        });
    }
}
