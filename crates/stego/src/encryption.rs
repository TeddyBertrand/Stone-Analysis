
use audio::dsp::fourier::{dft, idft};
use audio::wav::header::WavHeader;

use crate::errors::StegoError;
use crate::helpers::{calculate_adaptive_magnitude, normalize_samples, dft_scaling_factor, modify_sound_spectrum, mask_chars};
use crate::write_wav::write_to_wav;
use crate::{CHUNK_SIZE, SAMPLE_RATE, PHASE, MAX_HZ};

fn mask_message(file_samples: &Vec<f32>, header: &WavHeader, output_file: &str, given_msg: &str) -> Result<(), StegoError>
{
    let idft_scale_factor = dft_scaling_factor();
    let normalized_samples = normalize_samples(file_samples);

    let mut steg_audio: Vec<f32> = Vec::new();
    let msg_len = given_msg.len();
    let mut window_idx = 0;

    let mut fchunks_iter = normalized_samples.chunks(CHUNK_SIZE);
    let total_req_win = msg_len + 1;

    while window_idx < total_req_win || fchunks_iter.len() > 0 {

        let mut window_samples = match fchunks_iter.next() {
            Some(fchunk) => fchunk.to_vec(),
            None => vec![0.0; CHUNK_SIZE],
        };

        if window_samples.len() < CHUNK_SIZE { window_samples.resize(CHUNK_SIZE, 0.0); }

        let mut freq_samples = dft(window_samples.as_slice()); 
    
        let target_amp = 0.3;
        let inject_magni = calculate_adaptive_magnitude(&window_samples, CHUNK_SIZE, target_amp);

        if window_idx == 0 {
            let target_hz = MAX_HZ + ( msg_len as f64 * (SAMPLE_RATE / CHUNK_SIZE as f64));
            modify_sound_spectrum(&mut freq_samples, target_hz, SAMPLE_RATE, inject_magni, PHASE)?;
        } else if window_idx <= msg_len {
            let target_hz = mask_chars(given_msg, window_idx);
            modify_sound_spectrum(&mut freq_samples, target_hz, SAMPLE_RATE, inject_magni, PHASE)?;
        }

        let mut recon_audio_chunk = idft(&freq_samples);

        for sample in recon_audio_chunk.iter_mut() { *sample /= idft_scale_factor;}
    
        steg_audio.extend(recon_audio_chunk);

        window_idx += 1;
        if window_idx >= total_req_win && fchunks_iter.len() == 0 { break; }
    }

    write_to_wav(&steg_audio, output_file, &header)?;

    Ok(())
}

pub fn run_encryption(input_file: &str, output_file: &str, given_msg: &str) -> Result<(), StegoError>
{
    let (header, samples) = audio::wav::reader::read_wav(input_file).map_err(|e| StegoError::FailedToReadAudio { details: e.to_string() })?;
    mask_message(&samples, &header, output_file, given_msg)
}