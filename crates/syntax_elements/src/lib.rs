use std::{error::Error, fs};

use serde::{Deserialize, Serialize};
use tera::{self, Context};
use toml;

/// The structured syntax metadata.
#[derive(Serialize, Deserialize, Clone)]
pub struct SyntaxConfig {
    syntax_elements: Vec<SyntaxElement>,
    generic_elements: Vec<GenericElement>,
}

impl SyntaxConfig {
    pub fn finish(&mut self) {
        for syn_elem in &mut self.syntax_elements {
            syn_elem.finish()
        }
        for generic_elem in &mut self.generic_elements {
            generic_elem.finish(&self.syntax_elements)
        }
    }
}

/// We derive Serialize so the Tera config object has named members,
/// but implement Deserialize manually to deserialize from an unnamed sequence.
///
/// Note that this means `de::from_str(ser::to_string(punctuation_config))`
/// does not work, as the two formats do not line up. This is only ok to do
/// here because these are the _only_ de/serialization tasks we care about.
#[derive(Serialize, Clone)]
pub struct SyntaxElement {
    name: String,
    struct_name: String,
    token_text: String,
    regex: String,
    is_token: bool,
    is_node: bool,
}

impl SyntaxElement {
    pub fn finish(&mut self) {
        if self.is_token {
            self.struct_name = self.name.clone() + "Token"
        } else if self.is_node {
            self.struct_name = self.name.clone() + "Node"
        }
    }
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
            struct_name: String::new(),
        })
    }
}

#[derive(Serialize, Clone)]
pub struct GenericElement {
    name: String,
    enum_name: String,
    represents_element_names: Vec<String>,
    represents: Vec<SyntaxElement>,
}

impl<'de> Deserialize<'de> for GenericElement {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        #[derive(Deserialize)]
        #[serde(rename = "GenericElement")]
        struct Helper(String, Vec<String>);
        // We implement deserialize by just delegating to a helper tuple struct type.
        Helper::deserialize(deserializer).map(|helper| GenericElement {
            name: helper.0,
            enum_name: String::new(),
            represents_element_names: helper.1,
            represents: Vec::new(),
        })
    }
}

impl GenericElement {
    pub fn finish(&mut self, syn_elems: &[SyntaxElement]) {
        self.enum_name = self.name.clone() + "Node";

        self.represents = self
            .represents_element_names
            .iter()
            .map(|name| {
                syn_elems
                    .iter()
                    .find(|syn_elem| syn_elem.name == *name)
                    .expect(&format!(
                        "Generic element links to {:?} which is not present",
                        name
                    ))
            })
            .cloned()
            .collect()
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
    let mut config: SyntaxConfig = toml::from_str(&fs::read_to_string(syntax_config)?)?;
    config.finish();
    // And convert it into the Tera-compatible form.
    Ok(Context::from_serialize(config)?)
}
