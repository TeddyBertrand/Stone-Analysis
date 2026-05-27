
use crate::errors::StegoError;
use audio::wav::header::WavHeader;

use std::fs::File;
use std::io::Write;







pub fn write_to_wav(recon_freq: &[f32], file_name: &str, header: &WavHeader) -> Result<(), StegoError>
{
    let num_samples = recon_freq.len();
    let data_size = (num_samples * 2) as u32;
    let riff_size = 36 + data_size;

    let mut file = File::create(file_name)?;
    let mut clean_header_bytes = header.raw_bytes;

    clean_header_bytes[4..8].copy_from_slice(&riff_size.to_le_bytes());
    clean_header_bytes[40..44].copy_from_slice(&data_size.to_le_bytes());

    file.write_all(&clean_header_bytes)?;

    for &sample in recon_freq {
        let clamped = sample.clamp(-1.0, 1.0);
        let int_sample = (clamped * 32767.0) as i16;
        file.write_all(&int_sample.to_le_bytes())?;
    }

    println!("Success! Created '{}' ({} samples).", file_name, num_samples);
    Ok(())
}