use super::{Component, View};
use egui::{pos2, Rect};
use inputbot::KeybdKey;
use std::collections::HashMap;
use tracing::debug;

pub struct Bindings {
    pub disconnect: KeybdKey,
    pub exit: KeybdKey,
    pub hideout: KeybdKey,
    pub dnd: KeybdKey,
    pub kills: KeybdKey,
    pub custom: HashMap<String, KeybdKey>,
}

impl Default for Bindings {
    fn default() -> Self {
        Self {
            disconnect: KeybdKey::BackquoteKey,
            exit: KeybdKey::TKey,
            hideout: KeybdKey::HKey,
            dnd: KeybdKey::DKey,
            kills: KeybdKey::KKey,
            custom: HashMap::new(),
        }
    }
}

impl Component for Bindings {
    fn name(&self) -> &'static str {
        "Bindings"
    }

    fn show(&mut self, ctx: &egui::Context) {
        egui::TopBottomPanel::top("bindings").show(ctx, |ui| self.ui(ui));
    }
}

impl View for Bindings {
    fn ui(&mut self, ui: &mut egui::Ui) {
        // TCP Kill / Hard Disconnect
        // This is used as the base for assigning size values to the other
        // bindings.
        let label_disconnect = ui.label("Disconnect");
        let edit_size_max = label_disconnect.rect.max.x + 80.;
        let _response = ui.put(
            Rect {
                min: pos2(
                    label_disconnect.rect.max.x + 20.,
                    label_disconnect.rect.min.y,
                ),
                max: pos2(edit_size_max, label_disconnect.rect.max.y),
            },
            egui::Label::new(self.disconnect.to_string()),
        );

        // Exit to Character Selection
        let label_exit = ui.label("Exit");
        let _response = ui.put(
            Rect {
                min: pos2(label_disconnect.rect.max.x + 20., label_exit.rect.min.y),
                max: pos2(edit_size_max, label_exit.rect.max.y),
            },
            egui::Label::new(format!("Ctrl {}", self.exit)),
        );

        // Hideout
        let label_hideout = ui.label("Hideout");
        let _response = ui.put(
            Rect {
                min: pos2(label_disconnect.rect.max.x + 20., label_hideout.rect.min.y),
                max: pos2(edit_size_max, label_hideout.rect.max.y),
            },
            egui::Label::new(format!("Ctrl {}", self.hideout)),
        );

        // Do Not Disturb
        let label_dnd = ui.label("DND");
        let response = ui.put(
            Rect {
                min: pos2(label_disconnect.rect.max.x + 20., label_dnd.rect.min.y),
                max: pos2(edit_size_max, label_dnd.rect.max.y),
            },
            egui::Label::new(format!("Ctrl {}", self.dnd)),
        );

        // if response.has_focus() {
        //     ui.label(
        //         RichText::new("Press the hotkey you want to use to disconnect.")
        //             .color(egui::Color32::RED),
        //     );
        // }

        let label_kills = ui.label("Kills");
        let response = ui.put(
            Rect {
                min: pos2(label_disconnect.rect.max.x + 20., label_kills.rect.min.y),
                max: pos2(edit_size_max, label_kills.rect.max.y),
            },
            egui::Label::new(format!("Ctrl {}", self.kills)),
        );

        // if self.hotkeys.disconnect.is_empty() {
        //     ui.label(RichText::new("Hotkey cannot be empty.").color(egui::Color32::RED));
        // }

        if response.lost_focus() {
            debug!("Disconnect hotkey changed to {:?}", self.disconnect);
        }
    }
}
