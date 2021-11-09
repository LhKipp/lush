#[cfg(test)]
mod tests {
    use crate::{parse_as, Event, SourceFileRule};

    use {lu_conformance, serde_yaml};

    #[lu_conformance::tests(exact, serde=serde_yaml, file="test_data/grammar/incomplete_input")]
    fn parse_cmds(s: &str) -> Vec<Event> {
        lu_test_support::init_logger();
        parse_as(s, &SourceFileRule {})
    }

    #[test]
    fn parse_broken_input() {
        let s = r"
                fn asf {} ((
                impl Rule for BlockStmtRule {
                    fn matches(&self, p: &mut Parser) -> bool {
                        assert!(self.parse_begin);
                        p.next_non(CMT_NL_WS) == BeginKeyword
                    }

                let x = 1
            ";
        // must return as usual
        parse_as(s, &SourceFileRule {});
        assert!(true);
    }
}
