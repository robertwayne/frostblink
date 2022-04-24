use super::{Component, View};

#[derive(Default)]
pub struct Content;

impl Component for Content {
    fn name(&self) -> &'static str {
        "Content"
    }

    fn show(&mut self, ctx: &egui::Context) {
        egui::CentralPanel::default().show(ctx, |ui| self.ui(ui));
    }
}

impl View for Content {
    fn ui(&mut self, ui: &mut egui::Ui) {
        ui.label("WIP Content Area");
        // if let Some(text) = self.clipboard.get() {
        //     ui.label(text);
        // }
    }
}
