use crate::engine::context::RuleContext;
use crate::engine::rule::{ProblemSeed, Rule};
use crate::file_types::FileKind;
use crate::problem::{RuleMeta, Severity};

const SLS_LANGUAGE: &[FileKind] = &[FileKind::Sls];
const DEPRECATION_TAGS: &[&str] = &["deprecation"];

macro_rules! deprecation_meta {
    ($name:ident, $id:literal, $state:literal, $since:literal) => {
        const $name: RuleMeta = RuleMeta {
            id: $id,
            shortdesc: concat!(
                "State '",
                $state,
                "' is deprecated since SaltStack version '",
                $since,
                "'"
            ),
            description: concat!(
                "State '",
                $state,
                "' is deprecated since SaltStack version '",
                $since,
                "'"
            ),
            severity: Severity::High,
            tags: DEPRECATION_TAGS,
            languages: SLS_LANGUAGE,
        };
    };
}

deprecation_meta!(
    ELASTICSEARCH_INDEX_ABSENT_META,
    "902",
    "elasticsearch_index.absent",
    "2017.7.0"
);
deprecation_meta!(VIRT_REVERTED_META, "903", "virt.reverted", "2016.3.0");
deprecation_meta!(VIRT_SAVED_META, "904", "virt.saved", "2016.3.0");
deprecation_meta!(VIRT_UNPOWERED_META, "905", "virt.unpowered", "2016.3.0");
deprecation_meta!(DOCKER_ABSENT_META, "906", "docker.absent", "2017.7.0");
deprecation_meta!(
    DOCKER_IMAGE_ABSENT_META,
    "907",
    "docker.image_absent",
    "2017.7.0"
);
deprecation_meta!(
    DOCKER_IMAGE_PRESENT_META,
    "908",
    "docker.image_present",
    "2017.7.0"
);
deprecation_meta!(DOCKER_MOD_WATCH_META, "909", "docker.mod_watch", "2017.7.0");
deprecation_meta!(
    DOCKER_NETWORK_ABSENT_META,
    "910",
    "docker.network_absent",
    "2017.7.0"
);
deprecation_meta!(
    DOCKER_NETWORK_PRESENT_META,
    "911",
    "docker.network_present",
    "2017.7.0"
);
deprecation_meta!(DOCKER_RUNNING_META, "912", "docker.running", "2017.7.0");
deprecation_meta!(DOCKER_STOPPED_META, "913", "docker.stopped", "2017.7.0");
deprecation_meta!(
    DOCKER_VOLUME_ABSENT_META,
    "914",
    "docker.volume_absent",
    "2017.7.0"
);
deprecation_meta!(
    DOCKER_VOLUME_PRESENT_META,
    "915",
    "docker.volume_present",
    "2017.7.0"
);

#[derive(Clone, Copy)]
struct DeprecationRule {
    meta: &'static RuleMeta,
    state: &'static str,
}

impl Rule for DeprecationRule {
    fn meta(&self) -> &'static RuleMeta {
        self.meta
    }

    fn check_line(
        &self,
        _ctx: &RuleContext<'_>,
        line_no: usize,
        line: &str,
    ) -> Option<ProblemSeed> {
        let candidate = line.strip_prefix("  ")?;
        let remainder = candidate.strip_prefix(self.state)?;

        if remainder.is_empty() || remainder.starts_with(':') {
            Some(ProblemSeed::line(line_no, line, None::<String>))
        } else {
            None
        }
    }
}

const DEPRECATION_DEFS: &[DeprecationRule] = &[
    DeprecationRule {
        meta: &ELASTICSEARCH_INDEX_ABSENT_META,
        state: "elasticsearch_index.absent",
    },
    DeprecationRule {
        meta: &VIRT_REVERTED_META,
        state: "virt.reverted",
    },
    DeprecationRule {
        meta: &VIRT_SAVED_META,
        state: "virt.saved",
    },
    DeprecationRule {
        meta: &VIRT_UNPOWERED_META,
        state: "virt.unpowered",
    },
    DeprecationRule {
        meta: &DOCKER_ABSENT_META,
        state: "docker.absent",
    },
    DeprecationRule {
        meta: &DOCKER_IMAGE_ABSENT_META,
        state: "docker.image_absent",
    },
    DeprecationRule {
        meta: &DOCKER_IMAGE_PRESENT_META,
        state: "docker.image_present",
    },
    DeprecationRule {
        meta: &DOCKER_MOD_WATCH_META,
        state: "docker.mod_watch",
    },
    DeprecationRule {
        meta: &DOCKER_NETWORK_ABSENT_META,
        state: "docker.network_absent",
    },
    DeprecationRule {
        meta: &DOCKER_NETWORK_PRESENT_META,
        state: "docker.network_present",
    },
    DeprecationRule {
        meta: &DOCKER_RUNNING_META,
        state: "docker.running",
    },
    DeprecationRule {
        meta: &DOCKER_STOPPED_META,
        state: "docker.stopped",
    },
    DeprecationRule {
        meta: &DOCKER_VOLUME_ABSENT_META,
        state: "docker.volume_absent",
    },
    DeprecationRule {
        meta: &DOCKER_VOLUME_PRESENT_META,
        state: "docker.volume_present",
    },
];

pub fn all_rules() -> Vec<Box<dyn Rule>> {
    DEPRECATION_DEFS
        .iter()
        .copied()
        .map(|rule| Box::new(rule) as Box<dyn Rule>)
        .collect()
}
