pub mod errors;
pub mod wav;
pub mod dsp;

pub use errors::AudioError;
pub use dsp::analysis::run;
pub type Result<T> = std::result::Result<T, AudioError>;