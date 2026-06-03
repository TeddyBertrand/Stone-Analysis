use audio::dsp::fourier::{fft, ifft, Complex};
use audio::wav::header::WavHeader;

use crate::errors::StegoError;
use crate::helpers::{
    calculate_adaptive_magnitude, dft_scaling_factor, mask_chars, modify_sound_spectrum,
    normalize_samples,
};
use crate::write_wav::write_to_wav;
use crate::{CHUNK_SIZE, MAX_HZ, PHASE, SAMPLE_RATE};
fn mask_message(
    file_samples: &Vec<f32>,
    header: &WavHeader,
    output_file: &str,
    given_msg: &str,
) -> Result<(), StegoError> {
    let idft_scale_factor = dft_scaling_factor() as f32;
    let normalized_samples = normalize_samples(file_samples);

    let msg_len = given_msg.len();
    let total_chunks = file_samples.len() / CHUNK_SIZE;
    let max_bytes = total_chunks.saturating_sub(1);
    if msg_len > max_bytes {
        return Err(StegoError::PayloadTooLarge {
            max_bytes,
            requested: msg_len,
        });
    }

    let mut steg_audio = Vec::with_capacity(normalized_samples.len());
    let total_req_win = msg_len + 1;

    for window_idx in 0..total_req_win {
        let start_idx = window_idx * CHUNK_SIZE;
        let end_idx = start_idx + CHUNK_SIZE;
        
        let window_samples = if start_idx < normalized_samples.len() {
            let limit = std::cmp::min(end_idx, normalized_samples.len());
            let mut chunk = normalized_samples[start_idx..limit].to_vec();
            if chunk.len() < CHUNK_SIZE {
                chunk.resize(CHUNK_SIZE, 0.0);
            }
            chunk
        } else {
            vec![0.0; CHUNK_SIZE]
        };

        let complex_window: Vec<Complex> = window_samples
            .iter()
            .map(|&s| Complex { re: s, im: 0.0 })
            .collect();

        let mut freq_samples = fft(&complex_window);

        let target_amp = 0.3;
        let inject_magni = calculate_adaptive_magnitude(&window_samples, CHUNK_SIZE, target_amp);

        if window_idx == 0 {
            let target_hz = MAX_HZ + (msg_len as f64 * (SAMPLE_RATE / CHUNK_SIZE as f64));
            modify_sound_spectrum(
                &mut freq_samples,
                target_hz,
                SAMPLE_RATE,
                inject_magni,
                PHASE,
            )?;
        } else {
            let target_hz = mask_chars(given_msg, window_idx);
            modify_sound_spectrum(
                &mut freq_samples,
                target_hz,
                SAMPLE_RATE,
                inject_magni,
                PHASE,
            )?;
        }

        let recon_audio_chunk = ifft(&freq_samples);

        let processed_samples: Vec<f32> = recon_audio_chunk
            .iter()
            .map(|c| c.re / idft_scale_factor)
            .collect();

        steg_audio.extend(processed_samples);
    }

    let processed_bytes = total_req_win * CHUNK_SIZE;
    if normalized_samples.len() > processed_bytes {
        steg_audio.extend_from_slice(&normalized_samples[processed_bytes..]);
    }

    steg_audio.truncate(file_samples.len());

    write_to_wav(&steg_audio, output_file, &header)?;

    Ok(())
}

pub fn run_encryption(
    input_file: &str,
    output_file: &str,
    given_msg: &str,
) -> Result<(), StegoError> {
    let (header, samples) =
        audio::wav::reader::read_wav(input_file).map_err(|e| StegoError::FailedToReadAudio {
            details: e.to_string(),
        })?;
    mask_message(&samples, &header, output_file, given_msg)
}
