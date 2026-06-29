pub fn parse_noqa_ids(line: &str) -> impl Iterator<Item = &str> {
    line.split_once("# noqa")
        .into_iter()
        .flat_map(|(_, remainder)| remainder.split_whitespace())
}

pub fn text_has_noqa(text: &str, rule_id: &str) -> bool {
    text.lines()
        .any(|line| parse_noqa_ids(line).any(|id| id == rule_id))
}

#[cfg(test)]
mod tests {
    use super::{parse_noqa_ids, text_has_noqa};

    #[test]
    fn parses_noqa_suffix() {
        assert_eq!(
            parse_noqa_ids("x # noqa 201 202")
                .into_iter()
                .collect::<Vec<_>>(),
            vec!["201", "202"]
        );
    }

    #[test]
    fn returns_empty_when_no_noqa_marker_exists() {
        assert!(parse_noqa_ids("x").next().is_none());
    }

    #[test]
    fn detects_noqa_inside_multiline_text() {
        let text = "line one\nline two # noqa 201\nline three";
        assert!(text_has_noqa(text, "201"));
        assert!(!text_has_noqa(text, "202"));
    }
}
