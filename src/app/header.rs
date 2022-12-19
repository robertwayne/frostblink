use egui::Direction;

use super::{Component, View};

#[derive(Default)]
pub struct Header;

impl Component for Header {
    fn name(&self) -> &'static str {
        "Header"
    }

    fn show(&mut self, ctx: &egui::Context) {
        egui::TopBottomPanel::top("Header").show(ctx, |ui| self.ui(ui));
    }
}

impl View for Header {
    fn ui(&mut self, ui: &mut egui::Ui) {
        ui.with_layout(egui::Layout::centered_and_justified(Direction::LeftToRight), |ui| {
            ui.label("Shift+V to toggle overlay.");
        });
    }
}
