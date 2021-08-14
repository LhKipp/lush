use syntax_elements::syntax_elements_as_tera_context;

// https://dev.to/cad97/what-is-a-lexer-anyway-4kdo
use {
    glob::glob,
    heck::*,
    std::{collections::HashMap, env, error::Error, fs, path::Path},
    tera::{self, Tera, Value},
};

/// The manifest directory.
const MANIFEST: &str = env!("CARGO_MANIFEST_DIR");
/// Directory containing the Tera templates.
const TEMPLATE_DIR: &str = "templates";

const GENERATED_SRC_DIR: &str = "src/ast/generated";

/// The sytnax kinds enum template.
pub const NODES_FILE: &str = "nodes.rs";

/// Easy access to the project root path.
fn project_root() -> &'static Path {
    // We take the 2nd ancestor as our crate's manifest is two folders deep.
    Path::new(MANIFEST).ancestors().nth(2).unwrap()
}

/// A helper function to make Tera filter functions `(value, keys) -> Value`
/// out of a simpler `(T) -> T` transformation.
fn make_filter_fn<'a, T: Into<Value> + serde::de::DeserializeOwned>(
    name: &'a str,
    f: impl Fn(T) -> T + Sync + Send + 'a,
) -> impl tera::Filter + 'a {
    move |value: &Value, _: &HashMap<String, Value>| -> tera::Result<Value> {
        let val = tera::try_get_value!(name, "value", T, value);
        Ok(f(val).into())
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let root = project_root();
    let parser_root = root.join("crates/syntax");
    let templates = parser_root.join(TEMPLATE_DIR).join("**/*.rs");
    // All generated files go into `$OUT_DIR` and are `include!`d from there.
    let out = parser_root.join(GENERATED_SRC_DIR);

    // We have to tell cargo we depend on these files
    // so that cargo will rerun the build script when the files change.
    for path in glob(&templates.to_string_lossy())? {
        println!("cargo:rerun-if-changed={}", path?.to_string_lossy());
    }

    let tera = {
        // Initialize Tera.
        let mut tera = Tera::new(&root.join(templates).to_string_lossy())?;
        // Add the `camel_case` filter using `heck`.
        tera.register_filter(
            "to_syntax_kind_name",
            make_filter_fn("to_syntax_kind_name", |s: String| s.to_camel_case()),
        );
        tera.register_filter(
            "to_node_name",
            make_filter_fn("to_node_name", |s: String| s.to_camel_case() + "Node"),
        );
        tera
    };

    let context = syntax_elements_as_tera_context()?;
    // Write out the generated file.
    fs::write(out.join(NODES_FILE), tera.render(NODES_FILE, &context)?)?;
    Ok(())
}
