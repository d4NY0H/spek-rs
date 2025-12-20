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

    // -----------------------------
    // Default Settings
    // -----------------------------
    let mut settings = AppSettings::default();

    // -----------------------------
    // Optional CLI Flags
    // -----------------------------
    let mut i = 3;
    while i < args.len() {
        match args[i].as_str() {
            "--width" => {
                if i + 1 < args.len() {
                    settings.png_width = args[i + 1].parse().unwrap_or(0);
                    i += 1;
                }
            }
            "--height" => {
                if i + 1 < args.len() {
                    settings.png_height = args[i + 1].parse().unwrap_or(0);
                    i += 1;
                }
            }
            "--no-version" => {
                settings.show_version_in_legend = false;
            }
            "--help" | "-h" => {
                print_help(&args[0]);
                return;
            }
            _ => {}
        }
        i += 1;
    }

    // -----------------------------
    // Headless App
    // -----------------------------
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
  --no-version        Hide Spek version text in legend
  -h, --help          Show this help
"#,
        bin = bin
    );
}
