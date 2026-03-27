use std::collections::BTreeSet;
use std::env;
use std::fs;
use std::io::{self, Read, Write};
use std::path::PathBuf;

use clap::CommandFactory;
use thiserror::Error;

use crate::cli::CliArgs;
use crate::config::{Config, ConfigError};
use crate::engine::collection::{RuleCollection, sort_problems};
use crate::engine::context::RuleContext;
use crate::formatter::{FormatterKind, format_problems};
use crate::fs as lint_fs;
use crate::rules;

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
        self.run_with_input(args, None, stdout, stderr)
    }

    pub fn run_with_input<WOut, WErr>(
        self,
        args: CliArgs,
        stdin_text: Option<String>,
        stdout: &mut WOut,
        stderr: &mut WErr,
    ) -> Result<i32, AppError>
    where
        WOut: Write,
        WErr: Write,
    {
        let collection = rules::builtin_rules();

        if args.list_rules {
            writeln!(stdout, "{}", collection.render_rules())?;
            return Ok(0);
        }

        if args.list_tags {
            writeln!(stdout, "{}", collection.render_tags())?;
            return Ok(0);
        }

        if args.files.is_empty() && stdin_text.is_none() {
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

        let input_files = lint_fs::resolve_input_files(&args.files, &cwd, &config)?;
        let tags = config.tags.iter().cloned().collect::<BTreeSet<_>>();
        let problems = lint_inputs(
            &collection,
            &input_files,
            stdin_text,
            &config,
            &tags,
            stderr,
        )?;

        if problems.is_empty() {
            return Ok(0);
        }

        let output = format_problems(
            &problems,
            FormatterKind::from_flags(config.json, config.severity),
            !args.no_color || args.force_color,
        );
        write!(stdout, "{output}")?;
        Ok(2)
    }

    fn current_dir(&self) -> Result<PathBuf, AppError> {
        match &self.current_dir_override {
            Some(path) => Ok(path.clone()),
            None => Ok(env::current_dir()?),
        }
    }
}

fn lint_inputs<WErr>(
    collection: &RuleCollection,
    files: &[crate::fs::LintFile],
    stdin_text: Option<String>,
    config: &Config,
    tags: &BTreeSet<String>,
    stderr: &mut WErr,
) -> Result<Vec<crate::problem::Problem>, AppError>
where
    WErr: Write,
{
    let mut problems = Vec::new();

    for file in files {
        let text = match fs::read_to_string(&file.disk_path) {
            Ok(text) => text,
            Err(err) => {
                writeln!(
                    stderr,
                    "WARNING: Couldn't open {} - {}",
                    file.path.display(),
                    err
                )?;
                continue;
            }
        };

        let filename = file.path.to_string_lossy().into_owned();
        let context = RuleContext::new(&filename, file.kind, config);
        problems.extend(collection.run(&context, &text, tags, &config.skip_list));
    }

    if let Some(text) = stdin_text {
        let context = RuleContext::new("stdin.sls", crate::file_types::FileKind::Sls, config);
        problems.extend(collection.run(&context, &text, tags, &config.skip_list));
    }

    sort_problems(&mut problems);
    Ok(problems)
}

pub fn read_stdin_if_available<R>(stdin: &mut R) -> Result<Option<String>, AppError>
where
    R: Read,
{
    let mut buffer = String::new();
    stdin.read_to_string(&mut buffer)?;

    if buffer.is_empty() {
        Ok(None)
    } else {
        Ok(Some(buffer))
    }
}
