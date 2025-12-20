use crate::palettes;
use crate::utils::AudioInfo;
use ab_glyph::{Font, FontVec, PxScale};
use font_kit::source::SystemSource;
use image::{Rgba, RgbaImage};
use imageproc::drawing::{draw_filled_rect_mut, draw_line_segment_mut, draw_text_mut};
use imageproc::rect::Rect;

pub const TOP_MARGIN: u32 = 64;
pub const BOTTOM_MARGIN: u32 = 64;
pub const LEFT_MARGIN: u32 = 80;
pub const RIGHT_MARGIN: u32 = 100;

fn draw_time_scale(
    image: &mut RgbaImage,
    spec_width: u32,
    spec_height: u32,
    duration: f64,
    font: &FontVec,
    scale: PxScale,
    color: Rgba<u8>,
    is_top: bool,
    draw_labels: bool,
) {
    let num_ticks = 10;
    for i in 0..=num_ticks {
        let fraction = i as f32 / num_ticks as f32;
        let x = (LEFT_MARGIN - 1) as f32 + fraction * (spec_width as f32 + 1.0);

        let (y_start, y_end, label_y) = if is_top {
            let y_start = TOP_MARGIN as f32 - 6.0;
            let y_end = TOP_MARGIN as f32 - 1.0;
            let (_, text_height) = imageproc::drawing::text_size(scale, font, "0");
            (y_start, y_end, y_start - text_height as f32 - 4.0)
        } else {
            let y_start = TOP_MARGIN as f32 + spec_height as f32;
            let y_end = y_start + 5.0;
            (y_start, y_end, y_end + 8.0)
        };

        draw_line_segment_mut(image, (x, y_start), (x, y_end), color);

        if draw_labels {
            let time_sec = duration * fraction as f64;
            let minutes = (time_sec / 60.0).floor() as u32;
            let seconds = (time_sec % 60.0).floor() as u32;
            let label = format!("{}:{:02}", minutes, seconds);
            let (text_width, _) = imageproc::drawing::text_size(scale, font, &label);
            draw_text_mut(
                image,
                color,
                (x - text_width as f32 / 2.0) as i32,
                label_y as i32,
                scale,
                font,
                &label,
            );
        }
    }
}

fn draw_freq_scale(
    image: &mut RgbaImage,
    spec_width: u32,
    spec_height: u32,
    audio_info: AudioInfo,
    font: &FontVec,
    scale: PxScale,
    color: Rgba<u8>,
    split_channels: bool,
) {
    let max_freq_khz = (audio_info.sample_rate / 2) as f32 / 1000.0;
    let draw_multi_channel = audio_info.channels > 1 && split_channels;

    let channel_count = if draw_multi_channel { 2 } else { 1 };
    let height_per_channel = if draw_multi_channel {
        spec_height / 2
    } else {
        spec_height
    };

    for channel in 0..channel_count {
        let y_offset = TOP_MARGIN + (channel * height_per_channel);
        let num_ticks = if draw_multi_channel { 5 } else { 10 };

        for i in 0..=num_ticks {
            let fraction = i as f32 / num_ticks as f32;
            let y = (y_offset - 1) as f32 + (1.0 - fraction) * (height_per_channel + 1) as f32;

            let x_start_left = LEFT_MARGIN as f32 - 6.0;

            if !(draw_multi_channel && channel == 1 && i == num_ticks) {
                let x_end_left = LEFT_MARGIN as f32 - 1.0;
                draw_line_segment_mut(image, (x_start_left, y), (x_end_left, y), color);

                let x_start_right = LEFT_MARGIN as f32 + spec_width as f32 + 1.0;
                let x_end_right = x_start_right + 5.0;
                draw_line_segment_mut(image, (x_start_right, y), (x_end_right, y), color);
            }

            if !(draw_multi_channel && channel == 1 && i == num_ticks) {
                let freq_khz = fraction * max_freq_khz;
                let label = format!("{:.0} kHz", freq_khz);
                let (text_width, text_height) =
                    imageproc::drawing::text_size(scale, font, &label);
                draw_text_mut(
                    image,
                    color,
                    (x_start_left - text_width as f32 - 8.0) as i32,
                    (y - text_height as f32 / 2.0) as i32 - 2,
                    scale,
                    font,
                    &label,
                );
            }
        }
    }
}

