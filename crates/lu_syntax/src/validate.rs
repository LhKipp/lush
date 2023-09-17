use log::debug;
use lu_error::LuErr;

use crate::ast::SourceFileNode;

pub mod cmd_signature;

pub fn validate(node: &SourceFileNode) -> Vec<LuErr> {
    let all = cmd_signature::validate_sign(node);
    debug!("Ast-Validation found {} errors", all.len());
    all
}

#[cfg(test)]
mod tests {
    use crate::Parse;
    use lu_conformance;
    use lu_error::LuResults;
    use lu_test_support::init_logger;

    #[lu_conformance::tests(exact, serde=serde_json, file="test_data/validate")]
    fn parse_cmds(s: &str) -> LuResults<i32> {
        init_logger();
        let result = Parse::source_file(s.to_string().into())
            .map(|_| 1)
            .as_results();
        result
    }
}
