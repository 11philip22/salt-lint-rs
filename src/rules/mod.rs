pub mod deprecations;
pub mod files;
pub mod formatting;
pub mod fulltext;
pub mod jinja;
pub mod yaml;

use crate::engine::collection::RuleCollection;

pub fn builtin_rules() -> RuleCollection {
    let mut collection = RuleCollection::new();

    for rule in formatting::all_rules() {
        collection.register(rule);
    }

    for rule in jinja::all_rules() {
        collection.register(rule);
    }

    for rule in deprecations::all_rules() {
        collection.register(rule);
    }

    for rule in files::all_rules() {
        collection.register(rule);
    }

    for rule in yaml::all_rules() {
        collection.register(rule);
    }

    for rule in fulltext::all_rules() {
        collection.register(rule);
    }

    collection
}
