use std::process::Command;

use clap::Parser;
use salt_lint_rs::app::App;
use salt_lint_rs::cli::CliArgs;
use salt_lint_rs::config::Config;
use salt_lint_rs::file_types::FileKind;
use salt_lint_rs::formatter::FormatterKind;
use salt_lint_rs::fs::{map_input_path, resolve_input_files};
use tempfile::tempdir;

#[test]
fn parses_supported_flags() {
    let args = CliArgs::parse_from([
        "salt-lint",
        "-L",
        "-T",
        "-v",
        "-v",
        "-R",
        "--json",
        "--severity",
        "--nocolor",
        "--force-color",
        "-t",
        "formatting",
        "-x",
        "206",
        "-r",
        "rules",
        "--exclude",
        "vendor",
        "-c",
        ".salt-lint",
        "top.sls",
    ]);

    assert!(args.list_rules);
    assert!(args.list_tags);
    assert_eq!(args.verbosity, 2);
    assert!(args.use_default_rules);
    assert!(args.json);
    assert!(args.severity);
    assert!(args.no_color);
    assert!(args.force_color);
    assert_eq!(args.tags, vec!["formatting"]);
    assert_eq!(args.skip_list, vec!["206"]);
    assert_eq!(args.rulesdir.len(), 1);
    assert_eq!(args.exclude_paths.len(), 1);
    assert_eq!(args.files.len(), 1);
    assert_eq!(
        args.config.as_deref(),
        Some(std::path::Path::new(".salt-lint"))
    );
}

#[test]
fn no_input_returns_exit_code_one_and_help_text() {
    let args = CliArgs::parse_from(["salt-lint"]);
    let mut stdout = Vec::new();
    let mut stderr = Vec::new();

    let exit_code = App::default()
        .run(args, &mut stdout, &mut stderr)
        .expect("app run should succeed");

    assert_eq!(exit_code, 1);
    assert!(stdout.is_empty());
    let stderr = String::from_utf8(stderr).expect("stderr should be utf8");
    assert!(stderr.contains("Usage:"));
}

#[test]
fn file_kind_detection_matches_python_extensions() {
    assert_eq!(FileKind::detect("state.sls"), FileKind::Sls);
    assert_eq!(FileKind::detect("state.jinja"), FileKind::Jinja);
    assert_eq!(FileKind::detect("state.jinja2"), FileKind::Jinja);
    assert_eq!(FileKind::detect("state.j2"), FileKind::Jinja);
    assert_eq!(FileKind::detect("state.txt"), FileKind::Unknown);
}

#[test]
fn formatter_priority_prefers_json_over_severity() {
    assert_eq!(FormatterKind::from_flags(true, true), FormatterKind::Json);
    assert_eq!(
        FormatterKind::from_flags(false, true),
        FormatterKind::Severity
    );
}

#[test]
fn binary_help_succeeds() {
    let output = Command::new(env!("CARGO_BIN_EXE_salt-lint"))
        .arg("--help")
        .output()
        .expect("help command should run");

    assert!(output.status.success());
    let stdout = String::from_utf8(output.stdout).expect("stdout should be utf8");
    assert!(stdout.contains("Lint Salt state files"));
}

#[test]
fn binary_version_succeeds() {
    let output = Command::new(env!("CARGO_BIN_EXE_salt-lint"))
        .arg("--version")
        .output()
        .expect("version command should run");

    assert!(output.status.success());
    let stdout = String::from_utf8(output.stdout).expect("stdout should be utf8");
    assert!(stdout.contains("salt-lint"));
}

#[test]
fn binary_without_args_exits_with_one() {
    let output = Command::new(env!("CARGO_BIN_EXE_salt-lint"))
        .output()
        .expect("binary should run");

    assert_eq!(output.status.code(), Some(1));
    let stderr = String::from_utf8(output.stderr).expect("stderr should be utf8");
    assert!(stderr.contains("Usage:"));
}

#[test]
fn directory_input_maps_to_init_sls() {
    let tempdir = tempdir().expect("tempdir should be created");
    let cwd = tempdir.path();
    let states_dir = cwd.join("states");
    std::fs::create_dir(&states_dir).expect("states dir should exist");

    let mapped = map_input_path(std::path::Path::new("states"), cwd);

    assert_eq!(mapped, std::path::PathBuf::from("states").join("init.sls"));
}

#[test]
fn duplicate_inputs_are_suppressed() {
    let tempdir = tempdir().expect("tempdir should be created");
    let config = Config::empty(tempdir.path().to_path_buf());
    let files = resolve_input_files(
        &[
            std::path::PathBuf::from("top.sls"),
            std::path::PathBuf::from("top.sls"),
        ],
        tempdir.path(),
        &config,
    );

    assert_eq!(files.len(), 1);
    assert_eq!(files[0].path, std::path::PathBuf::from("top.sls"));
}

#[test]
fn app_warns_about_unsupported_rulesdir() {
    let tempdir = tempdir().expect("tempdir should be created");
    let args = CliArgs::parse_from(["salt-lint", "-r", "custom-rules", "top.sls"]);
    let mut stdout = Vec::new();
    let mut stderr = Vec::new();

    let exit_code = App::with_current_dir(tempdir.path())
        .run(args, &mut stdout, &mut stderr)
        .expect("app run should succeed");

    assert_eq!(exit_code, 0);
    let stderr = String::from_utf8(stderr).expect("stderr should be utf8");
    assert!(stderr.contains("custom Python rule directories are unsupported"));
    assert!(stderr.contains("custom-rules"));
}
