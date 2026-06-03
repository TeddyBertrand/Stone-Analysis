pub mod dsp;
pub mod errors;
pub mod wav;

pub use dsp::analysis::run;
pub use errors::AudioError;
pub type Result<T> = std::result::Result<T, AudioError>;

pub const SAMPLE_NORMALIZER: f32 = 32768.0;
