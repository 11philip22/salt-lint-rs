use std::path::PathBuf;

use clap::{ArgAction, Parser};

#[derive(Debug, Clone, Parser, PartialEq, Eq)]
#[command(name = "salt-lint", version, about = "Lint Salt state files", long_about = None)]
pub struct CliArgs {
    #[arg(value_name = "FILE")]
    pub files: Vec<PathBuf>,

    #[arg(short = 'L', long = "list-rules")]
    pub list_rules: bool,

    #[arg(short = 'r', value_name = "RULESDIR", action = ArgAction::Append)]
    pub rulesdir: Vec<PathBuf>,

    #[arg(short = 'R')]
    pub use_default_rules: bool,

    #[arg(short = 't', value_name = "TAG", action = ArgAction::Append)]
    pub tags: Vec<String>,

    #[arg(short = 'T', long = "list-tags")]
    pub list_tags: bool,

    #[arg(short = 'v', action = ArgAction::Count)]
    pub verbosity: u8,

    #[arg(short = 'x', value_name = "RULE", action = ArgAction::Append)]
    pub skip_list: Vec<String>,

    #[arg(long = "nocolor", alias = "nocolour")]
    pub no_color: bool,

    #[arg(long = "force-color", alias = "force-colour")]
    pub force_color: bool,

    #[arg(long = "exclude", value_name = "PATH", action = ArgAction::Append)]
    pub exclude_paths: Vec<PathBuf>,

    #[arg(long)]
    pub json: bool,

    #[arg(long)]
    pub severity: bool,

    #[arg(short = 'c', value_name = "CONFIG")]
    pub config: Option<PathBuf>,
}
