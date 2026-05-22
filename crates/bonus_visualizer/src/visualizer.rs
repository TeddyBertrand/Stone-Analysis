use audio::errors::AudioError;
use crate::amplitude;
use crate::frequency;

pub fn run(path: &str, mode: &str, output_path: &str) -> Result<(), AudioError> {
    match mode {
        "amplitude" => amplitude::render(path, output_path),
        "frequency" => frequency::render(path, output_path),
        _ => {
            eprintln!("Mode inconnu : '{}'. Disponibles : amplitude, frequency", mode);
            Ok(())
        }
    }
}