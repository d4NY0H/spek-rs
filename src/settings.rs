use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;

use crate::palettes::SpectrogramColorScheme;
use crate::utils::SpectogramWinFunc;

#[derive(Serialize, Deserialize, Debug, Clone, Copy, PartialEq)]
pub enum SpectrogramScale {
    Linear,
    Log,
    FourthRt,
    FifthRt,
}

impl SpectrogramScale {
    pub const ALL: [Self; 4] = [
        Self::Linear,
        Self::Log,
        Self::FourthRt,
        Self::FifthRt,
    ];

    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Linear => "Linear",
            Self::Log => "Log",
            Self::FourthRt => "Fourth root",
            Self::FifthRt => "Fifth root",
        }
    }
}

impl std::fmt::Display for SpectrogramScale {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(default)]
pub struct AppSettings {
    pub color_scheme: SpectrogramColorScheme,
    pub win_func: SpectogramWinFunc,
    pub scale: SpectrogramScale,
    pub gain: f32,
    pub saturation: f32,
    pub split_channels: bool,
    pub resize_with_window: bool,
    pub custom_resolution: bool,
    pub resolution: [u32; 2],
    pub horizontal: bool,
    pub legend: bool,
    pub live_mode: bool,
    pub remember_settings: bool,
    pub custom_legend: bool,
    pub save_window_size: bool,
    pub window_size: [f32; 2],

    // -------------------------
    // Headless / PNG Optionen
    // -------------------------
    pub png_width: u32,
    pub png_height: u32,
    pub show_version_in_legend: bool,
}

impl Default for AppSettings {
    fn default() -> Self {
        Self {
            color_scheme: SpectrogramColorScheme::Intensity,
            win_func: SpectogramWinFunc::Hann,
            scale: SpectrogramScale::Log,
            gain: 1.0,
            saturation: 1.0,
            split_channels: false,
            resize_with_window: true,
            custom_resolution: false,
            resolution: [500, 320],
            horizontal: false,
            legend: true,
            live_mode: false,
            remember_settings: false,
            custom_legend: true,
            save_window_size: false,
            window_size: [500.0 + 180.0, 320.0 + 128.0 + 39.0],

            // Headless Defaults (ändert aktuelles Verhalten NICHT)
            png_width: 0,   // 0 = automatisch / bestehende Logik
            png_height: 0,  // 0 = automatisch / bestehende Logik
            show_version_in_legend: true,
        }
    }
}

impl AppSettings {
    fn config_path() -> Option<PathBuf> {
        dirs::config_dir().map(|mut path| {
            path.push("spek-rs");
            fs::create_dir_all(&path).ok();
            path.push("config.toml");
            path
        })
    }

    pub fn load() -> Self {
        if let Some(path) = Self::config_path() {
            if let Ok(content) = fs::read_to_string(path) {
                if let Ok(settings) = toml::from_str::<AppSettings>(&content) {
                    if settings.remember_settings {
                        return settings;
                    }
                }
            }
        }
        Self::default()
    }

    pub fn save(&self) {
        if let Some(path) = Self::config_path() {
            if let Ok(content) = toml::to_string_pretty(self) {
                if fs::write(path, content).is_err() {
                    eprintln!("Failed to write config file.");
                }
            }
        }
    }
}
