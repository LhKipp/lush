use syntax_elements::syntax_elements_as_tera_context;

// https://dev.to/cad97/what-is-a-lexer-anyway-4kdo
use {
    glob::glob,
    std::{env, error::Error, fs, path::Path},
    tera::{self, Tera},
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

fn main() -> Result<(), Box<dyn Error>> {
    let root = project_root();
    let parser_root = root.join("crates/lu_syntax");
    let templates = parser_root.join(TEMPLATE_DIR).join("**/*.rs");
    // All generated files go into `$OUT_DIR` and are `include!`d from there.
    let out = parser_root.join(GENERATED_SRC_DIR);

    // We have to tell cargo we depend on these files
    // so that cargo will rerun the build script when the files change.
    for path in glob(&templates.to_string_lossy())? {
        println!("cargo:rerun-if-changed={}", path?.to_string_lossy());
    }

    // Initialize Tera.
    let tera = Tera::new(&root.join(templates).to_string_lossy())?;

    let context = syntax_elements_as_tera_context()?;
    // Write out the generated file.
    fs::write(out.join(NODES_FILE), tera.render(NODES_FILE, &context)?)?;
    Ok(())
}
