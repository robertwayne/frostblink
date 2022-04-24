use crate::app::App;
use inputbot::KeybdKey::*;
use tracing::debug;

use super::Command;

#[derive(Debug, Default)]
pub struct ToggleOverlay;

impl Command for ToggleOverlay {
    fn run(&self, app: &mut App) {
        let signal = app.toggle_signal.clone();

        LShiftKey.bind(|| {});

        VKey.bind(move || {
            if LShiftKey.is_pressed() && VKey.is_pressed() {
                debug!("Toggling overlay.");
                signal.toggle_overlay();
            }
        });
    }
}
