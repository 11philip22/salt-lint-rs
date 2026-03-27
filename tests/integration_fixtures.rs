use std::path::PathBuf;
use std::process::{Command, Stdio};

use tempfile::tempdir;

fn fixture(path: &str) -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("tests")
        .join("fixtures")
        .join(path)
}

fn run_binary(args: &[&str]) -> std::process::Output {
    Command::new(env!("CARGO_BIN_EXE_salt-lint"))
        .args(args)
        .output()
        .expect("binary should run")
}

#[test]
fn clean_fixture_exits_zero() {
    let output = run_binary(&[fixture("clean.sls").to_str().unwrap()]);

    assert_eq!(output.status.code(), Some(0));
    assert!(output.stdout.is_empty());
}

#[test]
fn multiple_findings_output_is_stably_sorted() {
    let output = run_binary(&[fixture("multiple_findings.sls").to_str().unwrap()]);

    assert_eq!(output.status.code(), Some(2));
    let stdout = String::from_utf8(output.stdout).expect("stdout should be utf8");
    let first = stdout.find("[201]").expect("201 finding should exist");
    let second = stdout.find("[204]").expect("204 finding should exist");
    let third = stdout.find("[217]").expect("217 finding should exist");
    assert!(first < second && second < third);
}

#[test]
fn raw_block_fixture_exits_zero() {
    let output = run_binary(&[fixture("raw_only.sls").to_str().unwrap()]);

    assert_eq!(output.status.code(), Some(0));
}

#[test]
fn section_noqa_fixture_exits_zero() {
    let output = run_binary(&[fixture("section_noqa.sls").to_str().unwrap()]);

    assert_eq!(output.status.code(), Some(0));
}

#[test]
fn directory_input_reads_init_sls() {
    let output = run_binary(&[fixture("dir_input").to_str().unwrap()]);

    assert_eq!(output.status.code(), Some(2));
    let stdout = String::from_utf8(output.stdout).expect("stdout should be utf8");
    assert!(stdout.contains("[201]"));
}

#[test]
fn stdin_input_is_linted_as_sls() {
    let mut child = Command::new(env!("CARGO_BIN_EXE_salt-lint"))
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .expect("binary should spawn");

    {
        use std::io::Write;
        let stdin = child.stdin.as_mut().expect("stdin should be piped");
        write!(
            stdin,
            "testfile:\n  file.managed:\n    - source:/salt://lorem  "
        )
        .expect("stdin should be writable");
    }

    let output = child
        .wait_with_output()
        .expect("output should be collected");
    assert_eq!(output.status.code(), Some(2));
    let stdout = String::from_utf8(output.stdout).expect("stdout should be utf8");
    assert!(stdout.contains("stdin.sls"));
    assert!(stdout.contains("[201]"));
}

#[test]
fn config_discovery_applies_skip_list() {
    let tempdir = tempdir().expect("tempdir should be created");
    let project = tempdir.path().join("project");
    let nested = project.join("nested");
    std::fs::create_dir_all(&nested).expect("nested dir should exist");
    std::fs::write(project.join(".salt-lint"), "skip_list:\n  - 201\n")
        .expect("config should be written");
    std::fs::write(
        nested.join("state.sls"),
        "test:\n  file.managed:\n    - source: x  ",
    )
    .expect("fixture should be written");

    let output = Command::new(env!("CARGO_BIN_EXE_salt-lint"))
        .arg("state.sls")
        .current_dir(&nested)
        .output()
        .expect("binary should run");

    assert_eq!(output.status.code(), Some(0));
}

#[test]
fn json_output_contains_expected_fields_and_order() {
    let output = run_binary(&["--json", fixture("multiple_findings.sls").to_str().unwrap()]);

    assert_eq!(output.status.code(), Some(2));
    let stdout = String::from_utf8(output.stdout).expect("stdout should be utf8");
    let parsed: serde_json::Value = serde_json::from_str(&stdout).expect("json should parse");
    let items = parsed.as_array().expect("json output should be an array");
    assert_eq!(items.len(), 3);
    assert_eq!(items[0]["id"], "201");
    assert_eq!(items[1]["id"], "204");
    assert_eq!(items[2]["id"], "217");
}

#[test]
fn severity_output_includes_severity_labels() {
    let output = run_binary(&[
        "--severity",
        fixture("multiple_findings.sls").to_str().unwrap(),
    ]);

    assert_eq!(output.status.code(), Some(2));
    let stdout = String::from_utf8(output.stdout).expect("stdout should be utf8");
    assert!(stdout.contains("[INFO]"));
    assert!(stdout.contains("[VERY_LOW]"));
    assert!(stdout.contains("[LOW]"));
}

#[test]
fn unsupported_rulesdir_warning_is_emitted() {
    let output = run_binary(&["-r", "custom-rules", fixture("clean.sls").to_str().unwrap()]);

    assert_eq!(output.status.code(), Some(0));
    let stderr = String::from_utf8(output.stderr).expect("stderr should be utf8");
    assert!(stderr.contains("custom Python rule directories are unsupported"));
}

#[test]
fn list_rules_outputs_real_builtin_rules() {
    let output = run_binary(&["-L"]);

    assert_eq!(output.status.code(), Some(0));
    let stdout = String::from_utf8(output.stdout).expect("stdout should be utf8");
    assert!(stdout.contains("201: Trailing whitespace"));
    assert!(stdout.contains("901: Using the quiet argument with cmd.run is deprecated"));
}

#[test]
fn list_tags_outputs_real_builtin_tags() {
    let output = run_binary(&["-T"]);

    assert_eq!(output.status.code(), Some(0));
    let stdout = String::from_utf8(output.stdout).expect("stdout should be utf8");
    assert!(stdout.contains("deprecation"));
    assert!(stdout.contains("[901]"));
    assert!(stdout.contains("formatting"));
}

#[test]
fn no_input_still_exits_one_without_piped_stdin() {
    let output = run_binary(&[]);

    assert_eq!(output.status.code(), Some(1));
}
