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

#[cfg(test)]
mod tests {
    use super::FileKind;

    #[test]
    fn detects_sls_files() {
        assert_eq!(FileKind::detect("top.sls"), FileKind::Sls);
    }

    #[test]
    fn detects_jinja_files() {
        assert_eq!(FileKind::detect("template.jinja"), FileKind::Jinja);
        assert_eq!(FileKind::detect("template.jinja2"), FileKind::Jinja);
        assert_eq!(FileKind::detect("template.j2"), FileKind::Jinja);
    }

    #[test]
    fn treats_other_extensions_as_unknown() {
        assert_eq!(FileKind::detect("README.md"), FileKind::Unknown);
        assert_eq!(FileKind::detect("saltfile"), FileKind::Unknown);
    }
}
