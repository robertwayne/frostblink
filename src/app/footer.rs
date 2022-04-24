use super::{Component, View};

#[derive(Default)]
pub struct Footer;

impl Component for Footer {
    fn name(&self) -> &'static str {
        "Footer"
    }

    fn show(&mut self, ctx: &egui::Context) {
        egui::TopBottomPanel::bottom("footer").show(ctx, |ui| self.ui(ui));
    }
}

impl View for Footer {
    fn ui(&mut self, ui: &mut egui::Ui) {
        ui.with_layout(egui::Layout::right_to_left(), |ui| {
            ui.hyperlink_to("GitHub", "https://github.com/robertwayne/frostblink");
            ui.label("♡ Open Source");
        });
    }
}
