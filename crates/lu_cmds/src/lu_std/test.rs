#[cfg(test)]
mod test {
    use lu_test_support::test_prelude::*;

    #[lu_conformance::tests(exact, serde=serde_json, file="test_data/lu_std")]
    fn std_tests(s: &str) -> LuResults<Value> {
        let (global_frame, itprt_cfg) = make_test_interpreter();
        Interpreter::eval_for_tests(s.to_string().into(), global_frame, &itprt_cfg)
    }
}
