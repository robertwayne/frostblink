use copypasta::{ClipboardContext, ClipboardProvider};
use inputbot::KeybdKey::{EnterKey, LControlKey, VKey};
use std::{thread::sleep, time::Duration};

use crate::app::App;

use self::{disconnect::Disconnect, dnd::DoNotDisturb, exit::Exit, hideout::Hideout, kills::Kills};

pub mod disconnect;
pub mod dnd;
pub mod exit;
pub mod hideout;
pub mod kills;

pub trait Command {
    fn run(&self, app: &mut App);
}

pub fn initialize(app: &mut App) {
    // This prevents a stutter on Linux systems when the first keybind is
    // executed.
    if cfg!(target_os = "linux") {
        inputbot::init_device();
    }

    Hideout::default().run(app);
    Disconnect::default().run(app);
    Exit::default().run(app);
    DoNotDisturb::default().run(app);
    Kills::default().run(app);

    // Spawn a thread to handle the global hotkey listener.
    std::thread::spawn(move || {
        inputbot::handle_input_events();
    });
}

/// Copies a given string into the clipboard, then executes a key sequence to
/// simulate pressing Enter -> Ctrl+V -> Enter.
pub fn send(text: &str) {
    if let Ok(mut ctx) = ClipboardContext::new() {
        if ctx.set_contents(text.into()).is_ok() {
            EnterKey.press();
            EnterKey.release();

            sleep(Duration::from_millis(50));

            // This might need some tweaking to be stable. Seems to work on my
            // machine right now 100% of the time.

            LControlKey.press();
            sleep(Duration::from_millis(50));
            VKey.press();
            sleep(Duration::from_millis(50));
            VKey.release();
            sleep(Duration::from_millis(50));
            LControlKey.release();

            sleep(Duration::from_millis(50));

            EnterKey.press();
            EnterKey.release();
        }
    }
}
