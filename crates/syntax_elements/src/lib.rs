use std::{error::Error, fs};

use serde::{Deserialize, Serialize};
use tera::{self, Context};
use toml;

/// The structured syntax metadata.
///
/// We derive Serialize to serialize to a Tera config object
/// and Deserialize to deserialize from the metadata file.
#[derive(Serialize, Deserialize)]
pub struct SyntaxConfig {
    syntax_elements: Vec<SyntaxElement>,
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
pub struct SyntaxElement {
    name: String,
    token_text: String,
    regex: String,
    is_token: bool,
    is_node: bool,
}

impl<'de> Deserialize<'de> for SyntaxElement {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        #[derive(Deserialize)]
        #[serde(rename = "SyntaxElement")]
        struct Helper(String, String, String, bool, bool);
        // We implement deserialize by just delegating to a helper tuple struct type.
        Helper::deserialize(deserializer).map(|helper| SyntaxElement {
            name: helper.0,
            token_text: helper.1,
            regex: helper.2,
            is_token: helper.3,
            is_node: helper.4,
        })
    }
}

/// Project-relative path to the syntax metadata.
const SYNTAX_CONFIG: &str = "src/syntax.toml";
/// The manifest directory.
const MANIFEST: &str = env!("CARGO_MANIFEST_DIR");

pub fn syntax_elements_as_tera_context() -> Result<Context, Box<dyn Error>> {
    let syntax_config = format!("{}/{}", MANIFEST, SYNTAX_CONFIG);
    println!("cargo:rerun-if-changed={}", syntax_config);
    // Read in the context file.
    let config: SyntaxConfig = toml::from_str(&fs::read_to_string(syntax_config)?)?;
    // And convert it into the Tera-compatible form.
    Ok(Context::from_serialize(config)?)
}
