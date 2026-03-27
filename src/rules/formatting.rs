use std::sync::LazyLock;

use regex::Regex;

use crate::engine::context::RuleContext;
use crate::engine::rule::{ProblemSeed, Rule};
use crate::file_types::FileKind;
use crate::problem::{RuleMeta, Severity};

const FORMATTING_TAGS: &[&str] = &["formatting"];
const ALL_LANGUAGES: &[FileKind] = &[];
const SLS_LANGUAGES: &[FileKind] = &[FileKind::Sls];

const TRAILING_WHITESPACE_META: RuleMeta = RuleMeta {
    id: "201",
    shortdesc: "Trailing whitespace",
    description: "There should not be any trailing whitespace",
    severity: Severity::Info,
    tags: FORMATTING_TAGS,
    languages: ALL_LANGUAGES,
};

const NO_TABS_META: RuleMeta = RuleMeta {
    id: "203",
    shortdesc: "Most files should not contain tabs",
    description: "Tabs can cause unexpected display issues, use spaces",
    severity: Severity::Low,
    tags: FORMATTING_TAGS,
    languages: SLS_LANGUAGES,
};

const LINE_TOO_LONG_META: RuleMeta = RuleMeta {
    id: "204",
    shortdesc: "Lines should be no longer than 160 chars",
    description: "Long lines make code harder to read and code review more difficult",
    severity: Severity::VeryLow,
    tags: FORMATTING_TAGS,
    languages: ALL_LANGUAGES,
};

const IRREGULAR_SPACES_META: RuleMeta = RuleMeta {
    id: "212",
    shortdesc: "Most files should not contain irregular spaces",
    description: "Irregular spaces can cause unexpected display issues, use spaces",
    severity: Severity::Low,
    tags: FORMATTING_TAGS,
    languages: ALL_LANGUAGES,
};

const CMD_WAIT_META: RuleMeta = RuleMeta {
    id: "213",
    shortdesc: "SaltStack recommends using cmd.run together with onchanges, rather than cmd.wait",
    description: "SaltStack recommends using cmd.run together with onchanges, rather than cmd.wait",
    severity: Severity::Low,
    tags: FORMATTING_TAGS,
    languages: SLS_LANGUAGES,
};

const TYPO_ONCHANGES_META: RuleMeta = RuleMeta {
    id: "216",
    shortdesc: "\"onchange\" looks like a typo. Did you mean \"onchanges\"?",
    description: "\"onchange\" looks like a typo. Did you mean \"onchanges\"?",
    severity: Severity::Low,
    tags: FORMATTING_TAGS,
    languages: SLS_LANGUAGES,
};

const TYPO_REQUIRE_META: RuleMeta = RuleMeta {
    id: "217",
    shortdesc: "\"requires\" looks like a typo. Did you mean \"require\"?",
    description: "\"requires\" looks like a typo. Did you mean \"require\"?",
    severity: Severity::Low,
    tags: FORMATTING_TAGS,
    languages: SLS_LANGUAGES,
};

static CMD_WAIT_REGEX: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"^\s{2}cmd\.wait:(\s+)?$").expect("valid cmd.wait regex"));
static TYPO_ONCHANGES_REGEX: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r"^\s+- (on_?change(|_in|_any)|on_changes(|_in|_any)):")
        .expect("valid typo onchange regex")
});
static TYPO_REQUIRE_REGEX: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"^\s+- requires(|_in|_any):").expect("valid typo require regex"));

pub const IRREGULAR_SPACES: &[char] = &[
    '\u{000B}', '\u{000C}', '\u{00A0}', '\u{0085}', '\u{1680}', '\u{180E}', '\u{FEFF}', '\u{2000}',
    '\u{2001}', '\u{2002}', '\u{2003}', '\u{2004}', '\u{2005}', '\u{2006}', '\u{2007}', '\u{2008}',
    '\u{2009}', '\u{200A}', '\u{200B}', '\u{2028}', '\u{2029}', '\u{202F}', '\u{205F}', '\u{3000}',
];

pub struct TrailingWhitespaceRule;
pub struct NoTabsRule;
pub struct LineTooLongRule;
pub struct NoIrregularSpacesRule;
pub struct CmdWaitRecommendRule;
pub struct TypoOnchangesRule;
pub struct TypoRequireRule;

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

impl Rule for NoTabsRule {
    fn meta(&self) -> &'static RuleMeta {
        &NO_TABS_META
    }

    fn check_line(
        &self,
        _ctx: &RuleContext<'_>,
        line_no: usize,
        line: &str,
    ) -> Option<ProblemSeed> {
        line.contains('\t')
            .then(|| ProblemSeed::line(line_no, line, None::<String>))
    }
}

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

impl Rule for NoIrregularSpacesRule {
    fn meta(&self) -> &'static RuleMeta {
        &IRREGULAR_SPACES_META
    }

    fn check_line(
        &self,
        _ctx: &RuleContext<'_>,
        line_no: usize,
        line: &str,
    ) -> Option<ProblemSeed> {
        IRREGULAR_SPACES
            .iter()
            .any(|space| line.contains(*space))
            .then(|| ProblemSeed::line(line_no, line, None::<String>))
    }
}

impl Rule for CmdWaitRecommendRule {
    fn meta(&self) -> &'static RuleMeta {
        &CMD_WAIT_META
    }

    fn check_line(
        &self,
        _ctx: &RuleContext<'_>,
        line_no: usize,
        line: &str,
    ) -> Option<ProblemSeed> {
        CMD_WAIT_REGEX
            .is_match(line)
            .then(|| ProblemSeed::line(line_no, line, None::<String>))
    }
}

impl Rule for TypoOnchangesRule {
    fn meta(&self) -> &'static RuleMeta {
        &TYPO_ONCHANGES_META
    }

    fn check_line(
        &self,
        _ctx: &RuleContext<'_>,
        line_no: usize,
        line: &str,
    ) -> Option<ProblemSeed> {
        TYPO_ONCHANGES_REGEX
            .is_match(line)
            .then(|| ProblemSeed::line(line_no, line, None::<String>))
    }
}

impl Rule for TypoRequireRule {
    fn meta(&self) -> &'static RuleMeta {
        &TYPO_REQUIRE_META
    }

    fn check_line(
        &self,
        _ctx: &RuleContext<'_>,
        line_no: usize,
        line: &str,
    ) -> Option<ProblemSeed> {
        TYPO_REQUIRE_REGEX
            .is_match(line)
            .then(|| ProblemSeed::line(line_no, line, None::<String>))
    }
}

pub fn all_rules() -> Vec<Box<dyn Rule>> {
    vec![
        Box::new(TrailingWhitespaceRule),
        Box::new(NoTabsRule),
        Box::new(LineTooLongRule),
        Box::new(NoIrregularSpacesRule),
        Box::new(CmdWaitRecommendRule),
        Box::new(TypoOnchangesRule),
        Box::new(TypoRequireRule),
    ]
}

pub fn builtin_rule_count() -> usize {
    7
}
