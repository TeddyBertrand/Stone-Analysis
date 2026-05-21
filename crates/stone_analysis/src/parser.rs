#![allow(dead_code)]

use crate::errors::CliError;
use std::collections::HashMap;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ArgKind {
    Flag,
    Value,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Mode {
    Analyze,
    Cypher,
    Decypher,
    Help,
}

impl Mode {
    const fn expected_positionals(self) -> usize {
        match self {
            Mode::Analyze  => 2,
            Mode::Cypher   => 3,
            Mode::Decypher => 1,
            Mode::Help      => 0,
        }
    }

    const fn positional_hint(self) -> &'static str {
        match self {
            Mode::Analyze  => "IN_FILE N",
            Mode::Cypher   => "IN_FILE OUT_FILE MESSAGE",
            Mode::Decypher => "IN_FILE",
            Mode::Help     => "",
        }
    }
}

pub struct ArgDef {
    pub short: &'static str,
    pub long:  &'static str,
    pub kind:  ArgKind,
    pub mode:  Option<Mode>,
    pub help:  &'static str,
}       

pub const ARG_DEFS: &[ArgDef] = &[
    ArgDef { short: "-a", long: "--analyze",  kind: ArgKind::Flag,  mode: Some(Mode::Analyze),  help: "Analyse un fichier de runes (IN_FILE N)"              },
    ArgDef { short: "-c", long: "--cypher",   kind: ArgKind::Flag,  mode: Some(Mode::Cypher),   help: "Chiffre un message dans une image (IN_FILE OUT_FILE MESSAGE)" },
    ArgDef { short: "-d", long: "--decypher", kind: ArgKind::Flag,  mode: Some(Mode::Decypher), help: "Déchiffre un message caché (IN_FILE)"                 },
    ArgDef { short: "-h", long: "--help",      kind: ArgKind::Flag,  mode: Some(Mode::Help),      help: "Affiche l'aide"          },
];

#[derive(Debug)]
pub struct ParsedArgs {
    flags:          HashMap<&'static str, String>,
    positionals:    Vec<String>,
    pub mode:       Mode,
}

impl ParsedArgs {
    pub fn value(&self, long: &'static str) -> Option<&str> {
        self.flags.get(long).map(String::as_str)
    }

    pub fn has(&self, long: &'static str) -> bool {
        self.flags.contains_key(long)
    }

    pub fn positional(&self, index: usize) -> Option<&str> {
        self.positionals.get(index).map(String::as_str)
    }

    pub fn positionals(&self) -> &[String] {
        &self.positionals
    }
}


pub fn parse_args() -> Result<ParsedArgs, CliError> {
    let raw: Vec<String> = std::env::args().collect();

    if raw.len() <= 1 {
        print_help();
        return Err(CliError::BadArgument("Aucun argument fourni".into()));
    }

    let (flags, positionals) = lex(&raw[1..])?;
    let mode = resolve_mode(&flags)?;
    validate_positionals(mode, &positionals)?;

    Ok(ParsedArgs { flags, positionals, mode })
}

fn lex(raw: &[String]) -> Result<(HashMap<&'static str, String>, Vec<String>), CliError> {
    let mut flags: HashMap<&'static str, String> = HashMap::new();
    let mut positionals: Vec<String> = Vec::new();
    let mut i = 0;

    while i < raw.len() {
        let arg = &raw[i];

        if looks_like_option(arg) {
            let key = key_part(arg);
            let def = find_def(key)
                .ok_or_else(|| CliError::BadArgument(format!("Option inconnue : {arg}")))?;

            match def.kind {
                ArgKind::Flag => {
                    flags.insert(def.long, "true".into());
                }
                ArgKind::Value => {
                    let val = if let Some(eq) = arg.find('=') {
                        arg[eq + 1..].to_string()
                    } else if let Some(next) = raw.get(i + 1).filter(|s| !looks_like_option(s)) {
                        i += 1;
                        next.clone()
                    } else {
                        return Err(CliError::MissingRequiredOption(
                            format!("Valeur manquante pour {key}"),
                        ));
                    };
                    flags.insert(def.long, val);
                }
            }
        } else {
            positionals.push(arg.clone());
        }

        i += 1;
    }

    Ok((flags, positionals))
}

fn resolve_mode(flags: &HashMap<&'static str, String>) -> Result<Mode, CliError> {
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

fn validate_positionals(mode: Mode, positionals: &[String]) -> Result<(), CliError> {
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

pub fn print_help() {
    println!("Usage : stone_analysis [MODE] [OPTIONS] [ARGS...]\n");
    println!("Modes :");
    for def in ARG_DEFS.iter().filter(|d| d.mode.is_some()) {
        println!("  {:<4}  {:<14}  {}", def.short, def.long, def.help);
    }
    let opts: Vec<_> = ARG_DEFS.iter().filter(|d| d.mode.is_none()).collect();
    if !opts.is_empty() {
        println!("\nOptions :");
        for def in opts {
            println!("  {:<4}  {:<14}  {}", def.short, def.long, def.help);
        }
    }
}