// src/amplitude.rs

use super::ppm::PpmWriter;
use audio::wav::read_samples;
use audio::errors::AudioError;

const CHUNK_SIZE:  usize = 512;
const IMG_HEIGHT:  usize = 256;
const PIXEL_WIDTH: usize = 2;

const BLACK: super::color::Rgb = super::color::Rgb(0,   0,   0);
const WHITE: super::color::Rgb = super::color::Rgb(255, 255, 255);
const GREY:  super::color::Rgb = super::color::Rgb(30,  30,  30);

pub fn render(path: &str, output_path: &str) -> Result<(), AudioError> {
    let samples = read_samples(path)?;
    if samples.is_empty() { return Ok(()); }

    let rms_values: Vec<f32> = samples
        .chunks(CHUNK_SIZE)
        .map(|chunk| {
            let sum_sq: f32 = chunk.iter().map(|&s| s * s).sum();
            (sum_sq / chunk.len() as f32).sqrt()
        })
        .collect();

    let global_max = rms_values.iter().cloned().fold(0.0_f32, f32::max);
    if global_max == 0.0 { return Ok(()); }

    let img_width = rms_values.len() * PIXEL_WIDTH;
    let mut ppm   = PpmWriter::new(img_width, IMG_HEIGHT, BLACK);

    let center = IMG_HEIGHT / 2;
    for x in 0..img_width {
        ppm.set(x, center, GREY);
    }

    for (col_idx, &rms) in rms_values.iter().enumerate() {
        let norm      = rms / global_max;           // [0.0, 1.0]
        let half_bar  = (norm * center as f32) as usize;
        let y_top     = center.saturating_sub(half_bar);
        let y_bot     = (center + half_bar).min(IMG_HEIGHT - 1);

        for y in y_top..=y_bot {
            for dx in 0..PIXEL_WIDTH {
                ppm.set(col_idx * PIXEL_WIDTH + dx, y, WHITE);
            }
        }
    }

    ppm.save(output_path)?;
    Ok(())
}