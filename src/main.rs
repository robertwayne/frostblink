mod app;
mod commands;
mod game;

use egui::{Pos2, Vec2};
use game::GameWindow;
use tracing_subscriber::fmt::format::FmtSpan;

// TODO: temporary; will be handled inside content via game_log_path
pub const GAME_LOG_PATH: &str =
    "/home/rob/.steam/debian-installation/steamapps/common/Path of Exile/logs/Client.txt";

fn main() -> Result<(), anyhow::Error> {
    tracing_subscriber::fmt()
        .with_env_filter("frostblink=debug,wgpu=error")
        .with_span_events(FmtSpan::CLOSE)
        .init();

    let game_window = GameWindow::new()?;
    let (x, y) = game_window.get_position()?;

    let opts = eframe::NativeOptions {
        always_on_top: true,
        decorated: true,
        resizable: false,
        initial_window_pos: Some(Pos2 {
            x: f32::from(x),
            y: f32::from(y),
        }),
        initial_window_size: Some(Vec2 { x: 400., y: 500. }),
        ..Default::default()
    };

    eframe::run_native(
        "Frostblink",
        opts,
        Box::new(|cc| {
            let mut app = app::App::new(cc);
            commands::initialize(&mut app);
            Box::new(app)
        }),
    );
}
