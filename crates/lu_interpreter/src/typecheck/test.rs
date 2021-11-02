#[cfg(test)]
mod test {
    use lu_test_support::test_prelude::*;

    #[lu_conformance::tests(exact, serde=serde_json, file="test_data/typecheck")]
    fn general_interpreter_tests(s: &str) -> LuResults<Option<ValueType>> {
        let (global_frame, itprt_cfg) = make_test_interpreter();
        Interpreter::ty_check_for_tests(s.to_string().into(), global_frame, &itprt_cfg)
    }
}
