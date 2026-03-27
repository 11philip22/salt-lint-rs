use crate::engine::context::RuleContext;
use crate::engine::rule::{ProblemSeed, Rule};
use crate::file_types::FileKind;
use crate::problem::{RuleMeta, Severity};

const FORMATTING_TAGS: &[&str] = &["formatting"];
const ALL_LANGUAGES: &[FileKind] = &[];

const TRAILING_WHITESPACE_META: RuleMeta = RuleMeta {
    id: "201",
    shortdesc: "Trailing whitespace",
    description: "There should not be any trailing whitespace",
    severity: Severity::Info,
    tags: FORMATTING_TAGS,
    languages: ALL_LANGUAGES,
};

const LINE_TOO_LONG_META: RuleMeta = RuleMeta {
    id: "204",
    shortdesc: "Lines should be no longer than 160 chars",
    description: "Long lines make code harder to read and code review more difficult",
    severity: Severity::VeryLow,
    tags: FORMATTING_TAGS,
    languages: ALL_LANGUAGES,
};

pub struct TrailingWhitespaceRule;

impl Rule for TrailingWhitespaceRule {
    fn meta(&self) -> &'static RuleMeta {
        &TRAILING_WHITESPACE_META
    }

    fn check_line(
        &self,
        _ctx: &RuleContext<'_>,
        line_no: usize,
        line: &str,
    ) -> Option<ProblemSeed> {
        let normalized = line.replace('\r', "");
        (normalized.trim_end() != normalized)
            .then(|| ProblemSeed::line(line_no, line, None::<String>))
    }
}

pub struct LineTooLongRule;

impl Rule for LineTooLongRule {
    fn meta(&self) -> &'static RuleMeta {
        &LINE_TOO_LONG_META
    }

    fn check_line(
        &self,
        _ctx: &RuleContext<'_>,
        line_no: usize,
        line: &str,
    ) -> Option<ProblemSeed> {
        (line.len() > 160).then(|| ProblemSeed::line(line_no, line, None::<String>))
    }
}

pub fn builtin_rule_count() -> usize {
    2
}
