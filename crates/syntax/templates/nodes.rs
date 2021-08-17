#[allow(unused_imports)]

use crate::{
    ast::{self, support, AstNodeChildren, AstElementChildren, AstNode, AstToken, AstElement},
    SyntaxKind::{self, *},
    SyntaxNode, SyntaxToken, SyntaxElement
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

{% for gen_elem in generic_elements -%}

pub enum {{ gen_elem.enum_name }} {
    {% for represented in gen_elem.represents -%}
    {{represented.name}}({{represented.struct_name}}),
    {% endfor -%}
}

impl {{ gen_elem.enum_name }} {
}

{% if gen_elem.impl_trait == "AstElement" -%}
impl AstElement for {{ gen_elem.enum_name }} {
    fn can_cast(kind: SyntaxKind) -> bool { 
        match kind{
            {{ gen_elem.represents | map(attribute="name") | join(sep=" | ") }} => true,
            _ => false,
        }
    }
    fn cast(syntax: SyntaxElement) -> Option<Self> {
        let res = match syntax.kind() {
            {% for represented in gen_elem.represents -%}
            {% if represented.is_token  -%}
            {{represented.name}} => {{gen_elem.enum_name}}::{{represented.name}}({{represented.struct_name}} { syntax: syntax.into_token().unwrap() }),
            {% elif represented.is_node  -%}
            {{represented.name}} => {{gen_elem.enum_name}}::{{represented.name}}({{represented.struct_name}} { syntax: syntax.into_node().unwrap() }),
            {% endif -%}
            {% endfor -%}
            _ => return None,
        };
        Some(res)
    }

    fn syntax(&self) -> SyntaxElement {
        match self {
            {% for represented in gen_elem.represents -%}
            {{gen_elem.enum_name}}::{{represented.name}}(it) => it.syntax.clone().into(),
            {% endfor -%}
        }
    }
}
{% elif gen_elem.impl_trait == "AstNode" -%}
impl AstNode for {{ gen_elem.enum_name }} {
    fn can_cast(kind: SyntaxKind) -> bool { 
        match kind{
            {{ gen_elem.represents | map(attribute="name") | join(sep=" | ") }} => true,
            _ => false,
        }
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        let res = match syntax.kind() {
            {% for represented in gen_elem.represents -%}
            {{represented.name}} => {{gen_elem.enum_name}}::{{represented.name}}({{represented.struct_name}} { syntax }),
            {% endfor -%}
            _ => return None,
        };
        Some(res)
    }

    fn syntax(&self) -> &SyntaxNode {
        match self {
            {% for represented in gen_elem.represents -%}
            {{gen_elem.enum_name}}::{{represented.name}}(it) => &it.syntax,
            {% endfor -%}
        }
    }
}
{% endif -%}
{% endfor -%}