fn draw_dbfs_scale(
    image: &mut RgbaImage,
    spec_width: u32,
    spec_height: u32,
    font: &FontVec,
    scale: PxScale,
    color: Rgba<u8>,
) {
    let db_range: f32 = -120.0;
    let num_ticks = 10;
    let gradient_x = LEFT_MARGIN as f32 + spec_width as f32 + 34.0;
    let gradient_width = 10.0;
    let label_x = gradient_x + gradient_width + 5.0;

    for i in 0..=num_ticks {
        let fraction = i as f32 / num_ticks as f32;
        let y = (TOP_MARGIN - 1) as f32 + (1.0 - fraction) * (spec_height + 1) as f32;
        let db_level = (fraction - 1.0) * db_range.abs();
        let label = format!("{:.0}", db_level);
        let (_, text_height) = imageproc::drawing::text_size(scale, font, &label);

        draw_text_mut(
            image,
            color,
            label_x as i32,
            (y - text_height as f32 / 2.0) as i32 - 2,
            scale,
            font,
            &label,
        );
    }
}

fn truncate_text(font: &FontVec, scale: PxScale, text: &str, max_width: u32) -> String {
    let (text_width, _) = imageproc::drawing::text_size(scale, font, text);
    if text_width <= max_width {
        return text.to_string();
    }

    let ellipsis = "...";
    let (ellipsis_width, _) = imageproc::drawing::text_size(scale, font, ellipsis);
    let mut truncated = text.to_string();

    if max_width > ellipsis_width {
        let target_width = max_width - ellipsis_width;
        while !truncated.is_empty() {
            let (w, _) = imageproc::drawing::text_size(scale, font, &truncated);
            if w <= target_width {
                truncated.push_str(ellipsis);
                return truncated;
            }
            truncated.pop();
        }
    }
    String::new()
}

use crate::settings::SpectrogramColorScheme;

/// Creates an image with a legend template.
pub fn draw_legend(
    spec_width: u32,
    spec_height: u32,
    filename: &str,
    ffmpeg_settings: &str,
    audio_info: Option<AudioInfo>,
    saturation: f32,
    color_scheme: SpectrogramColorScheme,
    split_channels: bool,
    show_version: bool,
) -> RgbaImage {
    let final_width = spec_width + LEFT_MARGIN + RIGHT_MARGIN;
    let final_height = spec_height + TOP_MARGIN + BOTTOM_MARGIN;

    let mut image = RgbaImage::new(final_width, final_height);
    draw_filled_rect_mut(
        &mut image,
        Rect::at(0, 0).of_size(final_width, final_height),
        Rgba([0, 0, 0, 255]),
    );

    let white = Rgba([255, 255, 255, 255]);
    let tl = (LEFT_MARGIN as f32 - 1.0, TOP_MARGIN as f32 - 1.0);
    let tr = ((LEFT_MARGIN + spec_width) as f32, TOP_MARGIN as f32 - 1.0);
    let bl = (LEFT_MARGIN as f32 - 1.0, (TOP_MARGIN + spec_height) as f32);
    let br = ((LEFT_MARGIN + spec_width) as f32, (TOP_MARGIN + spec_height) as f32);

    draw_line_segment_mut(&mut image, tl, tr, white);
    draw_line_segment_mut(&mut image, tr, br, white);
    draw_line_segment_mut(&mut image, br, bl, white);
    draw_line_segment_mut(&mut image, bl, tl, white);

    let font_data = include_bytes!("../assets/DejaVuLGCSans.ttf");
    let font = FontVec::try_from_vec(font_data.to_vec()).unwrap();

    let font_normal = PxScale::from(16.0);
    let font_small = PxScale::from(13.0);
    let font_scales = PxScale::from(14.0);
    let text_color = white;

    draw_text_mut(
        &mut image,
        text_color,
        LEFT_MARGIN as i32,
        10,
        font_normal,
        &font,
        filename,
    );

    if show_version {
        let app_info = format!("Spek-rs v{}", env!("CARGO_PKG_VERSION"));
        let (w, _) = imageproc::drawing::text_size(font_small, &font, &app_info);
        draw_text_mut(
            &mut image,
            text_color,
            (final_width - w - 10) as i32,
            5,
            font_small,
            &font,
            &app_info,
        );
    }

    image
}
