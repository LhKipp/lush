{% set empty = [] -%}
{% set regex_kinds = empty
    | concat(with=literals)
    | concat(with=tokens) -%}
use logos::Logos;

#[allow(bad_style, missing_docs, unreachable_pub)]
#[derive(Logos, Copy, Clone, Debug, Eq, PartialEq, Hash)]
pub enum SyntaxKind {
    #[error]
    Error,
    {% for token in regex_kinds -%}
    {% if token.regex != "" -%}
    #[regex("{{token.regex}}")]
    {% endif -%}
    {{ token.name | camel_case }},
    {% endfor -%}
    {% for keyword in keywords -%}
    #[token("{{keyword}}")]
    {{ keyword | camel_case }},
    {% endfor -%}
    {% for punc in punctuation -%}
    #[token("{{punc.character}}")]
    {{ punc.name | camel_case }},
    {% endfor -%}
}

#[macro_export]
macro_rules! T {
    {% for token in regex_kinds -%}
    [{{ token.name | camel_case }}] => {$crate::SyntaxKind::{{token.name | camel_case}} };
    {% endfor -%}
    {% for keyword in keywords -%}
    [{{ keyword | quoted }}] => {$crate::SyntaxKind::{{keyword | camel_case}} };
    {% endfor -%}
    {% for punc in punctuation -%}
    [{{ punc.character | quoted }}] => {$crate::SyntaxKind::{{punc.name | camel_case }} };
    {% endfor -%}
}
