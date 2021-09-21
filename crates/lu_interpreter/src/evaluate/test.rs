#[cfg(test)]
mod test {
    use lu_error::LuResults;
    use lu_test_support::make_test_interpreter;
    use lu_value::Value;
    use {lu_conformance, serde_json};

    #[lu_conformance::tests(exact, serde=serde_json, file="test_data/evaluate/")]
    fn general_interpreter_tests(s: &str) -> LuResults<Value> {
        let mut itprtr = make_test_interpreter();

        itprtr.eval(s.to_string().into())
    }
}
