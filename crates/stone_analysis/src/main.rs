mod errors;

fn main() {
    if let Err(e) = run() {
        eprintln!("\x1b[31merr\x1b[0m {e}");
        std::process::exit(1);
    }
}

fn execute_action(action: stone_cli::Action) -> Result<(), errors::AppError> {
    match action {
        stone_cli::Action::Analyze { file, n } => Ok(audio::run(&file, n)?),
        stone_cli::Action::Cypher { input, output, message } => {
            println!("[cypher]   input={input}  output={output}  message={message}");
            Ok(())
        },
        stone_cli::Action::Decypher { input } => {
            println!("[decypher] input={input}");
            Ok(())
        },
        stone_cli::Action::Help {} => {
            stone_cli::print_help();
            Ok(())
        }
        stone_cli::Action::Visualize { file, mode, output } => {
            bonus_visualizer::run(&file, &mode, &output)?;
            Ok(())
        },
    }
}

fn run() -> Result<(), errors::AppError> {
    let action = stone_cli::Action::from_env()?;
    execute_action(action)
}