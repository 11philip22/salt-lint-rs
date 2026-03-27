pub fn parse_noqa_ids(line: &str) -> Vec<&str> {
    let Some((_, remainder)) = line.split_once("# noqa") else {
        return Vec::new();
    };

    remainder.split_whitespace().collect()
}

#[cfg(test)]
mod tests {
    use super::parse_noqa_ids;

    #[test]
    fn parses_noqa_suffix() {
        assert_eq!(parse_noqa_ids("x # noqa 201 202"), vec!["201", "202"]);
    }

    #[test]
    fn returns_empty_when_no_noqa_marker_exists() {
        assert!(parse_noqa_ids("x").is_empty());
    }
}
