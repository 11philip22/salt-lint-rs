use crate::config::Config;
use crate::file_types::FileKind;

pub struct RuleContext<'a> {
    pub filename: &'a str,
    pub kind: FileKind,
    pub config: &'a Config,
}

impl<'a> RuleContext<'a> {
    pub fn new(filename: &'a str, kind: FileKind, config: &'a Config) -> Self {
        Self {
            filename,
            kind,
            config,
        }
    }
}
