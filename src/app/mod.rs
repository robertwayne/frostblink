pub mod bindings;
pub mod content;
pub mod footer;
pub mod header;
pub mod settings;

use std::sync::{Arc, Mutex};

use egui_winit::clipboard::Clipboard;

use self::{
    bindings::Bindings, content::Content, footer::Footer, header::Header, settings::Settings,
};

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
    pub game_log_path: Arc<Mutex<String>>,
    pub widgets: Vec<Box<dyn Component>>,
}

impl App {
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        cc.egui_ctx.set_visuals(egui::Visuals::dark());
        let storage = cc.storage.expect("no storage");
        let path = storage.get_string("game_log_path");
        let settings = Settings::new(Some(
            "/home/rob/.steam/debian-installation/steamapps/common/Path of Exile/logs/Client.txt"
                .to_string(),
        ));

        let game_log_path = Arc::new(Mutex::new(settings.game_log_path.clone()));

        Self {
            name: "Frostblink".to_string(),
            clipboard: Clipboard::default(),
            x: 0,
            y: 0,
            visible: true,
            use_tcpkill: true,
            bindings: Bindings::default(),
            game_log_path,
            // This order is important!
            widgets: vec![
                Box::new(Header::default()),
                Box::new(Footer::default()),
                Box::new(settings),
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
