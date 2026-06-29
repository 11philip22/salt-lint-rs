use std::path::Path;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FileKind {
    Sls,
    Jinja,
    Unknown,
}

impl FileKind {
    pub fn detect(path: impl AsRef<Path>) -> Self {
        let extension = path
            .as_ref()
            .extension()
            .and_then(|ext| ext.to_str())
            .map(|ext| ext.to_ascii_lowercase());

        match extension.as_deref() {
            Some("sls") => Self::Sls,
            Some("jinja") | Some("jinja2") | Some("j2") => Self::Jinja,
            _ => Self::Unknown,
        }
    }
}
