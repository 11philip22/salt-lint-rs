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

const JINJA_VARIABLE_META: RuleMeta = RuleMeta {
    id: "206",
    shortdesc: "Jinja variables should have spaces before and after: '{{ var_name }}'",
    description: "Jinja variables should have spaces before and after: '{{ var_name }}'",
    severity: Severity::Low,
    tags: JINJA_TAGS,
    languages: JINJA_LANGUAGES,
};

const JINJA_PILLAR_GRAINS_META: RuleMeta = RuleMeta {
    id: "211",
    shortdesc: "pillar.get or grains.get should be formatted differently",
    description: "pillar.get and grains.get should always be formatted like salt['pillar.get']('item'), grains['item1'] or  pillar.get('item')",
    severity: Severity::High,
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
static JINJA_VARIABLE_REGEX: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r"\{\{[^ {}\-\+\d]|\{\{[-\+][^ {}]|[^ {}\-\+\d]\}\}|[^ {}][-\+\d]\}\}")
        .expect("valid jinja variable regex")
});
static JINJA_PILLAR_GRAINS_REGEX: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r"\{\{(?: |\-|\+)?(?:pillar|grains)\.get\[[^}]+\}\}")
        .expect("valid jinja pillar or grains regex")
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
pub struct JinjaVariableHasSpacesRule;
pub struct JinjaPillarGrainsGetFormatRule;

impl Rule for JinjaCommentHasSpacesRule {
    fn meta(&self) -> &'static RuleMeta {
        &JINJA_COMMENT_META
    }

    fn check_text(&self, _ctx: &RuleContext<'_>, text: &str) -> Vec<ProblemSeed> {
        scan_escaped_lines(text, self.meta(), &JINJA_COMMENT_REGEX)
    }
}

impl Rule for JinjaVariableHasSpacesRule {
    fn meta(&self) -> &'static RuleMeta {
        &JINJA_VARIABLE_META
    }

    fn check_text(&self, _ctx: &RuleContext<'_>, text: &str) -> Vec<ProblemSeed> {
        scan_escaped_lines(text, self.meta(), &JINJA_VARIABLE_REGEX)
    }
}

impl Rule for JinjaPillarGrainsGetFormatRule {
    fn meta(&self) -> &'static RuleMeta {
        &JINJA_PILLAR_GRAINS_META
    }

    fn check_text(&self, _ctx: &RuleContext<'_>, text: &str) -> Vec<ProblemSeed> {
        scan_escaped_lines(text, self.meta(), &JINJA_PILLAR_GRAINS_REGEX)
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

            if parse_noqa_ids(line).any(|id| id == meta.id) {
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
        Box::new(JinjaVariableHasSpacesRule),
        Box::new(JinjaPillarGrainsGetFormatRule),
    ]
}
