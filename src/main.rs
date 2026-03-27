use std::io::IsTerminal;
use std::process::ExitCode;

use clap::Parser;

use salt_lint_rs::app::{App, AppError, read_stdin_if_available};
use salt_lint_rs::cli::CliArgs;

fn main() -> ExitCode {
    let args = CliArgs::parse();
    let app = App::default();

    match run(args, app) {
        Ok(code) => ExitCode::from(code as u8),
        Err(err) => {
            eprintln!("{err}");
            ExitCode::from(2)
        }
    }
}

fn run(args: CliArgs, app: App) -> Result<i32, AppError> {
    let mut stdin = std::io::stdin();
    let mut stdout = std::io::stdout();
    let mut stderr = std::io::stderr();
    let stdin_text = if stdin.is_terminal() {
        None
    } else {
        read_stdin_if_available(&mut stdin)?
    };
    app.run_with_input(args, stdin_text, &mut stdout, &mut stderr)
}
