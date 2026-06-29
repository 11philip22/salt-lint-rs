# salt-lint-rs

Rust implementation of the built-in [`salt-lint`](https://github.com/warpnet/salt-lint)
ruleset for Salt state files.

`salt-lint-rs` ships a `salt-lint` CLI that checks `.sls` and Jinja files with the
32 built-in rules, supports file, directory, and stdin input, and can print default,
JSON, or severity-tagged output.

## Features

- Implements all 32 built-in `salt-lint` rules
- Lints files, directories mapped to `init.sls`, and stdin
- Supports rule/tag filtering, skip lists, excludes, and `.salt-lint` config files
- Prints default text output, JSON output, or severity labels
- Lists built-in rules and tags from the CLI

> [!NOTE]
> Python rule plugins and dynamic custom rule directories are not supported. The
> `-r` flag and `rulesdir` config key are accepted for compatibility, ignored, and
> reported as warnings.

## Quick Start

Run against a Salt state file:

```bash
cargo run -- path/to/state.sls
```

Read from stdin:

```bash
cat path/to/state.sls | cargo run --
```

Print JSON output:

```bash
cargo run -- --json path/to/state.sls
```

List available rules and tags:

```bash
cargo run -- -L
cargo run -- -T
```

## Usage

```text
salt-lint [OPTIONS] [FILE]...
```

Common options:

| Option | Description |
| --- | --- |
| `-L`, `--list-rules` | List built-in rules |
| `-T`, `--list-tags` | List built-in tags |
| `-t <TAG>` | Run only rules matching a tag or rule id |
| `-x <RULE>` | Skip a rule id or tag |
| `--exclude <PATH>` | Skip paths with the given prefix |
| `--json` | Emit JSON output |
| `--severity` | Include severity labels in text output |
| `-c <CONFIG>` | Load a specific config file |

Exit codes:

| Code | Meaning |
| --- | --- |
| `0` | No findings |
| `1` | No input was provided |
| `2` | Findings were reported or an error occurred |

## Configuration

By default, `salt-lint` searches for a `.salt-lint` file from the current
directory upward and stops at a `.git` boundary. CLI values are merged with config
values.

Example:

```yaml
exclude_paths:
  - vendor
  - generated

skip_list:
  - 201
  - "207,208"

tags:
  - formatting

json: false
severity: true

rules:
  formatting:
    ignore: |
      tests/fixtures/**
      legacy/**/*.sls
```

Rule ignore patterns use gitwildmatch-style matching.

## Development

```bash
cargo fmt
cargo test
```

Useful local checks:

```bash
cargo run -- tests/fixtures/multiple_findings.sls
cargo run -- --json tests/fixtures/multiple_findings.sls
```
