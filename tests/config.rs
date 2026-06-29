use std::path::{Path, PathBuf};

use clap::Parser;
use salt_lint_rs::cli::CliArgs;
use salt_lint_rs::config::{Config, discover_config_path};
use tempfile::tempdir;

#[test]
fn discovers_nearest_config_file() {
    let tempdir = tempdir().expect("tempdir should be created");
    let project = tempdir.path().join("project");
    let nested = project.join("salt").join("states");
    std::fs::create_dir_all(&nested).expect("nested dir should exist");
    let config_path = project.join(".salt-lint");
    std::fs::write(&config_path, "verbosity: 1\n").expect("config should be written");

    let discovered = discover_config_path(&nested).expect("config should be discovered");

    assert_eq!(discovered, config_path);
}

#[test]
fn stops_searching_at_git_boundary() {
    let tempdir = tempdir().expect("tempdir should be created");
    let outer = tempdir.path().join("outer");
    let project = outer.join("project");
    let nested = project.join("salt").join("states");
    std::fs::create_dir_all(&nested).expect("nested dir should exist");
    std::fs::create_dir_all(project.join(".git")).expect(".git dir should exist");
    std::fs::write(outer.join(".salt-lint"), "verbosity: 2\n").expect("config should be written");

    let discovered = discover_config_path(&nested);

    assert!(discovered.is_none());
}

#[test]
fn merges_yaml_config_with_cli_values() {
    let tempdir = tempdir().expect("tempdir should be created");
    let config_path = tempdir.path().join(".salt-lint");
    std::fs::write(
        &config_path,
        r#"---
verbosity: 1
exclude_paths:
  - exclude_this_file
skip_list:
  - 207
  - "208,209"
tags:
  - formatting
use_default_rules: true
rulesdir:
  - from-config-rules
rules:
  formatting:
    ignore: |
      tests/test-extension-failure
      tests/**/*.jinja
"#,
    )
    .expect("config should be written");

    let args = CliArgs::parse_from([
        "salt-lint",
        "-v",
        "-x",
        "210,211",
        "-t",
        "deprecation",
        "-r",
        "from-cli-rules",
        "--exclude",
        "vendor",
        "-c",
        config_path.to_str().expect("config path should be utf8"),
        "top.sls",
    ]);

    let config = Config::from_cli(&args, tempdir.path()).expect("config should load");

    assert_eq!(
        config.exclude_paths,
        vec![PathBuf::from("vendor"), PathBuf::from("exclude_this_file")]
    );
    assert!(config.skip_list.contains("207"));
    assert!(config.skip_list.contains("208"));
    assert!(config.skip_list.contains("209"));
    assert!(config.skip_list.contains("210"));
    assert!(config.skip_list.contains("211"));
    assert_eq!(
        config.tags,
        vec!["deprecation".to_owned(), "formatting".to_owned()]
    );
    assert_eq!(
        config.rulesdir,
        vec![
            PathBuf::from("from-cli-rules"),
            PathBuf::from("from-config-rules")
        ]
    );
}

#[test]
fn rule_ignore_matching_uses_gitwildmatch_style_patterns() {
    let tempdir = tempdir().expect("tempdir should be created");
    let config_path = tempdir.path().join(".salt-lint");
    std::fs::write(
        &config_path,
        r#"---
rules:
  formatting:
    ignore: |
      tests/test-extension-failure
      tests/**/*.jinja
"#,
    )
    .expect("config should be written");

    let args = CliArgs::parse_from([
        "salt-lint",
        "-c",
        config_path.to_str().expect("config path should be utf8"),
        "top.sls",
    ]);

    let config = Config::from_cli(&args, tempdir.path()).expect("config should load");

    assert!(config.is_file_ignored(Path::new("tests/test-extension-failure"), "formatting"));
    assert!(config.is_file_ignored(Path::new("tests/other/test.jinja"), "formatting"));
    assert!(!config.is_file_ignored(Path::new("test.jinja"), "formatting"));
}

#[test]
fn exclude_paths_match_relative_and_absolute_prefixes() {
    let tempdir = tempdir().expect("tempdir should be created");
    let args = CliArgs::parse_from(["salt-lint", "--exclude", "states", "top.sls"]);

    let config = Config::from_cli(&args, tempdir.path()).expect("config should load");

    assert!(config.is_excluded(Path::new("states/init.sls")));
    assert!(config.is_excluded(&tempdir.path().join("states").join("init.sls")));
    assert!(!config.is_excluded(Path::new("other/init.sls")));
}
