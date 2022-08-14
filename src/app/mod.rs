pub mod bindings;
pub mod content;
pub mod footer;
pub mod header;

use egui_winit::clipboard::Clipboard;

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
    pub use_tcpkill: bool,
    pub bindings: Bindings,
    pub widgets: Vec<Box<dyn Component>>,
}

impl App {
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        cc.egui_ctx.set_visuals(egui::Visuals::dark());

        Self {
            name: "Frostblink".to_string(),
            clipboard: Clipboard::default(),
            x: 0,
            y: 0,
            visible: true,
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

impl eframe::App for App {
    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        for widget in &mut self.widgets {
            widget.show(ctx);
        }

        self.bindings.show(ctx);

        frame.set_window_size(ctx.used_size());
    }
}
