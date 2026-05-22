// crates/audio/src/errors.rs
use std::fmt;

#[derive(Debug)]
pub enum AudioError {
    FileNotFound(String),
    InvalidWavHeader(String),
    UnsupportedSampleFormat(String),
    DftLengthMismatch { expected: usize, got: usize },
    IoError(String),
    EmptySignal,
}

impl fmt::Display for AudioError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            AudioError::FileNotFound(path) => write!(f, "Fichier audio introuvable : {}", path),
            AudioError::InvalidWavHeader(msg) => write!(f, "Header WAV corrompu : {}", msg),
            AudioError::UnsupportedSampleFormat(fmt) => {
                write!(f, "Format d'échantillon non supporté : {}", fmt)
            }
            AudioError::DftLengthMismatch { expected, got } => {
                write!(
                    f,
                    "Taille DFT incorrecte (Attendu: {}, Obtenu: {})",
                    expected, got
                )
            }
            AudioError::IoError(msg) => write!(f, "Erreur système d'E/S : {}", msg),
            AudioError::EmptySignal => write!(f, "Le signal audio fourni est vide"),
        }
    }
}

impl std::error::Error for AudioError {}

impl From<std::io::Error> for AudioError {
    fn from(err: std::io::Error) -> Self {
        AudioError::IoError(err.to_string())
    }
}
