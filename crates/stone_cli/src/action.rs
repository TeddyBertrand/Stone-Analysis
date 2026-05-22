use crate::errors::CliError;
use crate::parser::mode::Mode;
use crate::parser::args::ParsedArgs;
use crate::parser;

pub enum Action {
    Analyze  { file: String, n: usize },
    Cypher   { input: String, output: String, message: String },
    Decypher { input: String },
    Help     {},
    Visualize { file: String, output: String, mode: String },
}

impl Action {
    pub fn from_env() -> Result<Self, CliError> {
        Self::from_parsed(parser::parse_args()?)
    }

    pub fn from_parsed(args: ParsedArgs) -> Result<Self, CliError> {
        let mut pos = args.positionals().into_iter();

        let mut next_arg = || pos.next().cloned().ok_or(CliError::MissingRequiredOption(
            "Argument positionnel manquant".into()
        ));

        match args.mode {
            Mode::Analyze => {
                let file = next_arg()?;
                
                let n = next_arg()?
                    .parse::<usize>()
                    .map_err(|_| CliError::BadArgument(
                        "L'argument doit être un entier positif valide".into()
                    ))?;
                Ok(Action::Analyze { file, n })
            },

            Mode::Cypher => Ok(Action::Cypher {
                input:   next_arg()?,
                output:  next_arg()?,
                message: next_arg()?,
            }),

            Mode::Decypher => Ok(Action::Decypher {
                input: next_arg()?,
            }),

            Mode::Help => Ok(Action::Help {}),
            Mode::Visualize => Ok(Action::Visualize {
                file: next_arg()?,
                output: next_arg()?,
                mode: next_arg()?,
            }),
        }
    }
}