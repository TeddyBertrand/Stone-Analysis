pub mod args;
pub mod help;
pub mod lexer;
pub mod mode;
pub mod validator;

use self::args::ParsedArgs;
use self::help::print_help;
use self::lexer::lex;
use self::validator::{resolve_mode, validate_positionals};
use crate::errors::CliError;

pub fn parse_args() -> Result<ParsedArgs, CliError> {
    let raw: Vec<String> = std::env::args().collect();

    if raw.len() <= 1 {
        print_help();
        return Err(CliError::BadArgument("Aucun argument fourni".into()));
    }

    let lex_result = lex(&raw[1..])?;
    let mode = resolve_mode(&lex_result.flags)?;
    validate_positionals(mode, &lex_result.positionals)?;

    Ok(ParsedArgs {
        flags: lex_result.flags,
        positionals: lex_result.positionals,
        mode,
    })
}
