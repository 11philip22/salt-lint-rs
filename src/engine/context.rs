use crate::file_types::FileKind;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RuleContext {
    pub filename: String,
    pub kind: FileKind,
}
