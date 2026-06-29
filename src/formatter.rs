use crate::problem::Problem;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FormatterKind {
    Default,
    Json,
    Severity,
}

impl FormatterKind {
    pub fn from_flags(json: bool, severity: bool) -> Self {
        if json {
            Self::Json
        } else if severity {
            Self::Severity
        } else {
            Self::Default
        }
    }
}

pub fn format_problems(problems: &[Problem], kind: FormatterKind) -> String {
    match kind {
        FormatterKind::Default => problems
            .iter()
            .map(|problem| {
                format!(
                    "[{}] {}\n{}:{}\n{}\n",
                    problem.id, problem.message, problem.filename, problem.linenumber, problem.line
                )
            })
            .collect(),
        FormatterKind::Json => {
            serde_json::to_string(problems).expect("serializing problems to JSON should not fail")
        }
        FormatterKind::Severity => problems
            .iter()
            .map(|problem| {
                format!(
                    "[{}] [{}] {}\n{}:{}\n{}\n",
                    problem.id,
                    problem.severity.as_str(),
                    problem.message,
                    problem.filename,
                    problem.linenumber,
                    problem.line
                )
            })
            .collect(),
    }
}
