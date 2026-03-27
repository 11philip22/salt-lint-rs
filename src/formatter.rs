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

pub fn format_problems(problems: &[Problem], kind: FormatterKind, _colored: bool) -> String {
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

#[cfg(test)]
mod tests {
    use super::{FormatterKind, format_problems};
    use crate::problem::{Problem, Severity};

    fn sample_problem() -> Problem {
        Problem {
            id: "201".into(),
            message: "Trailing whitespace".into(),
            filename: "top.sls".into(),
            linenumber: 3,
            line: "bad line ".into(),
            severity: Severity::Info,
        }
    }

    #[test]
    fn formats_json_with_expected_fields() {
        let output = format_problems(&[sample_problem()], FormatterKind::Json, false);

        assert!(output.contains("\"id\":\"201\""));
        assert!(output.contains("\"message\":\"Trailing whitespace\""));
        assert!(output.contains("\"filename\":\"top.sls\""));
        assert!(output.contains("\"linenumber\":3"));
        assert!(output.contains("\"severity\":\"INFO\""));
    }

    #[test]
    fn formats_severity_output() {
        let output = format_problems(&[sample_problem()], FormatterKind::Severity, false);

        assert!(output.contains("[201] [INFO] Trailing whitespace"));
        assert!(output.contains("top.sls:3"));
    }
}
