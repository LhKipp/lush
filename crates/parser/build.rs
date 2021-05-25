// https://dev.to/cad97/what-is-a-lexer-anyway-4kdo
use {
    glob::glob,
    heck::*,
    serde::{Deserialize, Serialize},
    std::{collections::HashMap, env, error::Error, fs, path::Path},
    tera::{self, Context, Tera, Value},
    toml,
};

/// The manifest directory.
const MANIFEST: &str = env!("CARGO_MANIFEST_DIR");
/// Project-relative path to the syntax metadata.
const SYNTAX_CONFIG: &str = "meta/syntax.toml";
/// Directory containing the Tera templates.
const TEMPLATE_DIR: &str = "templates";

const GENERATED_SRC_DIR: &str = "src/generated";

/// The sytnax kinds enum template.
pub const SYNTAX_KINDS: &str = "syntax_kind.rs";

/// Easy access to the project root path.
fn project_root() -> &'static Path {
    // We take the 2nd ancestor as our crate's manifest is two folders deep.
    Path::new(MANIFEST).ancestors().nth(2).unwrap()
}

/// The structured syntax metadata.
///
/// We derive Serialize to serialize to a Tera config object
/// and Deserialize to deserialize from the metadata file.
#[derive(Serialize, Deserialize)]
struct SyntaxConfig {
    keywords: Vec<String>,
    literals: Vec<RegexToken>,
    punctuation: Vec<PunctuationConfig>,
    tokens: Vec<RegexToken>,
}

/// A punctuation config item, represented in toml as `["character", "name"]`.
///
/// We derive Serialize so the Tera config object has named members,
/// but implement Deserialize manually to deserialize from an unnamed sequence.
///
/// Note that this means `de::from_str(ser::to_string(punctuation_config))`
/// does not work, as the two formats do not line up. This is only ok to do
/// here because these are the _only_ de/serialization tasks we care about.
#[derive(Serialize)]
struct PunctuationConfig {
    character: String,
    name: String,
}

impl<'de> Deserialize<'de> for PunctuationConfig {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        #[derive(Deserialize)]
        #[serde(rename = "PunctuationConfig")]
        struct Helper(String, String);
        // We implement deserialize by just delegating to a helper tuple struct type.
        Helper::deserialize(deserializer).map(|helper| PunctuationConfig {
            character: helper.0,
            name: helper.1,
        })
    }
}

#[derive(Serialize)]
struct RegexToken {
    name: String,
    regex: String,
}

impl<'de> Deserialize<'de> for RegexToken {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        #[derive(Deserialize)]
        #[serde(rename = "PunctuationConfig")]
        struct Helper(String, String);
        // We implement deserialize by just delegating to a helper tuple struct type.
        Helper::deserialize(deserializer).map(|helper| RegexToken {
            name: helper.0,
            regex: helper.1,
        })
    }
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
    let parser_root = root.join("crates/parser");
    let templates = parser_root.join(TEMPLATE_DIR).join("**/*.rs");
    let syntax_config = parser_root.join(SYNTAX_CONFIG);
    // All generated files go into `$OUT_DIR` and are `include!`d from there.
    let out = parser_root.join(GENERATED_SRC_DIR);

    // We have to tell cargo we depend on these files
    // so that cargo will rerun the build script when the files change.
    println!("cargo:rerun-if-changed={}", syntax_config.to_string_lossy());
    for path in glob(&templates.to_string_lossy())? {
        println!("cargo:rerun-if-changed={}", path?.to_string_lossy());
    }

    let tera = {
        // Initialize Tera.
        let mut tera = Tera::new(&root.join(templates).to_string_lossy())?;
        // Add the `camel_case` filter using `heck`.
        tera.register_filter(
            "camel_case",
            make_filter_fn("camel_case", |s: String| s.to_camel_case()),
        );
        // panic!("opt_quoted ");
        tera.register_filter(
            "quoted",
            make_filter_fn("quoted", |s: String| {
                // panic!("opt_quoted {}", s);
                if "{}()[]".contains(&s) {
                    format!("\"{}\"", s)
                } else {
                    s
                }
            }),
        );
        tera
    };

    // Read in the context file.
    let config: SyntaxConfig = toml::from_str(&fs::read_to_string(syntax_config)?)?;
    // And convert it into the Tera-compatible form.
    let context = Context::from_serialize(config)?;

    // Write out the generated file.
    fs::write(out.join(SYNTAX_KINDS), tera.render(SYNTAX_KINDS, &context)?)?;
    Ok(())
}
