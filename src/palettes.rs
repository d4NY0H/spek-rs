use crate::settings::SpectrogramColorScheme;

// ======================================================
// Palette definitions (YUV)
// ======================================================

const INTENSITY: &[(f32, f32, f32, f32)] = &[
    (0.0, 0.0, 0.0, 0.0),
    (1.0, 1.0, 0.0, 0.0),
];

const RAINBOW: &[(f32, f32, f32, f32)] = &[
    (0.0, 0.0, 0.0, 0.0),
    (0.16, 0.5, 0.5, 0.0),
    (0.33, 0.5, 0.0, 0.5),
    (0.50, 0.5, -0.5, 0.0),
    (0.66, 0.5, 0.0, -0.5),
    (0.83, 0.5, 0.5, -0.5),
    (1.0, 1.0, 0.0, 0.0),
];

const FIRE: &[(f32, f32, f32, f32)] = &[
    (0.0, 0.0, 0.0, 0.0),
    (0.4, 0.6, 0.2, 0.0),
    (0.7, 0.9, 0.0, -0.2),
    (1.0, 1.0, 0.0, 0.0),
];

/// Used for "Ice"
const COOL: &[(f32, f32, f32, f32)] = &[
    (0.0, 0.0, 0.0, 0.0),
    (0.5, 0.5, -0.3, 0.3),
    (1.0, 1.0, 0.0, 0.0),
];

// ======================================================
// Public API
// ======================================================

pub fn get_palette(
    scheme: SpectrogramColorScheme,
) -> &'static [(f32, f32, f32, f32)] {
    match scheme {
        SpectrogramColorScheme::Intensity => INTENSITY,
        SpectrogramColorScheme::Rainbow => RAINBOW,
        SpectrogramColorScheme::Fire => FIRE,
        SpectrogramColorScheme::Ice => COOL,
    }
}
