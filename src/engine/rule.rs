use crate::engine::context::RuleContext;
use crate::problem::RuleMeta;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ProblemSeed {
    pub linenumber: usize,
    pub line: String,
    pub message: Option<String>,
}

pub trait Rule: Send + Sync {
    fn meta(&self) -> &'static RuleMeta;

    fn check_line(&self, _ctx: &RuleContext, _line_no: usize, _line: &str) -> Option<ProblemSeed> {
        None
    }

    fn check_text(&self, _ctx: &RuleContext, _text: &str) -> Vec<ProblemSeed> {
        Vec::new()
    }
}
