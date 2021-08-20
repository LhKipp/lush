use std::{error::Error, fs};

use ron::from_str;
use serde::{Deserialize, Serialize};
use tera::{self, Context};

#[derive(Deserialize, Serialize, Debug, Clone)]
struct Config {
    pub syntax_elements: Vec<SyntaxElement>,
}

/// We derive Serialize so the Tera config object has named members,
/// but implement Deserialize manually to deserialize from an unnamed sequence.
///
/// Note that this means `de::from_str(ser::to_string(punctuation_config))`
/// does not work, as the two formats do not line up. This is only ok to do
/// here because these are the _only_ de/serialization tasks we care about.
#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct SyntaxElement {
    name: String,
    #[serde(default)]
    struct_name: String,
    #[serde(default)]
    token_text: String,
    #[serde(default)]
    regex: String,
    #[serde(default)]
    is_token: bool,
    #[serde(default)]
    is_node: bool,
    #[serde(default)]
    is_generic: bool,
    #[serde(default)]
    has_rule: bool,

    #[serde(default)]
    represents_element_names: Vec<String>,
    #[serde(default)]
    represents: Vec<SyntaxElement>,
    #[serde(default)]
    impl_trait: String,
}

impl SyntaxElement {
    pub fn before_linking(&mut self) {
        if self.is_token {
            self.struct_name = self.name.clone() + "Token";
        } else if self.is_node || self.is_generic {
            self.struct_name = self.name.clone() + "Node";
        }
    }
    pub fn link(&mut self, elems: &[SyntaxElement]) {
        for represents in &self.represents_element_names {
            let elem = elems
                .iter()
                .find(|e| e.name == *represents)
                .expect(&format!("No such elem: {}", represents));
            if elem.is_finished() {
                self.represents.push(elem.clone());
            }
        }

        let found = self.represents.clone();
        self.represents_element_names
            // retain if not yet found
            .retain(|name| !found.iter().any(|e| e.name == *name));
    }

    pub fn after_linkage(&mut self) {
        if self.is_generic {
            self.impl_trait = if self.represents.iter().all(|e| e.is_node) {
                "AstNode".to_string()
            } else {
                "AstElement".to_string()
            }
        }
    }

    pub fn is_finished(&self) -> bool {
        self.represents_element_names.is_empty()
    }
}

/// Project-relative path to the syntax metadata.
const SYNTAX_CONFIG: &str = "src/syntax.ron";
/// The manifest directory.
const MANIFEST: &str = env!("CARGO_MANIFEST_DIR");

pub fn syntax_elements_as_tera_context() -> Result<Context, Box<dyn Error>> {
    let syntax_config = format!("{}/{}", MANIFEST, SYNTAX_CONFIG);
    println!("cargo:rerun-if-changed={}", syntax_config);
    let mut config: Config = from_str(&fs::read_to_string(syntax_config)?)?;

    config
        .syntax_elements
        .iter_mut()
        .for_each(SyntaxElement::before_linking);
    // Deepest level of referencing is yet 2
    for _ in 0..2 {
        let clnd = config.syntax_elements.clone();
        config
            .syntax_elements
            .iter_mut()
            .for_each(|elem| elem.link(&clnd))
    }

    config
        .syntax_elements
        .iter_mut()
        .for_each(SyntaxElement::after_linkage);

    assert!(config
        .syntax_elements
        .iter()
        .all(SyntaxElement::is_finished));

    Ok(Context::from_serialize(config)?)
}
