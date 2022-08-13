mod app;
mod commands;
mod game;

use egui_wgpu_backend::{RenderPass, ScreenDescriptor};
use epi::{
    backend::{AppOutput, FrameData},
    egui, App,
};
use game::GameWindow;
use std::{
    iter,
    sync::{Arc, Mutex, PoisonError},
    time::Instant,
};
use tracing_subscriber::fmt::format::FmtSpan;
use winit::{
    dpi::{LogicalPosition, PhysicalSize},
    event::Event::{MainEventsCleared, RedrawRequested, UserEvent, WindowEvent},
    event_loop::{ControlFlow, EventLoop, EventLoopProxy},
    window::WindowBuilder,
};

// TODO: temporary; will be handled inside content via game_log_path
pub const GAME_LOG_PATH: &str =
    "/home/rob/.steam/debian-installation/steamapps/common/Path of Exile/logs/Client.txt";

enum Event {
    RequestRedraw,
    ToggleOverlay,
}

struct RepaintSignal(Mutex<EventLoopProxy<Event>>);
pub struct ToggleOverlaySignal(Mutex<EventLoopProxy<Event>>);

impl epi::backend::RepaintSignal for RepaintSignal {
    fn request_repaint(&self) {
        self.0
            .lock()
            .unwrap_or_else(PoisonError::into_inner)
            .send_event(Event::RequestRedraw)
            .ok();
    }
}

impl ToggleOverlaySignal {
    fn toggle_overlay(&self) {
        self.0
            .lock()
            .unwrap_or_else(PoisonError::into_inner)
            .send_event(Event::ToggleOverlay)
            .ok();
    }
}

