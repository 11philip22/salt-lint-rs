use std::collections::{BTreeMap, BTreeSet};

use crate::engine::context::RuleContext;
use crate::engine::rule::Rule;
use crate::engine::skip::{parse_noqa_ids, text_has_noqa};
use crate::problem::{Problem, RuleMeta};

#[derive(Default)]
pub struct RuleCollection {
    rules: Vec<Box<dyn Rule>>,
}

impl RuleCollection {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn register(&mut self, rule: Box<dyn Rule>) {
        self.rules.push(rule);
    }

    pub fn len(&self) -> usize {
        self.rules.len()
    }

    pub fn render_rules(&self) -> String {
        let mut metas = self
            .rules
            .iter()
            .map(|rule| rule.meta())
            .collect::<Vec<_>>();
        metas.sort_by(|left, right| left.id.cmp(right.id));
        metas
            .into_iter()
            .map(|meta| format!("{}: {}\n {}", meta.id, meta.shortdesc, meta.description))
            .collect::<Vec<_>>()
            .join("\n")
    }

    pub fn render_tags(&self) -> String {
        let mut tags = BTreeMap::<&str, Vec<&str>>::new();

        for rule in &self.rules {
            let meta = rule.meta();
            for tag in meta.tags {
                tags.entry(tag).or_default().push(meta.id);
            }
        }

        tags.into_iter()
            .map(|(tag, mut ids)| {
                ids.sort();
                let ids = ids
                    .into_iter()
                    .map(|id| format!("[{id}]"))
                    .collect::<Vec<_>>()
                    .join(", ");
                format!("{tag} {ids}")
            })
            .collect::<Vec<_>>()
            .join("\n")
    }

    pub fn run(
        &self,
        context: &RuleContext<'_>,
        text: &str,
        tags: &BTreeSet<String>,
        skip_list: &BTreeSet<String>,
    ) -> Vec<Problem> {
        let mut problems = Vec::new();

        for rule in &self.rules {
            let meta = rule.meta();
            if !rule_enabled(meta, context, tags, skip_list) {
                continue;
            }

            for (index, line) in text.split('\n').enumerate() {
                if line.trim_start().starts_with('#') {
                    continue;
                }

                if parse_noqa_ids(line).any(|id| id == meta.id) {
                    continue;
                }

                if let Some(seed) = rule.check_line(context, index + 1, line) {
                    problems.push(seed.into_problem(meta, context.filename));
                }
            }

            for seed in rule.check_text(context, text) {
                if seed
                    .skip_excerpt
                    .as_deref()
                    .is_some_and(|excerpt| text_has_noqa(excerpt, meta.id))
                {
                    continue;
                }

                problems.push(seed.into_problem(meta, context.filename));
            }
        }

        sort_problems(&mut problems);
        problems
    }
}

fn rule_enabled(
    meta: &'static RuleMeta,
    context: &RuleContext<'_>,
    tags: &BTreeSet<String>,
    skip_list: &BTreeSet<String>,
) -> bool {
    if !meta.languages.is_empty() && !meta.languages.contains(&context.kind) {
        return false;
    }

    let rule_tokens = meta
        .tags
        .iter()
        .copied()
        .chain(std::iter::once(meta.id))
        .collect::<Vec<_>>();

    if !tags.is_empty() && rule_tokens.iter().all(|token| !tags.contains(*token)) {
        return false;
    }

    if rule_tokens.iter().any(|token| skip_list.contains(*token)) {
        return false;
    }

    if rule_tokens.iter().any(|token| {
        context
            .config
            .is_file_ignored(context.filename.as_ref(), token)
    }) {
        return false;
    }

    true
}

pub fn sort_problems(problems: &mut [Problem]) {
    problems.sort_by(|left, right| {
        left.filename
            .cmp(&right.filename)
            .then_with(|| left.linenumber.cmp(&right.linenumber))
            .then_with(|| left.id.cmp(&right.id))
    });
}
