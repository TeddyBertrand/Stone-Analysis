use super::fourier::{dft, Complex};
use crate::wav::read_samples;
use crate::errors::AudioError;

const SAMPLE_RATE: f32 = 48000.0;
pub const CHUNK_SIZE: usize = 2048;

pub struct FrequencyResult {
    pub hz: f32,
    pub magnitude: f32,
}

pub fn run(path: &str, n: usize) -> Result<(), AudioError> {
    let samples = read_samples(path)?;
    let top_frequencies = analyze_spectrogram(&samples, n)?;
    
    print_results(&top_frequencies);
    Ok(())
}

pub fn print_results(top_frequencies: &[FrequencyResult]) {
    println!("Top {} frequencies:", top_frequencies.len());
    for freq in top_frequencies {
        println!("{:.1} Hz", freq.hz);
    }
}

fn analyze_spectrogram(samples: &[f32], n: usize) -> Result<Vec<FrequencyResult>, AudioError> {
    let half_size = CHUNK_SIZE / 2;
    let mut accumulated_magnitudes = vec![0.0f32; half_size];
    let mut block_count = 0;

    for chunk in samples.chunks_exact(CHUNK_SIZE) {
        let spectrum = dft(chunk);
        
        for i in 0..half_size {
            accumulated_magnitudes[i] += spectrum[i].magnitude();
        }
        block_count += 1;
    }

    if block_count == 0 {
        return Err(AudioError::EmptySignal);
    }

    let mut average_spectrum = Vec::with_capacity(half_size);
    for i in 0..half_size {
        let avg_mag = accumulated_magnitudes[i] / block_count as f32;
        
        average_spectrum.push(Complex {
            re: avg_mag,
            im: 0.0,
        });
    }

    Ok(find_top_n(&average_spectrum, SAMPLE_RATE, n))
}

pub fn find_top_n(spectrum: &[Complex], sample_rate: f32, n: usize) -> Vec<FrequencyResult> {
    let size = spectrum.len();
    let mut results = Vec::with_capacity(size);

    for i in 0..size {
        let hz = (i as f32 * sample_rate) / CHUNK_SIZE as f32;
        let mag = spectrum[i].magnitude();

        results.push(FrequencyResult { hz, magnitude: mag });
    }
    
    results.sort_by(|a, b| b.magnitude.partial_cmp(&a.magnitude).unwrap());
    results.truncate(n);

    return results;
}