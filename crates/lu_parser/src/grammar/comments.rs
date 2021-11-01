#[cfg(test)]
mod tests {
    use pretty_env_logger::env_logger;

    use crate::{parse_as, Event, SourceFileRule};

    use {conformance, serde_yaml};

    #[conformance::tests(exact, serde=serde_yaml, file="test_data/grammar/comments/comment_fn.yaml_test")]
    fn parse_cmds(s: &str) -> Vec<Event> {
        let _ = env_logger::builder().is_test(true).try_init();
        parse_as(s, &SourceFileRule {})
    }
}
