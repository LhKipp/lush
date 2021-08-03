use crate::{Event, Token};
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
            &u32::from(self.len),
        )
    }
}

impl Serialize for Event {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let data = match self {
            Event::Start {
                kind,
                forward_parent,
            } => match forward_parent {
                Some(idx) => {
                    format!("{} (parent: {})", kind.name(), idx)
                }
                None => kind.name().into(),
            },
            Event::Finish => "".into(),
            Event::Token(token) => {
                format!("{} (len: {})", token.kind.name(), u32::from(token.len))
            }
            Event::Error { msg } => msg.error.clone(),
        };
        serializer.serialize_newtype_variant(
            // The name of the type
            "Event",
            // TokenKind is `#[repr(u16)]`, so this cast is legal
            self.index(),
            // Using our added helper to get the name of the kind
            self.into(),
            // The data payload of the serialized newtype variant
            &data,
        )
    }
}
