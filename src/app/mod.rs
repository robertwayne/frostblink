pub mod bindings;
pub mod content;
pub mod footer;
pub mod header;

use std::sync::Arc;

use egui_winit::clipboard::Clipboard;
use epi::*;

use crate::ToggleOverlaySignal;

use self::{bindings::Bindings, content::Content, footer::Footer, header::Header};

pub trait View {
    fn ui(&mut self, ui: &mut egui::Ui);
}

pub trait Component {
    fn name(&self) -> &'static str;
    fn show(&mut self, ctx: &egui::Context);
}

pub struct App {
    pub name: String,
    pub clipboard: Clipboard,
    pub x: i32,
    pub y: i32,
    pub visible: bool,
    pub toggle_signal: Arc<ToggleOverlaySignal>,
    pub use_tcpkill: bool,
    pub bindings: Bindings,
    pub widgets: Vec<Box<dyn Component>>,
}

impl App {
    pub fn new(toggle_signal: Arc<ToggleOverlaySignal>) -> Self {
        Self {
            name: "Frostblink".to_string(),
            clipboard: Clipboard::default(),
            x: 0,
            y: 0,
            visible: true,
            toggle_signal,
            use_tcpkill: true,
            bindings: Bindings::default(),
            widgets: vec![
                Box::new(Header::default()),
                Box::new(Footer::default()),
                Box::new(Content::default()),
            ],
        }
    }
}

impl epi::App for App {
    fn name(&self) -> &str {
        &self.name
    }

    fn update(&mut self, ctx: &egui::Context, frame: &epi::Frame) {
        for widget in &mut self.widgets {
            widget.show(ctx);
        }

        self.bindings.show(ctx);

        frame.set_window_size(ctx.used_size());
    }
}
