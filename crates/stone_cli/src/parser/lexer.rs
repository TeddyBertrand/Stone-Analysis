use super::mode::{ArgDef, ArgKind, ARG_DEFS};
use crate::errors::CliError;
use std::collections::HashMap;

pub struct LexResult {
    pub flags: HashMap<&'static str, String>,
    pub positionals: Vec<String>,
}

pub fn lex(raw: &[String]) -> Result<LexResult, CliError> {
    let mut flags = HashMap::new();
    let mut positionals = Vec::new();

    let mut args_iter = raw.iter().peekable();

    while let Some(arg) = args_iter.next() {
        if looks_like_option(arg) {
            let (long_name, value) = parse_option(arg, &mut args_iter)?;
            flags.insert(long_name, value);
        } else {
            positionals.push(arg.clone());
        }
    }

    Ok(LexResult { flags, positionals })
}

fn parse_option<'a, I>(arg: &str, iter: &mut I) -> Result<(&'static str, String), CliError>
where
    I: Iterator<Item = &'a String>,
{
    let key = key_part(arg);
    let def =
        find_def(key).ok_or_else(|| CliError::BadArgument(format!("Option inconnue : {arg}")))?;

    let value = match def.kind {
        ArgKind::Flag => "true".into(),
        ArgKind::Value => extract_value(arg, key, iter)?,
    };

    Ok((def.long, value))
}

fn extract_value<'a, I>(arg: &str, key: &str, iter: &mut I) -> Result<String, CliError>
where
    I: Iterator<Item = &'a String>,
{
    if let Some(eq) = arg.find('=') {
        return Ok(arg[eq + 1..].to_string());
    }

    if let Some(next) = iter.next() {
        if !looks_like_option(next) {
            return Ok(next.clone());
        }
    }

    Err(CliError::MissingRequiredOption(format!(
        "Valeur manquante pour {key}"
    )))
}

#[inline]
fn looks_like_option(arg: &str) -> bool {
    arg.starts_with('-')
        && arg.len() > 1
        && !arg.chars().nth(1).map_or(false, |c| c.is_ascii_digit())
}

#[inline]
fn key_part(arg: &str) -> &str {
    arg.find('=').map_or(arg, |pos| &arg[..pos])
}

#[inline]
fn find_def(key: &str) -> Option<&'static ArgDef> {
    ARG_DEFS.iter().find(|d| d.short == key || d.long == key)
}
