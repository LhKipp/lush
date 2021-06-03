use crate::Token;
use ::serde::{Serialize, Serializer};

impl Serialize for Token {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_newtype_variant(
            // The name of the type
            "Token",
            // TokenKind is `#[repr(u16)]`, so this cast is legal
            self.kind as u16 as u32,
            // Using our added helper to get the name of the kind
            self.kind.name(),
            // The data payload of the serialized newtype variant
            &self.len,
        )
    }
}
