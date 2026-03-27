use std::collections::BTreeSet;

use salt_lint_rs::config::Config;
use salt_lint_rs::engine::collection::RuleCollection;
use salt_lint_rs::engine::context::RuleContext;
use salt_lint_rs::engine::rule::{ProblemSeed, Rule};
use salt_lint_rs::file_types::FileKind;
use salt_lint_rs::problem::{RuleMeta, Severity};
use salt_lint_rs::rules;
use tempfile::tempdir;

struct RunFromText {
    collection: RuleCollection,
}

impl RunFromText {
    fn new(collection: RuleCollection) -> Self {
        Self { collection }
    }

    fn run(&self, text: &str) -> Vec<salt_lint_rs::problem::Problem> {
        let tempdir = tempdir().expect("tempdir should be created");
        let config = Config::empty(tempdir.path().to_path_buf());
        let context = RuleContext::new("inline.sls", FileKind::Sls, &config);
        self.collection
            .run(&context, text, &BTreeSet::new(), &BTreeSet::new())
    }
}

const TEST_TAGS: &[&str] = &["test"];
const ALL_LANGUAGES: &[FileKind] = &[];

const TEST_LINE_RULE_META: RuleMeta = RuleMeta {
    id: "801",
    shortdesc: "Flags TODO lines",
    description: "Flags TODO lines",
    severity: Severity::Low,
    tags: TEST_TAGS,
    languages: ALL_LANGUAGES,
};

const TEST_TEXT_RULE_META: RuleMeta = RuleMeta {
    id: "802",
    shortdesc: "Flags BAD sections",
    description: "Flags BAD sections",
    severity: Severity::Low,
    tags: TEST_TAGS,
    languages: ALL_LANGUAGES,
};

const TEST_SORT_RULE_META: RuleMeta = RuleMeta {
    id: "700",
    shortdesc: "Sorts earlier",
    description: "Sorts earlier",
    severity: Severity::Low,
    tags: TEST_TAGS,
    languages: ALL_LANGUAGES,
};

struct TodoLineRule;

impl Rule for TodoLineRule {
    fn meta(&self) -> &'static RuleMeta {
        &TEST_LINE_RULE_META
    }

    fn check_line(
        &self,
        _ctx: &RuleContext<'_>,
        line_no: usize,
        line: &str,
    ) -> Option<ProblemSeed> {
        line.contains("TODO")
            .then(|| ProblemSeed::line(line_no, line, Some("Found TODO")))
    }
}

struct BadSectionRule;

impl Rule for BadSectionRule {
    fn meta(&self) -> &'static RuleMeta {
        &TEST_TEXT_RULE_META
    }

    fn check_text(&self, _ctx: &RuleContext<'_>, text: &str) -> Vec<ProblemSeed> {
        if !text.contains("BAD") {
            return Vec::new();
        }

        vec![ProblemSeed::section(2, "BAD", Some("Found BAD"), text)]
    }
}

struct SortRule;

impl Rule for SortRule {
    fn meta(&self) -> &'static RuleMeta {
        &TEST_SORT_RULE_META
    }

    fn check_line(
        &self,
        _ctx: &RuleContext<'_>,
        line_no: usize,
        line: &str,
    ) -> Option<ProblemSeed> {
        line.contains("TODO")
            .then(|| ProblemSeed::line(line_no, line, Some("Earlier id should sort first")))
    }
}

#[test]
fn line_rules_execute_against_text() {
    let mut collection = RuleCollection::new();
    collection.register(Box::new(TodoLineRule));

    let problems = RunFromText::new(collection).run("good\nTODO item\n");

    assert_eq!(problems.len(), 1);
    assert_eq!(problems[0].id, "801");
    assert_eq!(problems[0].linenumber, 2);
}

#[test]
fn full_text_rules_execute_against_text() {
    let mut collection = RuleCollection::new();
    collection.register(Box::new(BadSectionRule));

    let problems = RunFromText::new(collection).run("good\nBAD\n");

    assert_eq!(problems.len(), 1);
    assert_eq!(problems[0].id, "802");
    assert_eq!(problems[0].linenumber, 2);
}

#[test]
fn line_noqa_skips_matching_rule() {
    let mut collection = RuleCollection::new();
    collection.register(Box::new(TodoLineRule));

    let problems = RunFromText::new(collection).run("TODO item # noqa 801\n");

    assert!(problems.is_empty());
}

#[test]
fn section_noqa_skips_full_text_rule() {
    let mut collection = RuleCollection::new();
    collection.register(Box::new(BadSectionRule));

    let problems = RunFromText::new(collection).run("good\nBAD # noqa 802\n");

    assert!(problems.is_empty());
}

#[test]
fn collection_sorts_problems_by_id_when_line_matches_are_equal() {
    let mut collection = RuleCollection::new();
    collection.register(Box::new(TodoLineRule));
    collection.register(Box::new(SortRule));

    let problems = RunFromText::new(collection).run("TODO item\n");

    assert_eq!(problems.len(), 2);
    assert_eq!(problems[0].id, "700");
    assert_eq!(problems[1].id, "801");
}

#[test]
fn builtin_rules_register_minimal_host_validation_rules() {
    let collection = rules::builtin_rules();
    assert_eq!(collection.len(), 32);
}
