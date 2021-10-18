use derive_more::Display;
use enum_as_inner::EnumAsInner;
use lu_syntax::{
    ast::{FlagElement, FlagSignatureNode},
    AstToken,
};
#[derive(Clone, Debug, PartialEq, Eq, EnumAsInner, is_enum_variant, Display)]
pub enum FlagVariant {
    #[display(fmt = "--{}", _0)]
    LongFlag(String),
    #[display(fmt = "-{}", _0)]
    ShortFlag(char),
}

impl FlagVariant {
    pub fn convert<Iter>(flags: Iter) -> Vec<Self>
    where
        Iter: Iterator<Item = FlagElement>,
    {
        // TODO split short flags here
        flags.map(|flag_elem| Self::from_node(&flag_elem)).collect()
    }
    pub fn from_sign_node(flag_sign_node: &FlagSignatureNode) -> Self {
        if let Some(long_name) = flag_sign_node.long_name() {
            Self::LongFlag(long_name)
        } else if let Some(short_name) = flag_sign_node.short_name() {
            Self::ShortFlag(short_name)
        } else {
            unreachable!()
        }
    }
    pub fn from_node(flag_elem: &FlagElement) -> Self {
        match flag_elem {
            FlagElement::ShortFlag(n) => {
                assert!(n.text().len() == 2);
                FlagVariant::ShortFlag(n.text().chars().nth(1).unwrap())
            }
            FlagElement::LongFlag(n) => FlagVariant::LongFlag(n.text()[2..].to_string()),
        }
    }
}

pub struct FlagUsage {
    pub val: FlagVariant,
}
