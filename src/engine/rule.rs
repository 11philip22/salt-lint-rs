use crate::engine::context::RuleContext;
use crate::problem::{Problem, RuleMeta};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ProblemSeed {
    pub linenumber: usize,
    pub line: String,
    pub message: Option<String>,
    pub skip_excerpt: Option<String>,
}

impl ProblemSeed {
    pub fn line(
        linenumber: usize,
        line: impl Into<String>,
        message: Option<impl Into<String>>,
    ) -> Self {
        Self {
            linenumber,
            line: line.into(),
            message: message.map(Into::into),
            skip_excerpt: None,
        }
    }

    pub fn section(
        linenumber: usize,
        line: impl Into<String>,
        message: Option<impl Into<String>>,
        skip_excerpt: impl Into<String>,
    ) -> Self {
        Self {
            linenumber,
            line: line.into(),
            message: message.map(Into::into),
            skip_excerpt: Some(skip_excerpt.into()),
        }
    }

    pub fn into_problem(self, meta: &'static RuleMeta, filename: &str) -> Problem {
        Problem::from_meta(meta, filename, self.linenumber, self.line, self.message)
    }
}

pub trait Rule: Send + Sync {
    fn meta(&self) -> &'static RuleMeta;

    fn check_line(
        &self,
        _ctx: &RuleContext<'_>,
        _line_no: usize,
        _line: &str,
    ) -> Option<ProblemSeed> {
        None
    }

    fn check_text(&self, _ctx: &RuleContext<'_>, _text: &str) -> Vec<ProblemSeed> {
        Vec::new()
    }
}
