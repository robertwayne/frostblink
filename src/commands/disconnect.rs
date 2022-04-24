use std::{thread::sleep, time::Duration};

use inputbot::KeybdKey::*;
use tracing::debug;

use crate::{app::App, commands};

use super::Command;

#[derive(Debug, Default)]
pub struct Disconnect;

impl Command for Disconnect {
    /// Attempts to disconnect from Path of Exile by closing the TCP connection (by default). On
    /// Linux, this solves this by setting a temporary IP Tables rule that blocks the PoE port, then
    /// removes that rule shortly after.
    ///
    /// On Windows, this has no implementation for closing the connection (yet).
    ///
    /// If the `Use TCP Kill` box is not ticked this function will instead use the command /exit to
    /// disconnect. This can be less reliable due to the time it takes to copy, paste, and send the
    /// command - but it is guarenteed to log you out of the game in time if it gets through.
    fn run(&self, app: &mut App) {
        // Unlike other binds where we watch for modifiers, in this case we ALWAYS force disconnect
        // to be available to the key it is bound to. It is an emergency key, and should not be
        // blocked by modifiers.
        let use_tcpkill = app.use_tcpkill;

        BackquoteKey.bind(move || {
            debug!("Disconnecting...");

            if cfg!(target_os = "linux") {
                match use_tcpkill {
                    true => {
                        std::process::Command::new("iptables")
                            .args([
                                "-I",
                                "INPUT",
                                "-p",
                                "tcp",
                                "--sport",
                                "6112",
                                "--tcp-flags",
                                "PSH,ACK",
                                "PSH,ACK",
                                "-j",
                                "REJECT",
                                "--reject-with",
                                "tcp-reset",
                            ])
                            .spawn()
                            .expect("Failed to run `iptables` command.");

                        sleep(Duration::from_millis(1000));

                        std::process::Command::new("iptables")
                            .args([
                                "-D",
                                "INPUT",
                                "-p",
                                "tcp",
                                "--sport",
                                "6112",
                                "--tcp-flags",
                                "PSH,ACK",
                                "PSH,ACK",
                                "-j",
                                "REJECT",
                                "--reject-with",
                                "tcp-reset",
                            ])
                            .spawn()
                            .expect("Failed to run `iptables` command.");
                    }
                    false => {
                        commands::send("/exit");
                    }
                }
            }

            if cfg!(target_os = "windows") {
                // This needs a TCP kill equivalent.
                commands::send("/exit");
            }
        });
    }
}
