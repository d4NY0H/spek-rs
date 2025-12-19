use crate::settings::AppSettings;
use crate::MyApp;
use crate::utils::save_color_image_as_png;
use std::env;
use std::path::Path;

fn main() {
    // Kommandozeilenargumente: <input_file> <output_file>
    let args: Vec<String> = env::args().collect();
    if args.len() != 3 {
        eprintln!("Usage: {} <input_audio> <output_png>", args[0]);
        std::process::exit(1);
    }

    let input_path = args[1].clone();
    let output_path = args[2].clone();

    // Standard-Einstellungen laden
    let settings = AppSettings::default();

    // Headless MyApp
    let mut app = MyApp::new(None, Some(input_path.clone()), settings);

    // Spektrogramm erzeugen
    if let Some(color_image) = app.regenerate_spectrogram_headless() {
        let path = Path::new(&output_path);
        match save_color_image_as_png(&color_image, path) {
            Ok(_) => println!("Saved PNG to {:?}", path),
            Err(e) => eprintln!("Failed to save PNG: {}", e),
        }
    } else {
        eprintln!("Failed to generate spectrogram");
        std::process::exit(1);
    }
}
