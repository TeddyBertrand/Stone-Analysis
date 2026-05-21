use crate::errors::CliError;
use super::mode::{Mode, ARG_DEFS};
use std::collections::HashMap;

pub fn resolve_mode(flags: &HashMap<&'static str, String>) -> Result<Mode, CliError> {
    let active: Vec<Mode> = ARG_DEFS
        .iter()
        .filter_map(|d| d.mode.filter(|_| flags.contains_key(d.long)))
        .collect();

    match active.as_slice() {
        [] => Err(CliError::BadArgument(build_mode_list_error())),
        [mode] => Ok(*mode),
        _ => Err(CliError::BadArgument(
            "Les modes sont exclusifs — choisis-en un seul.".into(),
        )),
    }
}

pub fn validate_positionals(mode: Mode, positionals: &[String]) -> Result<(), CliError> {
    let expected = mode.expected_positionals();
    if positionals.len() != expected {
        return Err(CliError::BadArgument(format!(
            "Le mode {:?} attend {} argument(s) positionnels ({}) — reçu : {}.",
            mode,
            expected,
            mode.positional_hint(),
            positionals.len(),
        )));
    }
    Ok(())
}

fn build_mode_list_error() -> String {
    let modes: Vec<&str> = ARG_DEFS
        .iter()
        .filter(|d| d.mode.is_some())
        .map(|d| d.long)
        .collect();
    format!(
        "Spécifie un mode : {}",
        modes.join(", ")
    )
}
