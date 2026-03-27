use std::sync::LazyLock;

use regex::Regex;

use crate::engine::context::RuleContext;
use crate::engine::rule::{ProblemSeed, Rule};
use crate::engine::skip::parse_noqa_ids;
use crate::file_types::FileKind;
use crate::problem::{RuleMeta, Severity};

const JINJA_LANGUAGES: &[FileKind] = &[FileKind::Sls, FileKind::Jinja];
const JINJA_TAGS: &[&str] = &["formatting", "jinja"];

const JINJA_STATEMENT_META: RuleMeta = RuleMeta {
    id: "202",
    shortdesc: "Jinja statement should have spaces before and after: '{% statement %}'",
    description: "Jinja statement should have spaces before and after: '{% statement %}'",
    severity: Severity::Low,
    tags: JINJA_TAGS,
    languages: JINJA_LANGUAGES,
};

const JINJA_COMMENT_META: RuleMeta = RuleMeta {
    id: "209",
    shortdesc: "Jinja comment should have spaces before and after: '{# comment #}'",
    description: "Jinja comment should have spaces before and after: '{# comment #}'",
    severity: Severity::Low,
    tags: JINJA_TAGS,
    languages: JINJA_LANGUAGES,
};

static RAW_BLOCK_REGEX: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r"(?s)\{%[+-]?\s?raw\s?[+-]?%\}.*?\{%[+-]?\s?endraw\s?[+-]?%\}")
        .expect("valid raw block regex")
});
static JINJA_STATEMENT_REGEX: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r"\{%[^ \-\+]|\{%[\-\+][^ ]|[^ \-\+]%\}|[^ ][\-\+]%\}")
        .expect("valid jinja statement regex")
});
static JINJA_COMMENT_REGEX: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r"\{#[^ \-\+]|\{#[\-\+][^ ]|[^ \-\+]#\}|[^ ][\-\+]#\}")
        .expect("valid jinja comment regex")
});

pub struct JinjaStatementHasSpacesRule;

impl Rule for JinjaStatementHasSpacesRule {
    fn meta(&self) -> &'static RuleMeta {
        &JINJA_STATEMENT_META
    }

    fn check_text(&self, _ctx: &RuleContext<'_>, text: &str) -> Vec<ProblemSeed> {
        scan_escaped_lines(text, self.meta(), &JINJA_STATEMENT_REGEX)
    }
}

pub struct JinjaCommentHasSpacesRule;

impl Rule for JinjaCommentHasSpacesRule {
    fn meta(&self) -> &'static RuleMeta {
        &JINJA_COMMENT_META
    }

    fn check_text(&self, _ctx: &RuleContext<'_>, text: &str) -> Vec<ProblemSeed> {
        scan_escaped_lines(text, self.meta(), &JINJA_COMMENT_REGEX)
    }
}

fn scan_escaped_lines(text: &str, meta: &'static RuleMeta, regex: &Regex) -> Vec<ProblemSeed> {
    let escaped = escape_raw_blocks(text);
    escaped
        .split('\n')
        .enumerate()
        .filter_map(|(index, line)| {
            if line.trim_start().starts_with('#') {
                return None;
            }

            if parse_noqa_ids(line).contains(meta.id) {
                return None;
            }

            regex
                .find(line)
                .map(|_| ProblemSeed::line(index + 1, line, None::<String>))
        })
        .collect()
}

fn escape_raw_blocks(text: &str) -> String {
    let mut escaped = text.to_owned();

    for matched in RAW_BLOCK_REGEX
        .find_iter(text)
        .collect::<Vec<_>>()
        .into_iter()
        .rev()
    {
        let replacement = "\n".repeat(
            text[matched.start()..matched.end()]
                .lines()
                .count()
                .saturating_sub(1),
        );
        escaped.replace_range(matched.start()..matched.end(), &replacement);
    }

    escaped
}

pub fn all_rules() -> Vec<Box<dyn Rule>> {
    vec![
        Box::new(JinjaStatementHasSpacesRule),
        Box::new(JinjaCommentHasSpacesRule),
    ]
}

pub fn builtin_rule_count() -> usize {
    2
}
