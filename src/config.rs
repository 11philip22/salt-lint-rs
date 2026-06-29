use std::collections::{BTreeSet, HashMap};
use std::fs;
use std::io;
use std::path::{Path, PathBuf};

use ignore::gitignore::{Gitignore, GitignoreBuilder};
use serde::Deserialize;

use crate::cli::CliArgs;

pub type ConfigError = Box<dyn std::error::Error + 'static>;

pub struct Config {
    pub exclude_paths: Vec<PathBuf>,
    pub skip_list: BTreeSet<String>,
    pub tags: Vec<String>,
    pub rulesdir: Vec<PathBuf>,
    pub json: bool,
    pub severity: bool,
    cwd: PathBuf,
    rule_ignores: HashMap<String, RuleIgnoreSet>,
}

impl Config {
    pub fn empty(cwd: PathBuf) -> Self {
        Self {
            exclude_paths: Vec::new(),
            skip_list: BTreeSet::new(),
            tags: Vec::new(),
            rulesdir: Vec::new(),
            json: false,
            severity: false,
            cwd,
            rule_ignores: HashMap::new(),
        }
    }

    pub fn from_cli(args: &CliArgs, cwd: &Path) -> Result<Self, ConfigError> {
        let config_path = args.config.clone().or_else(|| discover_config_path(cwd));
        let raw = load_raw_config(config_path.as_deref())?;

        let mut config = Self::empty(cwd.to_path_buf());

        config.exclude_paths = args.exclude_paths.clone();
        config
            .exclude_paths
            .extend(raw.exclude_paths.unwrap_or_default());

        let mut skip_list = args.skip_list.clone();
        skip_list.extend(
            raw.skip_list
                .unwrap_or_default()
                .into_iter()
                .map(|value| match value {
                    ScalarValue::String(value) => value,
                    ScalarValue::Integer(value) => value.to_string(),
                    ScalarValue::Boolean(value) => value.to_string(),
                }),
        );
        config.skip_list = expand_csv_values(skip_list).collect();

        config.tags = args.tags.clone();
        if let Some(tags) = raw.tags {
            config.tags.extend(tags.into_vec());
        }

        config.rulesdir = args.rulesdir.clone();
        config.rulesdir.extend(raw.rulesdir.unwrap_or_default());

        config.json = raw.json.unwrap_or(args.json);
        config.severity = raw.severity.unwrap_or(args.severity);

        for (rule_name, rule_config) in raw.rules.unwrap_or_default() {
            let Some(ignore_patterns) = rule_config.ignore else {
                continue;
            };

            let ignore_set = RuleIgnoreSet::new(cwd, &rule_name, &ignore_patterns)?;
            config.rule_ignores.insert(rule_name, ignore_set);
        }

        Ok(config)
    }

    pub fn is_excluded(&self, path: &Path) -> bool {
        let path = normalize_path_string(path);
        self.exclude_paths.iter().any(|exclude| {
            let raw = normalize_path_string(exclude);
            let absolute = normalize_path_string(self.cwd.join(exclude));
            path.starts_with(&raw) || path.starts_with(&absolute)
        })
    }

    pub fn is_file_ignored(&self, path: &Path, rule: &str) -> bool {
        self.rule_ignores
            .get(rule)
            .is_some_and(|ignore_set| ignore_set.matches(&self.cwd, path))
    }
}

pub fn discover_config_path(start_dir: &Path) -> Option<PathBuf> {
    let mut current = start_dir.to_path_buf();

    loop {
        let candidate = current.join(".salt-lint");
        if candidate.exists() {
            return Some(candidate);
        }

        if current.join(".git").exists() {
            return None;
        }

        if !current.pop() {
            return None;
        }
    }
}

fn load_raw_config(config_path: Option<&Path>) -> Result<RawConfig, ConfigError> {
    let Some(config_path) = config_path else {
        return Ok(RawConfig::default());
    };

    if !config_path.exists() {
        return Ok(RawConfig::default());
    }

    let content = fs::read_to_string(config_path)?;
    Ok(serde_yaml::from_str(&content)?)
}

fn expand_csv_values(values: Vec<String>) -> impl Iterator<Item = String> {
    values.into_iter().flat_map(|value| {
        value
            .split(',')
            .map(str::trim)
            .filter(|item| !item.is_empty())
            .map(ToOwned::to_owned)
            .collect::<Vec<_>>()
    })
}

fn normalize_path_string(path: impl AsRef<Path>) -> String {
    path.as_ref().to_string_lossy().replace('\\', "/")
}

#[derive(Default, Deserialize)]
struct RawConfig {
    exclude_paths: Option<Vec<PathBuf>>,
    skip_list: Option<Vec<ScalarValue>>,
    tags: Option<TagsValue>,
    rulesdir: Option<Vec<PathBuf>>,
    json: Option<bool>,
    severity: Option<bool>,
    rules: Option<HashMap<String, RawRuleConfig>>,
}

#[derive(Deserialize)]
struct RawRuleConfig {
    ignore: Option<String>,
}

#[derive(Deserialize)]
#[serde(untagged)]
enum TagsValue {
    String(String),
    List(Vec<String>),
}

impl TagsValue {
    fn into_vec(self) -> Vec<String> {
        match self {
            Self::String(value) => value
                .split(',')
                .map(str::trim)
                .filter(|item| !item.is_empty())
                .map(ToOwned::to_owned)
                .collect(),
            Self::List(values) => values,
        }
    }
}

#[derive(Deserialize)]
#[serde(untagged)]
enum ScalarValue {
    String(String),
    Integer(i64),
    Boolean(bool),
}

struct RuleIgnoreSet {
    matcher: Gitignore,
}

impl RuleIgnoreSet {
    fn new(cwd: &Path, rule: &str, patterns: &str) -> Result<Self, ConfigError> {
        let mut builder = GitignoreBuilder::new(cwd);

        for pattern in patterns
            .lines()
            .map(str::trim)
            .filter(|line| !line.is_empty())
        {
            builder
                .add_line(None, pattern)
                .map_err(|_| invalid_ignore_pattern(rule, pattern))?;
        }

        Ok(Self {
            matcher: builder
                .build()
                .map_err(|_| invalid_ignore_pattern(rule, patterns))?,
        })
    }

    fn matches(&self, cwd: &Path, path: &Path) -> bool {
        let candidate = if path.is_absolute() {
            path.to_path_buf()
        } else {
            cwd.join(path)
        };

        self.matcher
            .matched_path_or_any_parents(&candidate, false)
            .is_ignore()
    }
}

fn invalid_ignore_pattern(rule: &str, pattern: &str) -> ConfigError {
    io::Error::new(
        io::ErrorKind::InvalidInput,
        format!("invalid ignore pattern for `{rule}`: {pattern}"),
    )
    .into()
}
