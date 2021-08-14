#[allow(unused_imports)]

use crate::{
    ast::{self, support, AstChildren, AstNode, AstToken},
    SyntaxKind::{self, *},
    SyntaxNode, SyntaxToken
};

{% for syn_elem in syntax_elements -%}
{% set syntax_kind_name = syn_elem.name %}

{% if syn_elem.is_token -%}
{% set token_name = syn_elem.name ~ "Token"  %}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct {{ token_name }} {
    pub(crate) syntax: SyntaxToken,
}

impl {{ token_name }} {
}
impl AstToken for {{ token_name }} {
    fn can_cast(kind: SyntaxKind) -> bool { kind == SyntaxKind::{{syntax_kind_name}} }
    fn cast(syntax: SyntaxToken) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxToken { &self.syntax }
}

{% else -%}
{% set node_name = syn_elem.name ~ "Node"  %}
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
{% endif -%}
{% endfor -%}
