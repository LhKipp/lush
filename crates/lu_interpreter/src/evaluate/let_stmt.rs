use lu_error::LuResult;
use lu_syntax::ast::LetStmtNode;
use lu_value::Value;

use crate::{Evaluable, Interpreter};

impl Evaluable for LetStmtNode {
    fn do_evaluate(&self, _state: &mut Interpreter) -> LuResult<Value> {
        todo!()
    }
}

// #[cfg(test)]
// mod test {
//     use lu_error::LuResult;
//     use lu_syntax::ast::SourceFileNode;
//     use lu_test_support::{init_logger, make_test_interpreter};
//     use lu_text_util::SourceCode;
//     use lu_value::Value;
//     use {conformance, serde_json};

//     #[conformance::tests(exact, serde=serde_json, file="test_data/evaluate/cmd_stmt/general.json_test")]
//     fn general_interpreter_tests(s: &str) -> LuResult<Value> {
//         init_logger();
//         let mut itprt = make_test_interpreter();

//         itprt.evaluate_as::<SourceFileNode>(SourceCode::Text(s.to_string()))
//     }
// }
