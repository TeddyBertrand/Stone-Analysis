
use audio::dsp::fourier::{Complex, dft};

use crate::errors::StegoError;
use crate::helpers::normalize_samples;
use crate::{CHUNK_SIZE, SAMPLE_RATE, OBS_KEY};


fn decode_secret_from_windows(decoded_chars: &mut Vec<u8>, freq_samples: &Vec<Complex>, base_k: usize, window_idx: usize)
{
    let mut max_mag = 0.0;
    let mut best_k = base_k;

    for k in base_k..(base_k + 256) {
        if k >= freq_samples.len() { break; }
        let mag = f32::sqrt((freq_samples[k].re * freq_samples[k].re) + (freq_samples[k].im * freq_samples[k].im));
        if mag > max_mag {
            max_mag = mag;
            best_k = k;
        }
    }

    if max_mag > 0.1 {
        let bin_offset = (best_k as isize - base_k as isize) as i32;
        let scrambled_byte = bin_offset as u8;
        let clean_byte = scrambled_byte ^ (OBS_KEY ^ (window_idx as u8));
        decoded_chars.push(clean_byte);
    }
}

fn exctract_header_len(freq_samples: &Vec<Complex>, base_k: usize, msg_len: &mut usize)
{
    let mut diag_max = 0.0;
    let mut diag_k = base_k;
            
    for k in base_k..1024 { // Protect Nyquist limit
        if k >= freq_samples.len() { break; }
        let mag = f32::sqrt((freq_samples[k].re * freq_samples[k].re) + (freq_samples[k].im * freq_samples[k].im));
        if mag > diag_max {
            diag_max = mag;
            diag_k = k;
        }
    }

    *msg_len = (diag_k as isize - base_k as isize) as usize;   
}

fn unmask_message(file_samples: Vec<f32>) -> Result<(), StegoError>
{
    if file_samples.is_empty() {
        return Ok(());
    }

    let normalized_samples = normalize_samples(&file_samples);
    let mut decoded_chars = Vec::new();
    let mut msg_len = 0;
    let mut window_idx = 0;

    let base_k = ((20000.0 * CHUNK_SIZE as f64) / SAMPLE_RATE).round() as usize;

    for chunk in normalized_samples.chunks(CHUNK_SIZE) {
        let window_samples = chunk.to_vec();
        if window_samples.len() < CHUNK_SIZE { break; }

        let freq_samples = dft(&window_samples);
        
        if window_idx == 0 { exctract_header_len(&freq_samples, base_k, &mut msg_len); }

        if window_idx > 0 && window_idx <= msg_len { decode_secret_from_windows(&mut decoded_chars, &freq_samples, base_k, window_idx); }

        window_idx += 1;

        if window_idx > msg_len && msg_len > 0 { break; }
    }

    if let Ok(secret_string) = String::from_utf8(decoded_chars) { println!("{}", secret_string); } 
    else { return Err(StegoError::FailedToDecodeMessage { details: "Decoded bytes are not valid UTF-8".to_string() });}

    Ok(())
}

pub fn run_decryption(input_file: &str) -> Result<(), StegoError>
{
    let (_header, samples) = audio::wav::reader::read_wav(input_file)
        .map_err(|e| StegoError::FailedToReadAudio { details: e.to_string() })?;
    
    unmask_message(samples)
}