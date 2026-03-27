use std::env;
use std::io::{self, Write};
use std::path::PathBuf;

use clap::CommandFactory;
use thiserror::Error;

use crate::cli::CliArgs;
use crate::config::{Config, ConfigError};
use crate::fs;

#[derive(Debug, Default, Clone)]
pub struct App {
    current_dir_override: Option<PathBuf>,
}

#[derive(Debug, Error)]
pub enum AppError {
    #[error(transparent)]
    Io(#[from] io::Error),
    #[error(transparent)]
    Config(#[from] ConfigError),
}

impl App {
    pub fn with_current_dir(path: impl Into<PathBuf>) -> Self {
        Self {
            current_dir_override: Some(path.into()),
        }
    }

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

        let cwd = self.current_dir()?;
        let config = Config::from_cli(&args, &cwd)?;

        for path in &config.unsupported_rulesdirs {
            writeln!(
                stderr,
                "WARNING: custom Python rule directories are unsupported and were ignored: {}",
                path.display()
            )?;
        }

        let _input_files = fs::resolve_input_files(&args.files, &cwd, &config)?;
        Ok(0)
    }

    fn current_dir(&self) -> Result<PathBuf, AppError> {
        match &self.current_dir_override {
            Some(path) => Ok(path.clone()),
            None => Ok(env::current_dir()?),
        }
    }
}
