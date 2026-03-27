# salt-lint-rs

Rust port of the built-in `salt-lint` ruleset.

Current scope:

- all 32 built-in `salt-lint` rules are implemented in Rust
- file, directory, and stdin inputs are supported
- default, JSON, and severity output modes are supported
- `.salt-lint` config discovery and merge behavior are supported

Not supported:

- external Python rule plugins
- dynamic loading of custom rule directories

Compatibility behavior for `-r` and `rulesdir`:

- the flags and config fields are accepted
- custom Python rule directories are ignored
- the binary emits a warning to stderr when they are provided

## Usage

```bash
cargo run -- path/to/state.sls
cargo run -- --json path/to/state.sls
cargo run -- -L
```

## Development

```bash
cargo fmt
cargo test
```
