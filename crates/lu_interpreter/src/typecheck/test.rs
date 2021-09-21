#[cfg(test)]
mod test {
    use lu_error::LuResults;
    use lu_test_support::make_test_interpreter;

    use lu_interpreter::ValueType;
    use {lu_conformance, serde_json};

    #[lu_conformance::tests(exact, serde=serde_json, file="test_data/typecheck")]
    fn general_interpreter_tests(s: &str) -> LuResults<Option<ValueType>> {
        let mut itprtr = make_test_interpreter();

        itprtr
            .ty_check(s.to_string().into())
            .map(|ty_checker| ty_checker.result)
    }
}
