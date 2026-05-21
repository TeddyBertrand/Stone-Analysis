use std::fmt;

#[derive(Debug)]
pub enum AppError {
    Audio(audio::AudioError),
    Stego(stego::StegoError),
    Cli(stone_cli::CliError),
}

impl fmt::Display for AppError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            AppError::Audio(e) => write!(f, "[Erreur Pipeline Audio] {}", e),
            AppError::Stego(e) => write!(f, "[Erreur Stéganographie] {}", e),
            AppError::Cli(e) => write!(f, "{}", e),
        }
    }
}

impl std::error::Error for AppError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            AppError::Audio(e) => Some(e),
            AppError::Stego(e) => Some(e),
            AppError::Cli(e) => Some(e),
        }
    }
}

impl From<audio::AudioError> for AppError {
    fn from(err: audio::AudioError) -> Self { AppError::Audio(err) }
}

impl From<stego::StegoError> for AppError {
    fn from(err: stego::StegoError) -> Self { AppError::Stego(err) }
}

impl From<stone_cli::CliError> for AppError {
    fn from(err: stone_cli::CliError) -> Self { AppError::Cli(err) }
}