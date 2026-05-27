use super::header::WavHeader;
use crate::errors::AudioError;
use std::fs::File;
use std::io::Read;

use crate::SAMPLE_NORMALIZER;

fn convert_to_samples(raw_data: &[u8]) -> Result<Vec<f32>, AudioError> {
    if raw_data.is_empty() {
        return Err(AudioError::EmptySignal);
    }

    let num_samples = raw_data.len() / 2;
    let mut samples = Vec::with_capacity(num_samples);

    for chunk in raw_data.chunks_exact(2) {
        let sample_i16 = i16::from_le_bytes([chunk[0], chunk[1]]);
        samples.push(sample_i16 as f32 / SAMPLE_NORMALIZER);
    }

    Ok(samples)
}

fn load_file_buffer(path: &str) -> Result<Vec<u8>, AudioError> {
    let mut file = File::open(path)?;
    let mut buffer = Vec::new();
    file.read_to_end(&mut buffer)?;
    Ok(buffer)
}

pub fn read_samples(path: &str) -> Result<Vec<f32>, AudioError> {
    let buffer = load_file_buffer(path)?;
    WavHeader::from_bytes(&buffer)?;
    let samples = convert_to_samples(&buffer[44..])?;
    Ok(samples)
}

#[allow(dead_code)]
pub fn read_header(path: &str) -> Result<WavHeader, AudioError> {
    let buffer = load_file_buffer(path)?;
    WavHeader::from_bytes(&buffer)
}

#[allow(dead_code)]
pub fn read_wav(path: &str) -> Result<(WavHeader, Vec<f32>), AudioError> {
    let buffer = load_file_buffer(path)?;
    let header = WavHeader::from_bytes(&buffer)?;
    let samples = convert_to_samples(&buffer[44..])?;
    Ok((header, samples))
}
