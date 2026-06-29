use std::sync::LazyLock;

use regex::Regex;

use crate::engine::context::RuleContext;
use crate::engine::rule::{ProblemSeed, Rule};
use crate::file_types::FileKind;
use crate::problem::{RuleMeta, Severity};

const FORMATTING_TAGS: &[&str] = &["formatting"];
const SLS_LANGUAGES: &[FileKind] = &[FileKind::Sls];

const YAML_HAS_OCTAL_META: RuleMeta = RuleMeta {
    id: "210",
    shortdesc: "Numbers that start with '0' should always be encapsulated in quotation marks",
    description: "Numbers that start with '0' should always be encapsulated in quotation marks",
    severity: Severity::High,
    tags: FORMATTING_TAGS,
    languages: SLS_LANGUAGES,
};

static YAML_OCTAL_REGEX: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"^[^:]+:\s*0[0-9]+\s*").expect("valid octal regex"));
static YAML_OCTAL_EXCLUDE_REGEX: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"[ T]\d\d:\d\d(?:[: ]|$)").expect("valid octal exclude regex"));

pub struct YamlHasOctalValueRule;

impl Rule for YamlHasOctalValueRule {
    fn meta(&self) -> &'static RuleMeta {
        &YAML_HAS_OCTAL_META
    }

    fn check_line(
        &self,
        _ctx: &RuleContext<'_>,
        line_no: usize,
        line: &str,
    ) -> Option<ProblemSeed> {
        let found = YAML_OCTAL_REGEX.find(line)?;
        let remainder = &line[found.end()..];

        if !(remainder.is_empty() || remainder.starts_with('#') || remainder.starts_with("{#")) {
            return None;
        }

        if YAML_OCTAL_EXCLUDE_REGEX.is_match(found.as_str()) {
            return None;
        }

        Some(ProblemSeed::line(line_no, line, None::<String>))
    }
}

pub fn all_rules() -> Vec<Box<dyn Rule>> {
    vec![Box::new(YamlHasOctalValueRule)]
}
