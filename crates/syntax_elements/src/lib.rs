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
pub struct PunctuationConfig {
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
pub struct RegexToken {
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
