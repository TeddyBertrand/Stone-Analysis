use std::fmt;

#[derive(Debug)]
pub enum CliError {
    BadArgument(String),
    MissingRequiredOption(String),
}

impl fmt::Display for CliError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            CliError::BadArgument(arg) => write!(f, "Argument invalide : {}", arg),
            CliError::MissingRequiredOption(opt) => {
                write!(f, "Option manquante obligatoire : {}", opt)
            }
        }
    }
}

impl std::error::Error for CliError {}
