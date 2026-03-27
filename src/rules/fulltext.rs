use std::sync::LazyLock;

use regex::Regex;

use crate::engine::context::RuleContext;
use crate::engine::rule::{ProblemSeed, Rule};
use crate::file_types::FileKind;
use crate::problem::{RuleMeta, Severity};

const DEPRECATION_TAGS: &[&str] = &["deprecation"];
const FORMATTING_TAGS: &[&str] = &["formatting"];
const SLS_LANGUAGES: &[FileKind] = &[FileKind::Sls];

const CMD_RUN_QUIET_META: RuleMeta = RuleMeta {
    id: "901",
    shortdesc: "Using the quiet argument with cmd.run is deprecated. Use output_loglevel: quiet",
    description: "Using the quiet argument with cmd.run is deprecated. Use output_loglevel: quiet",
    severity: Severity::High,
    tags: DEPRECATION_TAGS,
    languages: SLS_LANGUAGES,
};

const NESTED_DICT_META: RuleMeta = RuleMeta {
    id: "219",
    shortdesc: "Nested dictionaries (in context or defaults) should be over-indented",
    description: "Nested dictionaries (in context or defaults) should be over-indented",
    severity: Severity::High,
    tags: FORMATTING_TAGS,
    languages: SLS_LANGUAGES,
};

static CMD_RUN_QUIET_REGEX: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r"(?m)^.+\n^\s{2}cmd\.run:(?:\n.+)+\n^\s{4}- quiet\s?.*")
        .expect("valid cmd.run quiet regex")
});
static NESTED_DICT_HEADER_REGEX: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r"^(\s+)-\s+(context|defaults):[^{\[]*$").expect("valid nested dict header regex")
});

pub struct CmdRunQuietRule;
pub struct NestedDictRule;

impl Rule for CmdRunQuietRule {
    fn meta(&self) -> &'static RuleMeta {
        &CMD_RUN_QUIET_META
    }

    fn check_text(&self, _ctx: &RuleContext<'_>, text: &str) -> Vec<ProblemSeed> {
        CMD_RUN_QUIET_REGEX
            .find_iter(text)
            .map(|matched| {
                let section = &text[matched.start()..matched.end()];
                let prefix = &text[..matched.end()];
                let lines = prefix.lines().collect::<Vec<_>>();
                let last_line = lines.last().copied().unwrap_or_default();
                ProblemSeed::section(lines.len(), last_line, None::<String>, section.to_owned())
            })
            .collect()
    }
}

impl Rule for NestedDictRule {
    fn meta(&self) -> &'static RuleMeta {
        &NESTED_DICT_META
    }

    fn check_text(&self, _ctx: &RuleContext<'_>, text: &str) -> Vec<ProblemSeed> {
        let lines = text.split('\n').collect::<Vec<_>>();
        let mut results = Vec::new();

        for index in 0..lines.len().saturating_sub(1) {
            let line = lines[index];
            let Some(captures) = NESTED_DICT_HEADER_REGEX.captures(line) else {
                continue;
            };

            let Some(indent_match) = captures.get(1) else {
                continue;
            };
            let base_indent = indent_match.as_str().len();
            let next_line = lines[index + 1];
            let next_indent = next_line.chars().take_while(|ch| *ch == ' ').count();
            let trimmed = next_line.trim_start();

            let valid_indent = (base_indent..=base_indent + 3).contains(&next_indent);
            let looks_like_inline_key = !trimmed.is_empty()
                && !trimmed.starts_with('-')
                && !trimmed.starts_with('{')
                && !trimmed.starts_with('[')
                && !trimmed.starts_with(char::is_whitespace)
                && trimmed.contains(": ");

            if valid_indent && looks_like_inline_key {
                let section = format!("{line}\n{next_line}");
                results.push(ProblemSeed::section(
                    index + 2,
                    next_line,
                    None::<String>,
                    section,
                ));
            }
        }

        results
    }
}

pub fn all_rules() -> Vec<Box<dyn Rule>> {
    vec![Box::new(CmdRunQuietRule), Box::new(NestedDictRule)]
}

pub fn builtin_rule_count() -> usize {
    2
}
