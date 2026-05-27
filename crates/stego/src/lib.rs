
pub const CHUNK_SIZE: usize = 2048;
pub const SAMPLE_RATE: f64 = 48000.0;
pub const PHASE: f32 = 0.0;
pub const OBS_KEY: u8 = 0x42;
pub const MAX_HZ: f64 = 20000.0;

pub mod errors;
pub use errors::StegoError;

pub mod encryption;
pub use encryption::run_encryption;

pub mod decryption;
pub use decryption::run_decryption;

pub mod write_wav;
pub use write_wav::write_to_wav;

pub mod helpers;
