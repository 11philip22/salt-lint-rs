use std::collections::BTreeSet;
use std::path::{Path, PathBuf};

use crate::config::Config;
use crate::file_types::FileKind;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LintFile {
    pub path: PathBuf,
    pub disk_path: PathBuf,
    pub kind: FileKind,
}

pub fn resolve_input_files(inputs: &[PathBuf], cwd: &Path, config: &Config) -> Vec<LintFile> {
    let mut seen = BTreeSet::new();
    let mut files = Vec::new();

    for input in inputs {
        let resolved_path = map_input_path(input, cwd);

        if config.is_excluded(&resolved_path) {
            continue;
        }

        let dedupe_key = normalize_path_string(&resolved_path);
        if !seen.insert(dedupe_key) {
            continue;
        }

        files.push(LintFile {
            kind: FileKind::detect(&resolved_path),
            disk_path: to_disk_path(&resolved_path, cwd),
            path: resolved_path,
        });
    }

    files
}

pub fn map_input_path(input: &Path, cwd: &Path) -> PathBuf {
    let absolute = if input.is_absolute() {
        input.to_path_buf()
    } else {
        cwd.join(input)
    };

    if absolute.is_dir() {
        input.join("init.sls")
    } else {
        input.to_path_buf()
    }
}

fn normalize_path_string(path: impl AsRef<Path>) -> String {
    path.as_ref().to_string_lossy().replace('\\', "/")
}

fn to_disk_path(path: &Path, cwd: &Path) -> PathBuf {
    if path.is_absolute() {
        path.to_path_buf()
    } else {
        cwd.join(path)
    }
}
