pub fn parse_noqa_ids(line: &str) -> impl Iterator<Item = &str> {
    line.split_once("# noqa")
        .into_iter()
        .flat_map(|(_, remainder)| remainder.split_whitespace())
}

pub fn text_has_noqa(text: &str, rule_id: &str) -> bool {
    text.lines()
        .any(|line| parse_noqa_ids(line).any(|id| id == rule_id))
}
