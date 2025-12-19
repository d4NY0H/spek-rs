// src/lib.rs

// Core-Module
pub mod ffmpeg_setup;
pub mod legend;
pub mod palettes;
pub mod settings;
pub mod utils;

// UI-Modul (Ordner src/ui/)
pub mod ui;

// Wichtig: MyApp aus ui::mod.rs nach außen exportieren
pub use ui::MyApp;
