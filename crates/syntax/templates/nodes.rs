#[allow(unused_imports)]

{% set empty = [] -%}
{# We are only interested in names here #}
{% set literals = literals | map(attribute="name") -%}
{% set tokens = tokens | map(attribute="name") -%}
{% set node_kinds = empty
    | concat(with=literals)
    | concat(with=tokens) -%}

use crate::{
    ast::{self, support, AstChildren, AstNode},
    SyntaxKind::{self, *},
    SyntaxNode
};

{% for node in node_kinds -%}
{% set node_camel = node | camel_case %}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct {{ node_camel }} {
    pub(crate) syntax: SyntaxNode,
}

impl {{ node_camel }} {
}

impl AstNode for {{ node_camel }} {
    fn can_cast(kind: SyntaxKind) -> bool { kind == SyntaxKind::{{node_camel}} }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode { &self.syntax }
}
{% endfor -%}
