use crate::errors::AudioError;

#[allow(dead_code)]
pub struct WavHeader {
    pub channels: u16,
    pub sample_rate: u32,
    pub bits_per_sample: u16,
    pub data_size: u32,
    pub raw_bytes: [u8; 44],
}

impl WavHeader {
    pub fn from_bytes(bytes: &[u8]) -> Result<Self, AudioError> {
        if bytes.len() < 44 {
            return Err(AudioError::InvalidWavHeader(
                "En-tête WAV trop court (min 44 octets).".to_string(),
            ));
        }

        if &bytes[0..4] != b"RIFF" || &bytes[8..12] != b"WAVE" {
            return Err(AudioError::InvalidWavHeader(
                "Le fichier n'est pas un conteneur RIFF/WAVE valide.".to_string(),
            ));
        }

        let channels = u16::from_le_bytes([bytes[22], bytes[23]]);
        let sample_rate = u32::from_le_bytes([bytes[24], bytes[25], bytes[26], bytes[27]]);
        let bits_per_sample = u16::from_le_bytes([bytes[34], bytes[35]]);
        let data_size = u32::from_le_bytes([bytes[40], bytes[41], bytes[42], bytes[43]]);

        if channels != 1 {
            return Err(AudioError::UnsupportedSampleFormat(format!(
                "Attendu: Mono (1 canal), Obtenu: {} canaux",
                channels
            )));
        }
        if sample_rate != 48000 {
            return Err(AudioError::UnsupportedSampleFormat(format!(
                "Attendu: 48000 Hz, Obtenu: {} Hz",
                sample_rate
            )));
        }
        if bits_per_sample != 16 {
            return Err(AudioError::UnsupportedSampleFormat(format!(
                "Attendu: 16-bit PCM, Obtenu: {}-bit",
                bits_per_sample
            )));
        }

        let mut raw_bytes = [0u8; 44];
        raw_bytes.copy_from_slice(&bytes[0..44]);

        Ok(Self {
            channels,
            sample_rate,
            bits_per_sample,
            data_size,
            raw_bytes,
        })
    }
}
