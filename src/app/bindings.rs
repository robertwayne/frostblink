use super::{Component, View};
use egui::{pos2, Rect, RichText};
use inputbot::KeybdKey;
use std::collections::HashMap;
use tracing::debug;

pub struct Bindings {
    pub disconnect: KeybdKey,
    pub hideout: KeybdKey,
    pub dnd: KeybdKey,
    pub custom: HashMap<String, KeybdKey>,
}

impl Default for Bindings {
    fn default() -> Self {
        Self {
            disconnect: KeybdKey::BackquoteKey,
            hideout: KeybdKey::Numrow5Key,
            dnd: KeybdKey::F1Key,
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
        let label_disconnect = ui.label("Disconnect");
        let edit_size_max = label_disconnect.rect.max.x + 80.;

        let _key = inputbot::from_keybd_key(self.disconnect);

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

        let label_hideout = ui.label("Hideout");
        let _response = ui.put(
            Rect {
                min: pos2(label_disconnect.rect.max.x + 20., label_hideout.rect.min.y),
                max: pos2(edit_size_max, label_hideout.rect.max.y),
            },
            egui::Label::new(self.hideout.to_string()),
        );

        let label_dnd = ui.label("DND");

        let response = ui.put(
            Rect {
                min: pos2(label_disconnect.rect.max.x + 20., label_dnd.rect.min.y),
                max: pos2(edit_size_max, label_dnd.rect.max.y),
            },
            egui::Label::new(self.dnd.to_string()),
        );

        if response.has_focus() {
            ui.label(
                RichText::new("Press the hotkey you want to use to disconnect.")
                    .color(egui::Color32::RED),
            );
        }

        // if self.hotkeys.disconnect.is_empty() {
        //     ui.label(RichText::new("Hotkey cannot be empty.").color(egui::Color32::RED));
        // }

        if response.lost_focus() {
            debug!("Disconnect hotkey changed to {:?}", self.disconnect);
        }
    }
}
