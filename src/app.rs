use std::io::{self, Write};

use clap::CommandFactory;
use thiserror::Error;

use crate::cli::CliArgs;

#[derive(Debug, Default, Clone, Copy)]
pub struct App;

#[derive(Debug, Error)]
pub enum AppError {
    #[error(transparent)]
    Io(#[from] io::Error),
}

impl App {
    pub fn run<WOut, WErr>(
        self,
        args: CliArgs,
        stdout: &mut WOut,
        stderr: &mut WErr,
    ) -> Result<i32, AppError>
    where
        WOut: Write,
        WErr: Write,
    {
        if args.list_rules {
            writeln!(stdout, "Built-in rules listing is not implemented yet.")?;
            return Ok(0);
        }

        if args.list_tags {
            writeln!(stdout, "Built-in tag listing is not implemented yet.")?;
            return Ok(0);
        }

        if args.files.is_empty() {
            let mut command = CliArgs::command();
            command.write_help(stderr)?;
            writeln!(stderr)?;
            return Ok(1);
        }

        Ok(0)
    }
}
