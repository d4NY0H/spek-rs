#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release

use eframe::egui;
use image;
use std::env;
use std::process::{Command, Stdio};

mod ui;
use ui::MyApp;

mod ffmpeg_setup;
mod legend;
mod palettes;
mod settings;
mod utils;

fn main() -> eframe::Result {
    ffmpeg_setup::setup_ffmpeg()?;

    env_logger::init();
    println!("spek-rs v{}", env!("CARGO_PKG_VERSION"));

    let args: Vec<String> = env::args().collect();

    // -----------------------------
    // CLI parsing
    // -----------------------------
    let mut input_path: Option<String> = None;
    let mut png_output: Option<String> = None;

    let mut i = 1;
    while i < args.len() {
        match args[i].as_str() {
            "--png" => {
                if i + 1 < args.len() {
                    png_output = Some(args[i + 1].clone());
                    i += 1;
                }
            }
            arg => {
                if input_path.is_none() {
                    input_path = Some(arg.to_string());
                }
            }
        }
        i += 1;
    }

    let app_settings = settings::AppSettings::load();

    // -----------------------------
    // HEADLESS PNG MODE
    // -----------------------------
    if let Some(output_png) = png_output {
        let mut app = MyApp::new(None, input_path.clone(), app_settings);

        match app.regenerate_spectrogram_headless() {
            Some(image) => {
                utils::save_color_image_as_png(
    &image,
    std::path::Path::new(&output_png),
)
.expect("Failed to save PNG");
                println!("Saved spectrogram to {}", output_png);
            }
            None => {
                eprintln!("Failed to generate spectrogram");
            }
        }

        return Ok(());
    }

    // -----------------------------
    // GUI MODE (unchanged behaviour)
    // -----------------------------

    // Handle multiple files (spawn GUI instances)
    if args.len() > 2 {
        let exe_path = env::current_exe().expect("Failed to get current executable path");
        for path in args.iter().skip(2) {
            if let Err(e) = Command::new(&exe_path)
                .arg(path)
                .stdin(Stdio::null())
                .stdout(Stdio::null())
                .stderr(Stdio::null())
                .spawn()
            {
                eprintln!("Failed to spawn process for {}: {}", path, e);
            }
        }
    }

    let options = {
        let mut viewport = egui::ViewportBuilder::default();
        let icon = {
            let image = image::load_from_memory(include_bytes!("../assets/icon.ico"))
                .expect("Failed to load icon");
            let rgba = image.to_rgba8();
            let (width, height) = rgba.dimensions();
            egui::IconData {
                rgba: rgba.into_raw(),
                width,
                height,
            }
        };
        viewport = viewport.with_icon(std::sync::Arc::new(icon));

        if app_settings.save_window_size {
            viewport = viewport.with_inner_size(app_settings.window_size);
        } else {
            viewport = viewport.with_inner_size([500.0 + 180.0, 320.0 + 128.0 + 39.0]);
        }

        viewport = viewport
            .with_min_inner_size([500.0 + 180.0, 320.0 + 128.0 + 39.0])
            .with_resizable(true);

        eframe::NativeOptions {
            viewport,
            ..Default::default()
        }
    };

    eframe::run_native(
        "Spek-rs",
        options,
        Box::new(move |_cc| {
            egui_extras::install_image_loaders(&_cc.egui_ctx);
            _cc.egui_ctx.set_theme(egui::Theme::Dark);
            Ok(Box::new(MyApp::new(None, input_path, app_settings)))
        }),
    )
} 
