#[allow(unused_imports)]
use crate::{
    Rule,
    ast::{self, support, AstNodeChildren, AstElementChildren, AstNode, AstToken, AstElement, HasRule, HasSyntaxKind},
    SyntaxKind::{self, *},
    SyntaxNode, SyntaxToken, SyntaxElement
};

{% for syn_elem in syntax_elements -%}
{% if syn_elem.is_token -%}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct {{ syn_elem.struct_name }} {
    pub(crate) syntax: SyntaxToken,
}
impl AstToken for {{ syn_elem.struct_name }} {
    fn can_cast(kind: SyntaxKind) -> bool { kind == SyntaxKind::{{syn_elem.name}} }
    fn cast(syntax: SyntaxToken) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxToken { &self.syntax }
}
impl HasSyntaxKind for {{ syn_elem.struct_name }}{
    fn get_syntax_kind(&self) -> SyntaxKind{
        self.syntax().kind()
    }
}

{% elif syn_elem.is_node -%}

pub struct {{ syn_elem.struct_name }} {
    pub(crate) syntax: SyntaxNode,
}
impl AstNode for {{ syn_elem.struct_name }} {
    fn can_cast(kind: SyntaxKind) -> bool { kind == SyntaxKind::{{syn_elem.name}} }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode { &self.syntax }
}
impl HasSyntaxKind for {{ syn_elem.struct_name }}{
    fn get_syntax_kind(&self) -> SyntaxKind{
        self.syntax().kind()
    }
}

{% elif syn_elem.is_generic -%}

pub enum {{ syn_elem.struct_name }} {
    {% for represented in syn_elem.represents -%}
    {{represented.name}}({{represented.struct_name}}),
    {% endfor -%}
}

impl {{ syn_elem.struct_name }} {
}

{% if syn_elem.impl_trait == "AstElement" -%}
impl AstElement for {{ syn_elem.struct_name }} {
    fn can_cast(kind: SyntaxKind) -> bool { 
        {% for represented in syn_elem.represents -%}
        {% if represented.is_generic %}
        {{represented.struct_name}}::can_cast(kind) ||
        {% endif %}
        {% endfor %}
        match kind{
            {{ syn_elem.represents | map(attribute="name") | join(sep=" | ") }} => true,
            _ => false,
        }
    }
    fn cast(syntax: SyntaxElement) -> Option<Self> {
        {% for represented in syn_elem.represents -%}
        {% if represented.is_generic  -%}
            if let Some(casted) = {{represented.struct_name}}::cast(syntax.clone()){
                return Some(Self::{{represented.name}}(casted));
            }
        {% endif %}
        {% endfor %}
        let res = match syntax.kind() {
            {% for represented in syn_elem.represents -%}
            {% if represented.is_token  -%}
            {{represented.name}} => {{syn_elem.struct_name}}::{{represented.name}}({{represented.struct_name}} { syntax: syntax.into_token().unwrap() }),
            {% elif represented.is_node  -%}
            {{represented.name}} => {{syn_elem.struct_name}}::{{represented.name}}({{represented.struct_name}} { syntax: syntax.into_node().unwrap() }),
            {% endif -%}
            {% endfor -%}
            _ => return None,
        };
        Some(res)
    }

    fn syntax(&self) -> SyntaxElement {
        match self {
            {% for represented in syn_elem.represents -%}
            {% if represented.is_generic %}
            {{syn_elem.struct_name}}::{{represented.name}}(it) => it.syntax().clone().into(),
            {% else %}
            {{syn_elem.struct_name}}::{{represented.name}}(it) => it.syntax.clone().into(),
            {% endif %}
            {% endfor -%}
        }
    }
}
{% elif syn_elem.impl_trait == "AstNode" -%}
impl AstNode for {{ syn_elem.struct_name }} {
    fn can_cast(kind: SyntaxKind) -> bool { 
        {% for represented in syn_elem.represents -%}
        {% if represented.is_generic %}
        {{represented.struct_name}}::can_cast(kind) ||
        {% endif %}
        {% endfor %}
        match kind{
            {{ syn_elem.represents | map(attribute="name") | join(sep=" | ") }} => true,
            _ => false,
        }
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        let res = match syntax.kind() {
            {% for represented in syn_elem.represents -%}
            {{represented.name}} => {{syn_elem.struct_name}}::{{represented.name}}({{represented.struct_name}} { syntax }),
            {% endfor -%}
            _ => return None,
        };
        Some(res)
    }

    fn syntax(&self) -> &SyntaxNode {
        match self {
            {% for represented in syn_elem.represents -%}
            {{syn_elem.struct_name}}::{{represented.name}}(it) => &it.syntax,
            {% endfor -%}
        }
    }
}
{% endif -%}

impl HasSyntaxKind for {{ syn_elem.struct_name }}{
    fn get_syntax_kind(&self) -> SyntaxKind{
        match self {
            {% for represented in syn_elem.represents -%}
            {{syn_elem.struct_name}}::{{represented.name}}(it) => it.get_syntax_kind(),
            {% endfor -%}
        }
    }
}

{% endif -%}

{% if syn_elem.has_rule -%}
{% set rule_name = syn_elem.name ~ "Rule" %}
use lu_parser::grammar::{{rule_name}};
impl HasRule for {{syn_elem.struct_name}}{
    fn get_belonging_rule() -> Box<dyn Rule>{
        Box::new({{rule_name}}{})
    }
}
{% endif -%}


{% endfor -%}
