use std::collections::BTreeSet;

use salt_lint_rs::config::Config;
use salt_lint_rs::engine::collection::RuleCollection;
use salt_lint_rs::engine::context::RuleContext;
use salt_lint_rs::engine::rule::Rule;
use salt_lint_rs::file_types::FileKind;
use salt_lint_rs::rules::{files, fulltext, jinja, yaml};
use tempfile::tempdir;

fn run_rules_with_path(
    rules: Vec<Box<dyn Rule>>,
    path: &str,
    kind: FileKind,
    text: &str,
) -> Vec<salt_lint_rs::problem::Problem> {
    let tempdir = tempdir().expect("tempdir should be created");
    let config = Config::empty(tempdir.path().to_path_buf());
    let context = RuleContext::new(path, kind, &config);
    let mut collection = RuleCollection::new();

    for rule in rules {
        collection.register(rule);
    }

    collection.run(&context, text, &BTreeSet::new(), &BTreeSet::new())
}

#[test]
fn file_extension_rule_flags_non_sls_files() {
    let positive = run_rules_with_path(
        vec![Box::new(files::FileExtensionRule)],
        "tests/test-extension-success.sls",
        FileKind::Sls,
        "state:\n  file.managed:\n",
    );
    let negative = run_rules_with_path(
        vec![Box::new(files::FileExtensionRule)],
        "tests/test-extension-failure",
        FileKind::Unknown,
        "state:\n  file.managed:\n",
    );

    assert!(positive.is_empty());
    assert_eq!(negative.len(), 1);
    assert_eq!(negative[0].id, "205");
}

#[test]
fn sls_file_name_rule_flags_extra_periods_only_for_sls_files() {
    let good = run_rules_with_path(
        vec![Box::new(files::SlsFileNameRule)],
        "tests/test-extension-success.sls",
        FileKind::Sls,
        "state:\n  file.managed:\n",
    );
    let jinja = run_rules_with_path(
        vec![Box::new(files::SlsFileNameRule)],
        "tests/test-extension-failure.extra.jinja",
        FileKind::Jinja,
        "state:\n  file.managed:\n",
    );
    let bad = run_rules_with_path(
        vec![Box::new(files::SlsFileNameRule)],
        "tests/test-extension-failure.extra.sls",
        FileKind::Sls,
        "state:\n  file.managed:\n",
    );

    assert!(good.is_empty());
    assert!(jinja.is_empty());
    assert_eq!(bad.len(), 1);
    assert_eq!(bad[0].id, "214");
}

#[test]
fn jinja_variable_rule_handles_raw_blocks_and_edge_cases() {
    let good = run_rules_with_path(
        vec![Box::new(jinja::JinjaVariableHasSpacesRule)],
        "inline.sls",
        FileKind::Sls,
        "{{- variable +}}\n{% raw %}\n{{variable}}\n{% endraw %}\n{{ \"{{0}}\" }}\n    - name: '{${{ key }}}'\n",
    );
    let bad = run_rules_with_path(
        vec![Box::new(jinja::JinjaVariableHasSpacesRule)],
        "inline.sls",
        FileKind::Sls,
        "{% raw %}\n{{variable}}\n{% endraw %}\n{{variable}}  # line 4\n{{-variable0+}}\n{{ variable0}}\n{{ \"{{0}}\"}}\n{{\"{{0}}\" }}\n",
    );

    assert!(good.is_empty());
    assert_eq!(bad.len(), 5);
    assert_eq!(bad[0].linenumber, 4);
    assert!(bad.iter().all(|problem| problem.id == "206"));
}

