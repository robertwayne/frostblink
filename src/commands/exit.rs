use inputbot::KeybdKey::LControlKey;
use tracing::debug;

use crate::{app::App, commands};

use super::Command;

#[derive(Debug, Default)]
pub struct Exit;

impl Command for Exit {
    fn run(&self, app: &mut App) {
        app.bindings.exit.bind(|| {
            if LControlKey.is_pressed() {
                debug!("Exiting to character select...");
                commands::send("/exit");
            };
        });
    }
}
