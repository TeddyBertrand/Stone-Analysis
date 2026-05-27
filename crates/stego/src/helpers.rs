
use audio::dsp::fourier::{dft, idft};
use audio::dsp::fourier::{Complex};
use crate::errors::StegoError;

use crate::{CHUNK_SIZE, SAMPLE_RATE, OBS_KEY, MAX_HZ};



pub fn mask_chars(given_msg: &str, window_idx: usize) ->  f64
{
    let bchar = given_msg.as_bytes()[window_idx - 1];
    let uppercase_char = bchar.to_ascii_uppercase();

    let mask_char = uppercase_char ^ (OBS_KEY ^(window_idx as u8));
    let target_hz = MAX_HZ + (mask_char as f64 * (SAMPLE_RATE / CHUNK_SIZE as f64));

    return target_hz;
}

pub fn modify_sound_spectrum(freq_samples: &mut Vec<Complex>, target_hz: f64, sample_rate: f64, magnitude: f32, phase: f32) -> Result<(), StegoError>
{
    let n = freq_samples.len();
    let target_bin = (target_hz * n as f64 / sample_rate).round() as usize;

    if target_bin >= n {
        return Err(StegoError::FrequencyOutOfBounds { hz_rate: target_hz as f32 });
    }

    freq_samples[target_bin].re += magnitude * phase.cos();
    freq_samples[target_bin].im += magnitude * phase.sin();

    if target_bin > 0 && target_bin < n {
        let conjugate_bin = n - target_bin;
        freq_samples[conjugate_bin].re += magnitude * phase.cos();
        freq_samples[conjugate_bin].im -= magnitude * phase.sin();
    }

    Ok(())
}

pub fn dft_scaling_factor() -> f32
{
    let calibration_chunk = vec![0.5f32; CHUNK_SIZE];
    let cal_dft = dft(&calibration_chunk);
    let cal_idft = idft(&cal_dft);
    let cal_peak = cal_idft.iter().map(|s| s.abs()).fold(0.0f32, |m, x| m.max(x));
    let idft_scale_factor = if cal_peak > 0.001 { cal_peak / 0.5 } else { 1.0 };

    return idft_scale_factor;
}

pub fn normalize_samples(samples: &Vec<f32>) -> Vec<f32> {
    let max_peak = samples.iter().map(|s| s.abs()).fold(0.0f32, |m, x| m.max(x));
    if max_peak > 1.0 {
        samples.iter().map(|&s| s / 32768.0).collect()
    } else {
        samples.clone()
    }
}
pub fn calculate_adaptive_magnitude(chunk: &[f32], chunk_size: usize, target_time_amplitude: f32) -> f32 
{
    let peak_amplitude = chunk.iter()
        .map(|&sample| sample.abs())
        .fold(0.0f32, |max, val| if val > max { val } else { max });

    let headroom = 1.0 - peak_amplitude;


    let safe_target_amplitude = if target_time_amplitude > headroom {
        (headroom - 0.01).max(0.005) // Fallback to a tiny but valid floor if chunk is slammed
    } else {
        target_time_amplitude
    };

    // Formula: (Amplitude * N) / 2
    let ideal_bin_magnitude = (safe_target_amplitude * chunk_size as f32) / 2.0;

    ideal_bin_magnitude
}