#[test]
fn file_mode_quotation_rule_matches_unquoted_and_malformed_quotes() {
    let good = run_rules_with_path(
        vec![Box::new(files::FileModeQuotationRule)],
        "inline.sls",
        FileKind::Sls,
        "testfile:\n  file.managed:\n    - mode: '0700'\n",
    );
    let bad = run_rules_with_path(
        vec![Box::new(files::FileModeQuotationRule)],
        "inline.sls",
        FileKind::Sls,
        "testfile:\n  file.managed:\n    - mode: 0700\n    - file_mode: 0660\n    - dir_mode: 0775\n    - mode: \"0700\n    - file_mode: '0660\n    - dir_mode: 0775\"\n",
    );
    let network = run_rules_with_path(
        vec![Box::new(files::FileModeQuotationRule)],
        "inline.sls",
        FileKind::Sls,
        "bond0:\n  network.managed:\n    - mode: 802.3ad\n",
    );

    assert!(good.is_empty());
    assert_eq!(bad.len(), 6);
    assert!(network.is_empty());
}

#[test]
fn file_mode_leading_zero_rule_ignores_network_mode_values() {
    let good = run_rules_with_path(
        vec![Box::new(files::FileModeLeadingZeroRule)],
        "inline.sls",
        FileKind::Sls,
        "testfile:\n  file.managed:\n    - mode: '0700'\n",
    );
    let bad = run_rules_with_path(
        vec![Box::new(files::FileModeLeadingZeroRule)],
        "inline.sls",
        FileKind::Sls,
        "testfile:\n  file.managed:\n    - mode: 600\n    - file_mode: '660'\n    - dir_mode: '775'\n",
    );
    let network = run_rules_with_path(
        vec![Box::new(files::FileModeLeadingZeroRule)],
        "inline.sls",
        FileKind::Sls,
        "bond0:\n  network.managed:\n    - mode: 802.3ad\n",
    );

    assert!(good.is_empty());
    assert_eq!(bad.len(), 3);
    assert!(network.is_empty());
}

#[test]
fn yaml_octal_rule_avoids_mac_and_time_false_positives() {
    let good = run_rules_with_path(
        vec![Box::new(yaml::YamlHasOctalValueRule)],
        "inline.sls",
        FileKind::Sls,
        "apache_disable_default_site:\n  apache_site.disabled:\n    - name: 000-default\ninfoblox_remove_record1:\n  infoblox_host_record.absent:\n    - mac: 4c:f2:d3:1b:2e:05\ninfoblox_remove_record2:\n  infoblox_host_record.absent:\n    - mac: 05:f2:d3:1b:2e:4c\nsome_calendar_entry:\n  file.managed:\n    - contents: |\n        oncalendar=Sun 18:00\n",
    );
    let bad = run_rules_with_path(
        vec![Box::new(yaml::YamlHasOctalValueRule)],
        "inline.sls",
        FileKind::Sls,
        "testdirectory:\n  file.recurse:\n    - file_mode: 00\n    - dir_mode: 0700\ntestdirectory2:\n  file.recurse:\n    - file_mode: 00 # COMMENT\n    - dir_mode:0700{# JINJA COMMENT #}\n",
    );

    assert!(good.is_empty());
    assert_eq!(bad.len(), 4);
    assert!(bad.iter().all(|problem| problem.id == "210"));
}

#[test]
fn jinja_pillar_grains_rule_flags_broken_get_syntax() {
    let good = run_rules_with_path(
        vec![Box::new(jinja::JinjaPillarGrainsGetFormatRule)],
        "inline.sls",
        FileKind::Sls,
        "{{ salt['pillar.get']('item') }}\n{{ pillar.get('item') }}\n{{ pillar['item'] }}\n{{ salt['grains.get']('saltversion') }}\n{{ grains.get('saltversion') }}\n{{ grains['saltversion'] }}\n",
    );
    let bad = run_rules_with_path(
        vec![Box::new(jinja::JinjaPillarGrainsGetFormatRule)],
        "inline.sls",
        FileKind::Sls,
        "{{ pillar.get['item'] }}\n{{ grains.get['saltversion'] }}\n",
    );

    assert!(good.is_empty());
    assert_eq!(bad.len(), 2);
    assert!(bad.iter().all(|problem| problem.id == "211"));
}

