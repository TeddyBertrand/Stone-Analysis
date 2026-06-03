use super::color::{heat_colormap, Rgb};
use audio::dsp::fourier::dft;
use audio::errors::AudioError;
use audio::wav::read_samples;

const CHUNK_SIZE: usize = 2048;
const NUM_FREQ_BINS: usize = 256;
const PIXEL_WIDTH: usize = 2;

pub fn render(path: &str, output_path: &str) -> Result<(), AudioError> {
    let samples = read_samples(path)?;
    let half_size = CHUNK_SIZE / 2;
    let bin_size = half_size / NUM_FREQ_BINS;

    let grid: Vec<Vec<f32>> = samples
        .chunks_exact(CHUNK_SIZE)
        .map(|chunk| {
            let windowed: Vec<f32> = chunk
                .iter()
                .enumerate()
                .map(|(i, &s)| {
                    let hann = 0.5
                        * (1.0
                            - (2.0 * std::f32::consts::PI * i as f32 / (CHUNK_SIZE - 1) as f32)
                                .cos());
                    s * hann
                })
                .collect();
            let spectrum = dft(&windowed);
            (0..NUM_FREQ_BINS)
                .map(|b| {
                    (0..bin_size)
                        .map(|i| spectrum[b * bin_size + i].magnitude())
                        .sum::<f32>()
                        / bin_size as f32
                })
                .collect()
        })
        .collect();

    if grid.is_empty() {
        return Ok(());
    }

    let max_mag = grid
        .iter()
        .flat_map(|c| c.iter())
        .cloned()
        .fold(f32::NEG_INFINITY, f32::max);
    if max_mag == 0.0 {
        return Ok(());
    }

    let img_width = grid.len() * PIXEL_WIDTH;

    use std::fs::File;
    use std::io::{BufWriter, Write};
    let file = File::create(output_path).map_err(|_| AudioError::EmptySignal)?;
    let mut w = BufWriter::new(file);
    write!(w, "P6\n{} {}\n255\n", img_width, NUM_FREQ_BINS).unwrap();

    for y in (0..NUM_FREQ_BINS).rev() {
        for x in 0..grid.len() {
            let norm = (grid[x][y] / max_mag).max(1e-9);
            let intensity = (1.0_f32 + norm.log10() / 4.0).clamp(0.0, 1.0);
            let Rgb(r, g, b) = heat_colormap(intensity);
            for _ in 0..PIXEL_WIDTH {
                w.write_all(&[r, g, b]).unwrap();
            }
        }
    }

    w.flush().unwrap();
    Ok(())
}
