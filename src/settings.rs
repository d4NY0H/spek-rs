use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;

// ======================================================
// Core enums – OWNED by settings
// ======================================================

#[derive(Serialize, Deserialize, Debug, Clone, Copy, PartialEq)]
pub enum SpectrogramColorScheme {
    Intensity,
    Fire,
    Ice,
    Rainbow,
}

impl std::fmt::Display for SpectrogramColorScheme {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, Copy, PartialEq)]
pub enum SpectogramWinFunc {
    Hann,
    Hamming,
    Blackman,
}

impl std::fmt::Display for SpectogramWinFunc {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

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
}

impl std::fmt::Display for SpectrogramScale {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

// ======================================================
// App Settings (Headless-first, GUI tolerated for now)
// ======================================================

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(default)]
pub struct AppSettings {
    pub color_scheme: SpectrogramColorScheme,
    pub win_func: SpectogramWinFunc,
    pub scale: SpectrogramScale,
    pub gain: f32,
    pub saturation: f32,
    pub split_channels: bool,

    // Resolution
    pub custom_resolution: bool,
    pub resolution: [u32; 2],

    // Legend (ALWAYS ON for you)
    pub legend: bool,

    // Headless / PNG
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

            custom_resolution: false,
            resolution: [500, 320],

            legend: true,

            png_width: 0,
            png_height: 0,
            show_version_in_legend: true,
        }
    }
}

// ======================================================
// Optional config persistence (can be removed later)
// ======================================================

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
                    return settings;
                }
            }
        }
        Self::default()
    }

    pub fn save(&self) {
        if let Some(path) = Self::config_path() {
            if let Ok(content) = toml::to_string_pretty(self) {
                let _ = fs::write(path, content);
            }
        }
    }
}
