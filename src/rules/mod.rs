pub mod deprecations;
pub mod files;
pub mod formatting;
pub mod fulltext;
pub mod jinja;
pub mod yaml;

use crate::engine::collection::RuleCollection;

pub fn builtin_rules() -> RuleCollection {
    let mut collection = RuleCollection::new();
    collection.register(Box::new(formatting::TrailingWhitespaceRule));
    collection.register(Box::new(formatting::LineTooLongRule));
    collection
}
