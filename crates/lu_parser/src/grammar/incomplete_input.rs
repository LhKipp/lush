#[cfg(test)]
mod tests {
    use crate::{parse_as, Event, SourceFileRule};

    use {lu_conformance, serde_yaml};

    #[lu_conformance::tests(exact, serde=serde_yaml, file="test_data/grammar/incomplete_input")]
    fn parse_cmds(s: &str) -> Vec<Event> {
        lu_test_support::init_logger();
        parse_as(s, &SourceFileRule {})
    }

    #[lu_conformance::tests(exact, serde=serde_yaml, file="test_data/grammar/broken_input")]
    fn parse_broken_input(s: &str) -> Vec<Event> {
        lu_test_support::init_logger();
        parse_as(s, &SourceFileRule {})
    }
}
