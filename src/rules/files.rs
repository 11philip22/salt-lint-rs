use std::path::Path;
use std::sync::LazyLock;

use regex::Regex;

use crate::engine::context::RuleContext;
use crate::engine::rule::{ProblemSeed, Rule};
use crate::file_types::FileKind;
use crate::problem::{RuleMeta, Severity};

const FORMATTING_TAGS: &[&str] = &["formatting"];
const ALL_LANGUAGES: &[FileKind] = &[];
const SLS_LANGUAGES: &[FileKind] = &[FileKind::Sls];

const FILE_EXTENSION_META: RuleMeta = RuleMeta {
    id: "205",
    shortdesc: "Use \".sls\" as a Salt State file extension",
    description: "Salt State files should have the \".sls\" extension",
    severity: Severity::Medium,
    tags: FORMATTING_TAGS,
    languages: ALL_LANGUAGES,
};

const FILE_MODE_QUOTATION_META: RuleMeta = RuleMeta {
    id: "207",
    shortdesc: "File modes should always be encapsulated in quotation marks",
    description: "File modes should always be encapsulated in quotation marks",
    severity: Severity::High,
    tags: FORMATTING_TAGS,
    languages: SLS_LANGUAGES,
};

const FILE_MODE_LEADING_ZERO_META: RuleMeta = RuleMeta {
    id: "208",
    shortdesc: "File modes should always contain a leading zero",
    description: "File modes should always contain a leading zero",
    severity: Severity::Low,
    tags: FORMATTING_TAGS,
    languages: SLS_LANGUAGES,
};

const SLS_FILE_NAME_META: RuleMeta = RuleMeta {
    id: "214",
    shortdesc: "SLS file with a period in the name (besides the suffix period) can not be referenced",
    description: "SLS file with a period in the name (besides the suffix period) can not be referenced",
    severity: Severity::High,
    tags: FORMATTING_TAGS,
    languages: SLS_LANGUAGES,
};

static FILE_MODE_QUOTATION_REGEX: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(
        r#"(?x)^
        \s+
        -\s
        (?:dir_|file_)?mode
        :\s?
        (?:
            (\d{3,4})
            (?:
                ['"]
              |
                \s
              |
                $
            )
          |
            (['"]\d{3,4}(?:[^\d'"]|$))
        )
        "#,
    )
    .expect("valid file mode quotation regex")
});

static FILE_MODE_LEADING_ZERO_REGEX: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(
        r#"(?x)^
        \s+
        -\s
        (?:dir_|file_)?mode
        :\s?
        ['"]?
        ([0-9]{3})
        (?:['"\s]|$)
        "#,
    )
    .expect("valid file mode leading zero regex")
});

pub struct FileExtensionRule;
pub struct FileModeQuotationRule;
pub struct FileModeLeadingZeroRule;
pub struct SlsFileNameRule;

impl Rule for FileExtensionRule {
    fn meta(&self) -> &'static RuleMeta {
        &FILE_EXTENSION_META
    }

    fn check_text(&self, ctx: &RuleContext<'_>, text: &str) -> Vec<ProblemSeed> {
        let extension = Path::new(ctx.filename)
            .extension()
            .and_then(|ext| ext.to_str())
            .map(|ext| ext.to_ascii_lowercase());

        if extension.as_deref() == Some("sls") {
            return Vec::new();
        }

        let first_line = text.lines().next().unwrap_or_default();
        vec![ProblemSeed::line(1, first_line, None::<String>)]
    }
}

impl Rule for FileModeQuotationRule {
    fn meta(&self) -> &'static RuleMeta {
        &FILE_MODE_QUOTATION_META
    }

    fn check_line(
        &self,
        _ctx: &RuleContext<'_>,
        line_no: usize,
        line: &str,
    ) -> Option<ProblemSeed> {
        FILE_MODE_QUOTATION_REGEX
            .is_match(line)
            .then(|| ProblemSeed::line(line_no, line, None::<String>))
    }
}

impl Rule for FileModeLeadingZeroRule {
    fn meta(&self) -> &'static RuleMeta {
        &FILE_MODE_LEADING_ZERO_META
    }

    fn check_line(
        &self,
        _ctx: &RuleContext<'_>,
        line_no: usize,
        line: &str,
    ) -> Option<ProblemSeed> {
        FILE_MODE_LEADING_ZERO_REGEX
            .is_match(line)
            .then(|| ProblemSeed::line(line_no, line, None::<String>))
    }
}

impl Rule for SlsFileNameRule {
    fn meta(&self) -> &'static RuleMeta {
        &SLS_FILE_NAME_META
    }

    fn check_text(&self, ctx: &RuleContext<'_>, text: &str) -> Vec<ProblemSeed> {
        let basename = Path::new(ctx.filename)
            .file_name()
            .and_then(|name| name.to_str())
            .unwrap_or_default();

        if basename.split('.').count() <= 2 {
            return Vec::new();
        }

        let first_line = text.lines().next().unwrap_or_default();
        vec![ProblemSeed::line(1, first_line, None::<String>)]
    }
}

pub fn all_rules() -> Vec<Box<dyn Rule>> {
    vec![
        Box::new(FileExtensionRule),
        Box::new(FileModeQuotationRule),
        Box::new(FileModeLeadingZeroRule),
        Box::new(SlsFileNameRule),
    ]
}
