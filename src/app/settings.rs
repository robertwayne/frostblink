use egui::TextEdit;

use super::{Component, View};

pub struct Settings {
    pub game_log_path: String,
}

impl Default for Settings {
    fn default() -> Self {
        Self { game_log_path: "".to_string() }
    }
}

impl Settings {
    pub fn new(game_log_path: Option<String>) -> Self {
        Self { game_log_path: game_log_path.unwrap_or_else(|| "".to_string()) }
    }
}

impl Component for Settings {
    fn name(&self) -> &'static str {
        "Settings"
    }

    fn show(&mut self, ctx: &egui::Context) {
        egui::TopBottomPanel::bottom("settings").show(ctx, |ui| self.ui(ui));
    }
}

impl View for Settings {
    fn ui(&mut self, ui: &mut egui::Ui) {
        ui.with_layout(egui::Layout::left_to_right(), |ui| {
            let log_path = ui.label("Game Log Path");
            let edit = TextEdit::singleline(&mut self.game_log_path).show(ui);
        });
    }
}
