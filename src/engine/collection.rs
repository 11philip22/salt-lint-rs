use crate::engine::rule::Rule;

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

    pub fn is_empty(&self) -> bool {
        self.rules.is_empty()
    }
}
