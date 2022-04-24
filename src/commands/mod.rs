use copypasta::{ClipboardContext, ClipboardProvider};
use inputbot::KeybdKey::*;
use std::{thread::sleep, time::Duration};
use tracing::debug;

use crate::app::App;

use self::{
    disconnect::Disconnect, dnd::DoNotDisturb, hideout::Hideout, toggle_overlay::ToggleOverlay,
};

pub mod disconnect;
pub mod dnd;
pub mod hideout;
pub mod toggle_overlay;

pub trait Command {
    fn run(&self, app: &mut App);
}

pub fn initialize(app: &mut App) {
    if cfg!(target_os = "linux") {
        debug!("Spawning initial device.");
        inputbot::init_device();
    }

    Hideout::default().run(app);
    ToggleOverlay::default().run(app);
    Disconnect::default().run(app);
    DoNotDisturb::default().run(app);

    // Load all our global hotkeys.
    // disconnect::bind_disconnect(true);
    // hideout::bind_hideout();
    // dnd::bind_dnd();
    // toggle_visible::bind_toggle_visible(signal);

    // Spawn a thread to handle the global hotkey listener.
    std::thread::spawn(move || {
        inputbot::handle_input_events();
    });
}

/// Copies a given string into the clipboard, then executes a key sequence to simulate pressing
/// Enter -> Ctrl+V -> Enter.
pub fn send(text: &str) {
    if let Ok(mut ctx) = ClipboardContext::new() {
        if ctx.set_contents(text.into()).is_ok() {
            EnterKey.press();
            EnterKey.release();

            sleep(Duration::from_millis(20));

            // This might need some tweaking to be stable. Seems to work on my machine right now
            // 100% of the time.

            LControlKey.press();
            sleep(Duration::from_millis(20));
            VKey.press();
            sleep(Duration::from_millis(20));
            VKey.release();
            sleep(Duration::from_millis(20));
            LControlKey.release();

            sleep(Duration::from_millis(20));

            EnterKey.press();
            EnterKey.release();
        }
    }
}
