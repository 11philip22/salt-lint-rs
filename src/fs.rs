use std::path::PathBuf;

use crate::file_types::FileKind;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LintFile {
    pub path: PathBuf,
    pub kind: FileKind,
}
