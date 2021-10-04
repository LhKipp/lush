#[cfg(test)]
mod test {
    use lu_test_support::test_prelude::*;

    #[lu_conformance::tests(exact, serde=serde_json, file="test_data/evaluate/")]
    fn general_interpreter_tests(s: &str) -> LuResults<Value> {
        let mut itprtr = make_test_interpreter();

        itprtr.eval(s.to_string().into())
    }
}
