// crates/stego/src/errors.rs
use std::fmt;

#[derive(Debug)]
pub enum StegoError {
    PayloadTooLarge { max_bytes: usize, requested: usize },
    NoPayloadFound,
    CorruptedMagicBytes,
}

impl fmt::Display for StegoError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            StegoError::PayloadTooLarge { max_bytes, requested } => {
                write!(f, "Le message est trop grand pour ce WAV (Max: {} octets, Demandé: {})", max_bytes, requested)
            }
            StegoError::NoPayloadFound => write!(f, "Aucun message caché détecté dans ce fichier"),
            StegoError::CorruptedMagicBytes => write!(f, "Signature de stéganographie invalide"),
        }
    }
}

impl std::error::Error for StegoError {}