#[test]
fn nested_dict_rule_preserves_line_numbers_and_noqa_behavior() {
    let good = run_rules_with_path(
        vec![Box::new(fulltext::NestedDictRule)],
        "inline.sls",
        FileKind::Sls,
        "/etc/http/conf/http.conf:\n  file.managed:\n    - source: salt://apache/http.conf\n    - template: jinja\n    - context:\n        custom_var: \"override\"\n    - defaults:\n        custom_var: \"default value\"\n        other_var: 123\n\n/etc/http/conf/http.conf:\n  file.managed:\n    - template: jinja\n    - context:\n      custom_var: \"override\"  # noqa: 219\n    - defaults:\n        custom_var: \"default value\"\n        other_var: 123\n",
    );
    let bad = run_rules_with_path(
        vec![Box::new(fulltext::NestedDictRule)],
        "inline.sls",
        FileKind::Sls,
        "/etc/http/conf/http.conf:\n  file.managed:\n    - source: salt://apache/http.conf\n    - template: jinja\n    - context:\n      custom_var: \"override\"\n    - defaults:\n        custom_var: \"default value\"\n        other_var: 123\n\n/etc/http/conf/http.conf:\n  file.managed:\n    - source: salt://apache/http.conf\n    - template: jinja\n    - context:\n        custom_var: \"override\"\n    - defaults:\n      custom_var: \"default value\"\n      other_var: 123\n\n/etc/http/conf/http.conf:\n  file.managed:\n    - source: salt://apache/http.conf\n    - template: jinja\n    - context:\n      custom_var: \"override\"\n    - defaults:\n      custom_var: \"default value\"\n      other_var: 123\n",
    );

    assert!(good.is_empty());
    assert_eq!(bad.len(), 4);
    assert_eq!(bad[0].linenumber, 6);
    assert_eq!(bad[1].linenumber, 18);
    assert_eq!(bad[2].linenumber, 26);
    assert_eq!(bad[3].linenumber, 28);
}

#[test]
fn cmd_run_quiet_rule_preserves_line_numbers_and_section_skip() {
    let good = run_rules_with_path(
        vec![Box::new(fulltext::CmdRunQuietRule)],
        "inline.sls",
        FileKind::Sls,
        "getpip:\n  cmd.run:\n    - name: /usr/bin/python /usr/local/sbin/get-pip.py\n    - output_loglevel: quiet\n",
    );
    let bad = run_rules_with_path(
        vec![Box::new(fulltext::CmdRunQuietRule)],
        "inline.sls",
        FileKind::Sls,
        "getpip:\n  cmd.run:\n    - name: /usr/bin/python /usr/local/sbin/get-pip.py\n    - unless: which pip\n    - require:\n      - pkg: python\n      - file: /usr/local/sbin/get-pip.py\n    - quiet  # This is the eighth line\n\ngetpip2:\n  cmd.run:\n    - name: /usr/bin/python /usr/local/sbin/get-pip.py\n    - quiet\n\ngetpip3:\n  cmd.run:\n    - name: /usr/bin/python /usr/local/sbin/get-pip.py\n    - quiet # noqa: 901\n\nget_pip_jinja:\n  cmd.run:\n    - name: /usr/bin/python /usr/local/sbin/get-pip.py\n{% if pillar.get('quiet') %}\n    - quiet\n{% endif %}\n",
    );

    assert!(good.is_empty());
    assert_eq!(bad.len(), 3);
    assert_eq!(bad[0].linenumber, 8);
    assert_eq!(bad[1].linenumber, 13);
    assert_eq!(bad[2].linenumber, 25);
}

#[test]
fn builtin_rule_collection_now_contains_all_rules() {
    let collection = salt_lint_rs::rules::builtin_rules();
    assert_eq!(collection.len(), 32);
}
