use logos::Logos;
use derive_more::Display;
use ::serde::{Deserialize, Serialize};

#[allow(bad_style, missing_docs, unreachable_pub)]
#[derive(Logos, Copy, Clone, Eq, PartialEq, Hash, Debug, Serialize, Deserialize, Display)]
#[repr(u16)]
pub enum SyntaxKind {
    {# Error and BareWord are a little special rest is normal #}
    {% for syn_elem in syntax_elements -%}
    {% if syn_elem.name == "Error" -%}
    #[error]
    {% elif syn_elem.name == "BareWord" -%}
    #[regex("{{syn_elem.regex}}", priority = 0)]
    {% elif syn_elem.regex and syn_elem.priority -%}
    #[regex("{{syn_elem.regex}}", priority = {{syn_elem.priority}})]
    {% elif syn_elem.token_text and syn_elem.priority -%}
    #[token("{{syn_elem.token_text}}", priority = {{syn_elem.priority}})]
    {% elif syn_elem.regex -%}
    #[regex("{{syn_elem.regex}}{{syn_elem.priority}}")]
    {% elif syn_elem.token_text -%}
    #[token("{{syn_elem.token_text}}")]
    {% endif -%}
    {{ syn_elem.name }},
    {% endfor -%}
    __LAST,
}

impl SyntaxKind{
    pub const fn name(self) -> &'static str {
        match self {
            {% for syn_elem in syntax_elements -%}
            SyntaxKind::{{ syn_elem.name }} => "{{ syn_elem.name }}",
            {% endfor -%}
            #[allow(unreachable_patterns)]
            _ => "", // For the future
        }
    }
}

#[macro_export]
macro_rules! T {
    {% for syn_elem in syntax_elements -%}
    {% if syn_elem.token_text != "" -%}
    [{{ syn_elem.token_text | quote_brackets }}] => {$crate::SyntaxKind::{{syn_elem.name }} };
    {% endif -%}
    {% endfor -%}
}
