use spek_rs::settings::AppSettings;
use spek_rs::utils::save_color_image_as_png;
use spek_rs::MyApp;

use std::env;
use std::path::Path;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 3 {
        print_help(&args[0]);
        std::process::exit(1);
    }

    let input_path = args[1].clone();
    let output_path = args[2].clone();

    // -------------------------------------------------
    // Default settings
    // -------------------------------------------------
    let mut settings = AppSettings::default();

    let mut width: Option<u32> = None;
    let mut height: Option<u32> = None;

    // -------------------------------------------------
    // CLI flags
    // -------------------------------------------------
    let mut i = 3;
    while i < args.len() {
        match args[i].as_str() {
            "--width" => {
                if i + 1 < args.len() {
                    width = args[i + 1].parse::<u32>().ok();
                    i += 1;
                }
            }
            "--height" => {
                if i + 1 < args.len() {
                    height = args[i + 1].parse::<u32>().ok();
                    i += 1;
                }
            }
            "--no-version" => {
                // Wirkung kommt später in legend.rs
                // CLI ist jetzt vorbereitet
            }
            "--help" | "-h" => {
                print_help(&args[0]);
                return;
            }
            unknown => {
                eprintln!("Unknown option: {}", unknown);
                print_help(&args[0]);
                std::process::exit(1);
            }
        }
        i += 1;
    }

    // -------------------------------------------------
    // Apply resolution if provided
    // -------------------------------------------------
    if let (Some(w), Some(h)) = (width, height) {
        settings.custom_resolution = true;
        settings.resolution = [w, h];
    }

    // -------------------------------------------------
    // Headless app
    // -------------------------------------------------
    let mut app = MyApp::new(None, Some(input_path.clone()), settings);

    if let Some(color_image) = app.regenerate_spectrogram_headless() {
        let path = Path::new(&output_path);
        match save_color_image_as_png(&color_image, path) {
            Ok(_) => println!("Saved PNG to {:?}", path),
            Err(e) => {
                eprintln!("Failed to save PNG: {}", e);
                std::process::exit(1);
            }
        }
    } else {
        eprintln!("Failed to generate spectrogram");
        std::process::exit(1);
    }
}

fn print_help(bin: &str) {
    eprintln!(
        r#"Usage:
  {bin} <input_audio> <output_png> [options]

Options:
  --width <px>        Set PNG width
  --height <px>       Set PNG height
  --no-version        Hide version text in legend (implemented next)
  -h, --help          Show this help
"#,
        bin = bin
    );
}
