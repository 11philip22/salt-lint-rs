use std::collections::BTreeSet;

use salt_lint_rs::config::Config;
use salt_lint_rs::engine::collection::RuleCollection;
use salt_lint_rs::engine::context::RuleContext;
use salt_lint_rs::engine::rule::Rule;
use salt_lint_rs::file_types::FileKind;
use salt_lint_rs::rules::{deprecations, formatting, jinja};
use tempfile::tempdir;

fn run_rules(
    rules: Vec<Box<dyn Rule>>,
    text: &str,
    kind: FileKind,
) -> Vec<salt_lint_rs::problem::Problem> {
    let tempdir = tempdir().expect("tempdir should be created");
    let config = Config::empty(tempdir.path().to_path_buf());
    let context = RuleContext::new("inline.sls", kind, &config);
    let mut collection = RuleCollection::new();

    for rule in rules {
        collection.register(rule);
    }

    collection.run(&context, text, &BTreeSet::new(), &BTreeSet::new())
}

#[test]
fn trailing_whitespace_rule_finds_one_match() {
    let results = run_rules(
        vec![Box::new(formatting::TrailingWhitespaceRule)],
        "/tmp/testfile:\n  file.managed:\n    - source:/salt://lorem  ",
        FileKind::Sls,
    );

    assert_eq!(results.len(), 1);
    assert_eq!(results[0].id, "201");
    assert_eq!(results[0].linenumber, 3);
}

#[test]
fn no_tabs_rule_counts_tab_lines() {
    let results = run_rules(
        vec![Box::new(formatting::NoTabsRule)],
        "/tmp/testfile:\n\tfile.managed:\n\t- source:/salt://lorem",
        FileKind::Sls,
    );

    assert_eq!(results.len(), 2);
    assert!(results.iter().all(|problem| problem.id == "203"));
}

#[test]
fn line_too_long_rule_finds_long_line() {
    let long_line = format!("    - source:/salt://{}", "lorem/".repeat(30));
    let text = format!("/tmp/testfile:\n  file.managed:\n{long_line}");
    let results = run_rules(
        vec![Box::new(formatting::LineTooLongRule)],
        &text,
        FileKind::Sls,
    );

    assert_eq!(results.len(), 1);
    assert_eq!(results[0].id, "204");
}

#[test]
fn irregular_spaces_rule_flags_each_irregular_code_point() {
    for space in formatting::IRREGULAR_SPACES {
        let text = format!("/tmp/testfile:\n  file.managed:\n    - content:{space}\"foobar\"");
        let results = run_rules(
            vec![Box::new(formatting::NoIrregularSpacesRule)],
            &text,
            FileKind::Sls,
        );

        assert_eq!(results.len(), 1, "space {space:?} should match");
        assert_eq!(results[0].id, "212");
    }
}

#[test]
fn cmd_wait_recommend_rule_flags_cmd_wait() {
    let text = "run_postinstall:\n  cmd.wait:\n    - name: /usr/local/bin/postinstall.sh";
    let results = run_rules(
        vec![Box::new(formatting::CmdWaitRecommendRule)],
        text,
        FileKind::Sls,
    );

    assert_eq!(results.len(), 1);
    assert_eq!(results[0].id, "213");
    assert_eq!(results[0].linenumber, 2);
}

#[test]
fn typo_onchanges_rule_matches_all_bad_variants() {
    let text = "testfile:\n  file.managed:\n    - onchange:\n    - onchange_in:\n    - onchange_any:\n    - on_change:\n    - on_change_in:\n    - on_change_any:\n    - on_changes:\n    - on_changes_in:\n    - on_changes_any:";
    let results = run_rules(
        vec![Box::new(formatting::TypoOnchangesRule)],
        text,
        FileKind::Sls,
    );

    assert_eq!(results.len(), 9);
    assert!(results.iter().all(|problem| problem.id == "216"));
}

#[test]
fn typo_require_rule_matches_all_bad_variants() {
    let text =
        "testfile:\n  file.managed:\n    - requires:\n    - requires_in:\n    - requires_any:";
    let results = run_rules(
        vec![Box::new(formatting::TypoRequireRule)],
        text,
        FileKind::Sls,
    );

    assert_eq!(results.len(), 3);
    assert!(results.iter().all(|problem| problem.id == "217"));
}

#[test]
fn jinja_statement_rule_ignores_raw_blocks_and_flags_bad_spacing() {
    let text = "{%- set example='good' +%}\n\n{% raw %}\n{%-set example='ignored'+%}\n{% endraw %}\n{%-set example='bad'+%}";
    let results = run_rules(
        vec![Box::new(jinja::JinjaStatementHasSpacesRule)],
        text,
        FileKind::Sls,
    );

    assert_eq!(results.len(), 1);
    assert_eq!(results[0].id, "202");
    assert_eq!(results[0].linenumber, 6);
}

#[test]
fn jinja_comment_rule_ignores_raw_blocks_and_flags_bad_spacing() {
    let text = "{#- set example='good' +#}\n\n{% raw %}\n{#-set example='ignored'+#}\n{% endraw %}\n{#-set example='bad'+#}";
    let results = run_rules(
        vec![Box::new(jinja::JinjaCommentHasSpacesRule)],
        text,
        FileKind::Sls,
    );

    assert_eq!(results.len(), 1);
    assert_eq!(results[0].id, "209");
    assert_eq!(results[0].linenumber, 6);
}

#[test]
fn deprecation_rules_are_data_driven_and_flag_matching_states() {
    let cases = [
        ("902", "elasticsearch_index.absent"),
        ("903", "virt.reverted"),
        ("904", "virt.saved"),
        ("905", "virt.unpowered"),
        ("906", "docker.absent"),
        ("907", "docker.image_absent"),
        ("908", "docker.image_present"),
        ("909", "docker.mod_watch"),
        ("910", "docker.network_absent"),
        ("911", "docker.network_present"),
        ("912", "docker.running"),
        ("913", "docker.stopped"),
        ("914", "docker.volume_absent"),
        ("915", "docker.volume_present"),
    ];

    for (expected_id, state) in cases {
        let text = format!("example:\n  {state}:\n    - name: example\n\nexample:\n  {state}");
        let results = run_rules(deprecations::all_rules(), &text, FileKind::Sls);

        assert_eq!(results.len(), 2, "state {state} should match twice");
        assert!(results.iter().all(|problem| problem.id == expected_id));
    }
}

#[test]
fn builtin_rule_collection_contains_low_risk_rules_and_deprecations() {
    let collection = salt_lint_rs::rules::builtin_rules();
    assert_eq!(
        collection.len(),
        formatting::builtin_rule_count()
            + jinja::builtin_rule_count()
            + deprecations::builtin_rule_count()
            + salt_lint_rs::rules::files::builtin_rule_count()
            + salt_lint_rs::rules::yaml::builtin_rule_count()
            + salt_lint_rs::rules::fulltext::builtin_rule_count()
    );
}
