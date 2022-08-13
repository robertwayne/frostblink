use inputbot::KeybdKey::LControlKey;
use tracing::debug;

use crate::{app::App, commands};

use super::Command;

#[derive(Debug, Default)]
pub struct Hideout;

impl Command for Hideout {
    fn run(&self, app: &mut App) {
        app.bindings.hideout.bind(|| {
            if LControlKey.is_pressed() {
                debug!("Moving to hideout...");
                commands::send("/hideout");
            };
        });
    }
}
