#[cfg(test)]
mod tests {
    use log::debug;
    use lu_test_support::init_logger;
    use lu_text_util::SourceCode;

    use crate::Parse;
    use serde::{Deserialize, Serialize};
    use {lu_conformance, toml};

    #[derive(Serialize, Deserialize)]
    struct Tree {
        pub tree: String,
    }

    #[lu_conformance::tests(exact, serde=toml, file="test_data/ast")]
    fn parse_cmds(s: &str) -> Tree {
        init_logger();
        let src = SourceCode::new_text(s.to_string());
        let parse = Parse::source_file(src);
        let tree = parse
            .map(|parse| {
                let s = format!("\n{:#?}\n", parse.sf_node);
                debug!("{}", s);
                s
            })
            .unwrap();
        Tree { tree }
    }
}
