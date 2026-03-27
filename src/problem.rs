use serde::Serialize;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum Severity {
    Info,
    VeryLow,
    Low,
    Medium,
    High,
}

impl Severity {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Info => "INFO",
            Self::VeryLow => "VERY_LOW",
            Self::Low => "LOW",
            Self::Medium => "MEDIUM",
            Self::High => "HIGH",
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RuleMeta {
    pub id: &'static str,
    pub shortdesc: &'static str,
    pub description: &'static str,
    pub severity: Severity,
    pub tags: &'static [&'static str],
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct Problem {
    pub id: String,
    pub message: String,
    pub filename: String,
    pub linenumber: usize,
    pub line: String,
    pub severity: Severity,
}

impl Problem {
    pub fn from_meta(
        meta: &'static RuleMeta,
        filename: impl Into<String>,
        linenumber: usize,
        line: impl Into<String>,
        message: Option<impl Into<String>>,
    ) -> Self {
        Self {
            id: meta.id.to_owned(),
            message: message
                .map(Into::into)
                .unwrap_or_else(|| meta.shortdesc.to_owned()),
            filename: filename.into(),
            linenumber,
            line: line.into(),
            severity: meta.severity,
        }
    }
}
