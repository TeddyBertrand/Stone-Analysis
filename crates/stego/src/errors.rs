// crates/stego/src/errors.rs
use std::fmt;

use audio::errors::AudioError;

#[derive(Debug)]
pub enum StegoError {
    PayloadTooLarge { max_bytes: usize, requested: usize },
    NoPayloadFound,
    CorruptedMagicBytes,
    FrequencyOutOfBounds { hz_rate: f32},
    FailedToWriteEncryptedAudio { details: String },
    FailedToReadAudio { details: String },
    FailedToDecodeMessage { details: String },
}

impl fmt::Display for StegoError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            StegoError::PayloadTooLarge {
                max_bytes,
                requested,
            } => {
                write!(
                    f,
                    "Le message est trop grand pour ce WAV (Max: {} octets, Demandé: {})",
                    max_bytes, requested
                )
            }
            StegoError::NoPayloadFound => write!(f, "Aucun message caché détecté dans ce fichier"),
            StegoError::CorruptedMagicBytes => write!(f, "Signature de stéganographie invalide"),
            StegoError::FrequencyOutOfBounds { hz_rate } => write!(f, "Warning: Frequency {} Hz is out of bounds for window size", hz_rate),
            StegoError::FailedToWriteEncryptedAudio { details } => write!(f, "Failed to write encrypted audio: {}", details),
            StegoError::FailedToReadAudio { details } => write!(f, "Failed to read audio: {}", details),
            StegoError::FailedToDecodeMessage { details } => write!(f, "Failed to decode message: {}", details),
        }
    }
}

impl std::error::Error for StegoError {}

impl From<std::io::Error> for StegoError {
    fn from(err: std::io::Error) -> Self {
        StegoError::FailedToWriteEncryptedAudio { details: err.to_string() }
    }
}

impl From<AudioError> for StegoError {
    fn from(err: AudioError) -> Self {
        StegoError::FailedToWriteEncryptedAudio { details: err.to_string() };
        StegoError::FailedToReadAudio { details: err.to_string() }
    }
}