fn main() -> Result<(), anyhow::Error> {
    tracing_subscriber::fmt()
        .with_env_filter("frostblink=debug,wgpu=error")
        .with_span_events(FmtSpan::CLOSE)
        .init();

    // TODO: acquire method that runs in separate thread and grabs the handle if
    // the window is closed / returns an Error
    let game_window = GameWindow::new()?;
    let (x, y) = game_window.get_position()?;

    let event_loop = EventLoop::with_user_event();
    let window = WindowBuilder::new()
        .with_decorations(false)
        .with_resizable(false)
        .with_transparent(true)
        .with_always_on_top(true)
        .with_position(LogicalPosition::new(x as f32, y as f32))
        .with_inner_size(PhysicalSize {
            width: 400,
            height: 500,
        })
        .build(&event_loop)?;

    // Currently there is no way to use the wgpu without this instance of unsafe
    let instance = wgpu::Instance::new(wgpu::Backends::PRIMARY);
    let surface = unsafe { instance.create_surface(&window) };

    let adapter = pollster::block_on(instance.request_adapter(&wgpu::RequestAdapterOptions {
        power_preference: wgpu::PowerPreference::HighPerformance,
        compatible_surface: Some(&surface),
        force_fallback_adapter: false,
    }))
    .expect("Failed to get adapter.");

    let (device, queue) = pollster::block_on(adapter.request_device(
        &wgpu::DeviceDescriptor {
            features: wgpu::Features::default(),
            limits: wgpu::Limits::default(),
            label: None,
        },
        None,
    ))?;

    let size = window.inner_size();
    let surface_format = surface
        .get_preferred_format(&adapter)
        .expect("Failed to get surface format.");

    let mut surface_config = wgpu::SurfaceConfiguration {
        usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
        format: surface_format,
        width: size.width as u32,
        height: size.height as u32,
        present_mode: wgpu::PresentMode::Fifo,
    };

    surface.configure(&device, &surface_config);

    let repaint_signal = Arc::new(RepaintSignal(Mutex::new(event_loop.create_proxy())));

    let toggle_overlay_signal =
        Arc::new(ToggleOverlaySignal(Mutex::new(event_loop.create_proxy())));

    // We use the egui_winit_platform crate as the platform.
    let mut state = egui_winit::State::new(4096, &window);
    let context = egui::Context::default();

    // We use the egui_wgpu_backend crate as the render backend.
    let mut egui_rpass = RenderPass::new(&device, surface_format, 1);

    // Initialize a clipboard and pass it into the app.
    let clipboard = egui_winit::clipboard::Clipboard::default();
    let mut app = app::App {
        clipboard,
        ..app::App::new(toggle_overlay_signal)
    };

    // Load all our global hotkeys and starts a listener thread.
    commands::initialize(&mut app);

    let mut previous_frame_time = None;
    event_loop.run(move |event, _, control_flow| {
        match event {
            RedrawRequested(..) => {
                if app.visible {
                    let output_frame = match surface.get_current_texture() {
                        Ok(frame) => frame,
                        Err(wgpu::SurfaceError::Outdated) => {
                            // This error occurs when the app is minimized on
                            // Windows. Silently return here to prevent spamming
                            // the console with: "The underlying surface has
                            // changed, and therefore the swap chain must be
                            // updated"
                            return;
                        }
                        Err(e) => {
                            eprintln!("Dropped frame with error: {}", e);
                            return;
                        }
                    };
                    let output_view = output_frame
                        .texture
                        .create_view(&wgpu::TextureViewDescriptor::default());

                    // Begin to draw the UI frame.
                    let egui_start = Instant::now();
                    let input = state.take_egui_input(&window);
                    context.begin_frame(input);
                    let app_output = AppOutput::default();

                    let frame = epi::Frame::new(FrameData {
                        info: epi::IntegrationInfo {
                            name: "frostblink",
                            web_info: None,
                            cpu_usage: previous_frame_time,
                            native_pixels_per_point: Some(window.scale_factor() as _),
                            prefer_dark_mode: None,
                        },
                        output: app_output,
                        repaint_signal: repaint_signal.clone(),
                    });

                    app.update(&context, &frame);

                    // End the UI frame. We could now handle the output and draw
                    // the UI with the backend.
                    let output = context.end_frame();
                    let paint_jobs = context.tessellate(output.shapes);

                    let frame_time = egui_start.elapsed().as_secs_f32();
                    previous_frame_time = Some(frame_time);

                    let mut encoder =
                        device.create_command_encoder(&wgpu::CommandEncoderDescriptor {
                            label: Some("encoder"),
                        });

                    // Upload all resources for the GPU.
                    let screen_descriptor = ScreenDescriptor {
                        physical_width: surface_config.width,
                        physical_height: surface_config.height,
                        scale_factor: window.scale_factor() as f32,
                    };

                    egui_rpass
                        .add_textures(&device, &queue, &output.textures_delta)
                        .expect("Failed to add textures to render pass.");
                    egui_rpass
                        .remove_textures(output.textures_delta)
                        .expect("Failed to remove textures");
                    egui_rpass.update_buffers(&device, &queue, &paint_jobs, &screen_descriptor);

                    // Record all render passes.
                    egui_rpass
                        .execute(
                            &mut encoder,
                            &output_view,
                            &paint_jobs,
                            &screen_descriptor,
                            Some(wgpu::Color::BLACK),
                        )
                        .expect("Failed to execute render pass.");

                    // Submit the commands.
                    queue.submit(iter::once(encoder.finish()));

                    // Redraw egui
                    output_frame.present();

                    // Suppport reactive on windows only, but not on linux.
                    if cfg!(target_os = "windows") {
                        *control_flow = ControlFlow::Poll;
                    } else {
                        *control_flow = ControlFlow::Wait;
                    }
                }
            }
            UserEvent(Event::ToggleOverlay) => {
                app.visible = !app.visible;
                window.set_visible(app.visible);

                if app.visible {
                    window.set_outer_position(LogicalPosition::new(app.x, app.y));
                    window.set_always_on_top(app.visible);
                }
            }
            MainEventsCleared | UserEvent(Event::RequestRedraw) => {
                window.request_redraw();
            }
            WindowEvent { event, .. } => match event {
                winit::event::WindowEvent::Resized(size) => {
                    // Resize with 0 width and height is used by winit to signal
                    // a minimize event on Windows. See:
                    // https://github.com/rust-windowing/winit/issues/208 This
                    // solves an issue where the app would panic when minimizing
                    // on Windows.
                    if size.width > 0 && size.height > 0 {
                        surface_config.width = size.width;
                        surface_config.height = size.height;
                        surface.configure(&device, &surface_config);
                    }
                }
                winit::event::WindowEvent::Moved(position) => {
                    app.x = position.x;
                    app.y = position.y;
                }
                winit::event::WindowEvent::CloseRequested => {
                    *control_flow = ControlFlow::Exit;
                }
                event => {
                    // Pass the winit events to the platform integration.
                    state.on_event(&context, &event);
                }
            },
            _ => (),
        }
    });
}
