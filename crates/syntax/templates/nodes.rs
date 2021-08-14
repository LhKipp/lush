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
{% set syntax_kind_name = node | to_syntax_kind_name %}
{% set node_name = node | to_node_name %}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct {{ node_name }} {
    pub(crate) syntax: SyntaxNode,
}

impl {{ node_name }} {
}

impl AstNode for {{ node_name }} {
    fn can_cast(kind: SyntaxKind) -> bool { kind == SyntaxKind::{{syntax_kind_name}} }
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
