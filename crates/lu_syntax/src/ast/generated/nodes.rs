#![allow(unused_imports)]
use std::fmt::Display;
use serde::Serialize;
use crate::TextRange;
use enum_as_inner::EnumAsInner;
use crate::{
    Rule,
    ast::{self, support, AstNodeChildren, AstElementChildren, AstNode, AstToken, AstElement, HasRule, HasSyntaxKind, HasTextRange},
    SyntaxKind::{self, *},
    SyntaxNode, SyntaxToken, SyntaxElement
};

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize)]
pub struct ImpureKeywordToken {
    pub(crate) syntax: SyntaxToken,
}
impl AstToken for ImpureKeywordToken {
    fn can_cast(kind: SyntaxKind) -> bool { kind == SyntaxKind::ImpureKeyword }
    fn cast(syntax: SyntaxToken) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxToken { &self.syntax }
}
impl HasSyntaxKind for ImpureKeywordToken{
    fn get_syntax_kind(&self) -> SyntaxKind{
        self.syntax().kind()
    }
}
impl HasTextRange for ImpureKeywordToken{
    fn get_text_range(&self) -> TextRange{
        self.syntax().text_range()
    }
}

impl Display for ImpureKeywordToken {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.text())
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize)]
pub struct StrctKeywordToken {
    pub(crate) syntax: SyntaxToken,
}
impl AstToken for StrctKeywordToken {
    fn can_cast(kind: SyntaxKind) -> bool { kind == SyntaxKind::StrctKeyword }
    fn cast(syntax: SyntaxToken) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxToken { &self.syntax }
}
impl HasSyntaxKind for StrctKeywordToken{
    fn get_syntax_kind(&self) -> SyntaxKind{
        self.syntax().kind()
    }
}
impl HasTextRange for StrctKeywordToken{
    fn get_text_range(&self) -> TextRange{
        self.syntax().text_range()
    }
}

impl Display for StrctKeywordToken {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.text())
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize)]
pub struct ReqKeywordToken {
    pub(crate) syntax: SyntaxToken,
}
impl AstToken for ReqKeywordToken {
    fn can_cast(kind: SyntaxKind) -> bool { kind == SyntaxKind::ReqKeyword }
    fn cast(syntax: SyntaxToken) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxToken { &self.syntax }
}
impl HasSyntaxKind for ReqKeywordToken{
    fn get_syntax_kind(&self) -> SyntaxKind{
        self.syntax().kind()
    }
}
impl HasTextRange for ReqKeywordToken{
    fn get_text_range(&self) -> TextRange{
        self.syntax().text_range()
    }
}

impl Display for ReqKeywordToken {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.text())
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize)]
pub struct UseKeywordToken {
    pub(crate) syntax: SyntaxToken,
}
impl AstToken for UseKeywordToken {
    fn can_cast(kind: SyntaxKind) -> bool { kind == SyntaxKind::UseKeyword }
    fn cast(syntax: SyntaxToken) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxToken { &self.syntax }
}
impl HasSyntaxKind for UseKeywordToken{
    fn get_syntax_kind(&self) -> SyntaxKind{
        self.syntax().kind()
    }
}
impl HasTextRange for UseKeywordToken{
    fn get_text_range(&self) -> TextRange{
        self.syntax().text_range()
    }
}

impl Display for UseKeywordToken {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.text())
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize)]
pub struct LetKeywordToken {
    pub(crate) syntax: SyntaxToken,
}
impl AstToken for LetKeywordToken {
    fn can_cast(kind: SyntaxKind) -> bool { kind == SyntaxKind::LetKeyword }
    fn cast(syntax: SyntaxToken) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxToken { &self.syntax }
}
impl HasSyntaxKind for LetKeywordToken{
    fn get_syntax_kind(&self) -> SyntaxKind{
        self.syntax().kind()
    }
}
impl HasTextRange for LetKeywordToken{
    fn get_text_range(&self) -> TextRange{
        self.syntax().text_range()
    }
}

impl Display for LetKeywordToken {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.text())
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize)]
pub struct FnKeywordToken {
    pub(crate) syntax: SyntaxToken,
}
impl AstToken for FnKeywordToken {
    fn can_cast(kind: SyntaxKind) -> bool { kind == SyntaxKind::FnKeyword }
    fn cast(syntax: SyntaxToken) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxToken { &self.syntax }
}
impl HasSyntaxKind for FnKeywordToken{
    fn get_syntax_kind(&self) -> SyntaxKind{
        self.syntax().kind()
    }
}
impl HasTextRange for FnKeywordToken{
    fn get_text_range(&self) -> TextRange{
        self.syntax().text_range()
    }
}

impl Display for FnKeywordToken {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.text())
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize)]
pub struct ForKeywordToken {
    pub(crate) syntax: SyntaxToken,
}
impl AstToken for ForKeywordToken {
    fn can_cast(kind: SyntaxKind) -> bool { kind == SyntaxKind::ForKeyword }
    fn cast(syntax: SyntaxToken) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxToken { &self.syntax }
}
impl HasSyntaxKind for ForKeywordToken{
    fn get_syntax_kind(&self) -> SyntaxKind{
        self.syntax().kind()
    }
}
impl HasTextRange for ForKeywordToken{
    fn get_text_range(&self) -> TextRange{
        self.syntax().text_range()
    }
}

impl Display for ForKeywordToken {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.text())
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize)]
pub struct ElifKeywordToken {
    pub(crate) syntax: SyntaxToken,
}
impl AstToken for ElifKeywordToken {
    fn can_cast(kind: SyntaxKind) -> bool { kind == SyntaxKind::ElifKeyword }
    fn cast(syntax: SyntaxToken) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxToken { &self.syntax }
}
impl HasSyntaxKind for ElifKeywordToken{
    fn get_syntax_kind(&self) -> SyntaxKind{
        self.syntax().kind()
    }
}
impl HasTextRange for ElifKeywordToken{
    fn get_text_range(&self) -> TextRange{
        self.syntax().text_range()
    }
}

impl Display for ElifKeywordToken {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.text())
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize)]
pub struct ElseKeywordToken {
    pub(crate) syntax: SyntaxToken,
}
impl AstToken for ElseKeywordToken {
    fn can_cast(kind: SyntaxKind) -> bool { kind == SyntaxKind::ElseKeyword }
    fn cast(syntax: SyntaxToken) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxToken { &self.syntax }
}
impl HasSyntaxKind for ElseKeywordToken{
    fn get_syntax_kind(&self) -> SyntaxKind{
        self.syntax().kind()
    }
}
impl HasTextRange for ElseKeywordToken{
    fn get_text_range(&self) -> TextRange{
        self.syntax().text_range()
    }
}

impl Display for ElseKeywordToken {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.text())
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize)]
pub struct IfKeywordToken {
    pub(crate) syntax: SyntaxToken,
}
impl AstToken for IfKeywordToken {
    fn can_cast(kind: SyntaxKind) -> bool { kind == SyntaxKind::IfKeyword }
    fn cast(syntax: SyntaxToken) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxToken { &self.syntax }
}
impl HasSyntaxKind for IfKeywordToken{
    fn get_syntax_kind(&self) -> SyntaxKind{
        self.syntax().kind()
    }
}
impl HasTextRange for IfKeywordToken{
    fn get_text_range(&self) -> TextRange{
        self.syntax().text_range()
    }
}

impl Display for IfKeywordToken {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.text())
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize)]
pub struct WhileKeywordToken {
    pub(crate) syntax: SyntaxToken,
}
impl AstToken for WhileKeywordToken {
    fn can_cast(kind: SyntaxKind) -> bool { kind == SyntaxKind::WhileKeyword }
    fn cast(syntax: SyntaxToken) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxToken { &self.syntax }
}
impl HasSyntaxKind for WhileKeywordToken{
    fn get_syntax_kind(&self) -> SyntaxKind{
        self.syntax().kind()
    }
}
impl HasTextRange for WhileKeywordToken{
    fn get_text_range(&self) -> TextRange{
        self.syntax().text_range()
    }
}

impl Display for WhileKeywordToken {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.text())
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize)]
pub struct EndKeywordToken {
    pub(crate) syntax: SyntaxToken,
}
impl AstToken for EndKeywordToken {
    fn can_cast(kind: SyntaxKind) -> bool { kind == SyntaxKind::EndKeyword }
    fn cast(syntax: SyntaxToken) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxToken { &self.syntax }
}
impl HasSyntaxKind for EndKeywordToken{
    fn get_syntax_kind(&self) -> SyntaxKind{
        self.syntax().kind()
    }
}
impl HasTextRange for EndKeywordToken{
    fn get_text_range(&self) -> TextRange{
        self.syntax().text_range()
    }
}

impl Display for EndKeywordToken {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.text())
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize)]
pub struct BeginKeywordToken {
    pub(crate) syntax: SyntaxToken,
}
impl AstToken for BeginKeywordToken {
    fn can_cast(kind: SyntaxKind) -> bool { kind == SyntaxKind::BeginKeyword }
    fn cast(syntax: SyntaxToken) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxToken { &self.syntax }
}
impl HasSyntaxKind for BeginKeywordToken{
    fn get_syntax_kind(&self) -> SyntaxKind{
        self.syntax().kind()
    }
}
impl HasTextRange for BeginKeywordToken{
    fn get_text_range(&self) -> TextRange{
        self.syntax().text_range()
    }
}

impl Display for BeginKeywordToken {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.text())
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize)]
pub struct InKeywordToken {
    pub(crate) syntax: SyntaxToken,
}
impl AstToken for InKeywordToken {
    fn can_cast(kind: SyntaxKind) -> bool { kind == SyntaxKind::InKeyword }
    fn cast(syntax: SyntaxToken) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxToken { &self.syntax }
}
impl HasSyntaxKind for InKeywordToken{
    fn get_syntax_kind(&self) -> SyntaxKind{
        self.syntax().kind()
    }
}
impl HasTextRange for InKeywordToken{
    fn get_text_range(&self) -> TextRange{
        self.syntax().text_range()
    }
}

impl Display for InKeywordToken {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.text())
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize)]
pub struct AnyKeywordToken {
    pub(crate) syntax: SyntaxToken,
}
impl AstToken for AnyKeywordToken {
    fn can_cast(kind: SyntaxKind) -> bool { kind == SyntaxKind::AnyKeyword }
    fn cast(syntax: SyntaxToken) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxToken { &self.syntax }
}
impl HasSyntaxKind for AnyKeywordToken{
    fn get_syntax_kind(&self) -> SyntaxKind{
        self.syntax().kind()
    }
}
impl HasTextRange for AnyKeywordToken{
    fn get_text_range(&self) -> TextRange{
        self.syntax().text_range()
    }
}

impl Display for AnyKeywordToken {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.text())
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize)]
pub struct NilKeywordToken {
    pub(crate) syntax: SyntaxToken,
}
impl AstToken for NilKeywordToken {
    fn can_cast(kind: SyntaxKind) -> bool { kind == SyntaxKind::NilKeyword }
    fn cast(syntax: SyntaxToken) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxToken { &self.syntax }
}
impl HasSyntaxKind for NilKeywordToken{
    fn get_syntax_kind(&self) -> SyntaxKind{
        self.syntax().kind()
    }
}
impl HasTextRange for NilKeywordToken{
    fn get_text_range(&self) -> TextRange{
        self.syntax().text_range()
    }
}

impl Display for NilKeywordToken {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.text())
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize)]
pub struct BoolKeywordToken {
    pub(crate) syntax: SyntaxToken,
}
impl AstToken for BoolKeywordToken {
    fn can_cast(kind: SyntaxKind) -> bool { kind == SyntaxKind::BoolKeyword }
    fn cast(syntax: SyntaxToken) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxToken { &self.syntax }
}
impl HasSyntaxKind for BoolKeywordToken{
    fn get_syntax_kind(&self) -> SyntaxKind{
        self.syntax().kind()
    }
}
impl HasTextRange for BoolKeywordToken{
    fn get_text_range(&self) -> TextRange{
        self.syntax().text_range()
    }
}

impl Display for BoolKeywordToken {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.text())
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize)]
pub struct TrueKeywordToken {
    pub(crate) syntax: SyntaxToken,
}
impl AstToken for TrueKeywordToken {
    fn can_cast(kind: SyntaxKind) -> bool { kind == SyntaxKind::TrueKeyword }
    fn cast(syntax: SyntaxToken) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxToken { &self.syntax }
}
impl HasSyntaxKind for TrueKeywordToken{
    fn get_syntax_kind(&self) -> SyntaxKind{
        self.syntax().kind()
    }
}
impl HasTextRange for TrueKeywordToken{
    fn get_text_range(&self) -> TextRange{
        self.syntax().text_range()
    }
}

impl Display for TrueKeywordToken {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.text())
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize)]
pub struct FalseKeywordToken {
    pub(crate) syntax: SyntaxToken,
}
impl AstToken for FalseKeywordToken {
    fn can_cast(kind: SyntaxKind) -> bool { kind == SyntaxKind::FalseKeyword }
    fn cast(syntax: SyntaxToken) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxToken { &self.syntax }
}
impl HasSyntaxKind for FalseKeywordToken{
    fn get_syntax_kind(&self) -> SyntaxKind{
        self.syntax().kind()
    }
}
impl HasTextRange for FalseKeywordToken{
    fn get_text_range(&self) -> TextRange{
        self.syntax().text_range()
    }
}

impl Display for FalseKeywordToken {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.text())
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize)]
pub struct NumberKeywordToken {
    pub(crate) syntax: SyntaxToken,
}
impl AstToken for NumberKeywordToken {
    fn can_cast(kind: SyntaxKind) -> bool { kind == SyntaxKind::NumberKeyword }
    fn cast(syntax: SyntaxToken) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxToken { &self.syntax }
}
impl HasSyntaxKind for NumberKeywordToken{
    fn get_syntax_kind(&self) -> SyntaxKind{
        self.syntax().kind()
    }
}
impl HasTextRange for NumberKeywordToken{
    fn get_text_range(&self) -> TextRange{
        self.syntax().text_range()
    }
}

impl Display for NumberKeywordToken {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.text())
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize)]
pub struct StringKeywordToken {
    pub(crate) syntax: SyntaxToken,
}
impl AstToken for StringKeywordToken {
    fn can_cast(kind: SyntaxKind) -> bool { kind == SyntaxKind::StringKeyword }
    fn cast(syntax: SyntaxToken) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxToken { &self.syntax }
}
impl HasSyntaxKind for StringKeywordToken{
    fn get_syntax_kind(&self) -> SyntaxKind{
        self.syntax().kind()
    }
}
impl HasTextRange for StringKeywordToken{
    fn get_text_range(&self) -> TextRange{
        self.syntax().text_range()
    }
}

impl Display for StringKeywordToken {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.text())
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize)]
pub struct RetKeywordToken {
    pub(crate) syntax: SyntaxToken,
}
impl AstToken for RetKeywordToken {
    fn can_cast(kind: SyntaxKind) -> bool { kind == SyntaxKind::RetKeyword }
    fn cast(syntax: SyntaxToken) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxToken { &self.syntax }
}
impl HasSyntaxKind for RetKeywordToken{
    fn get_syntax_kind(&self) -> SyntaxKind{
        self.syntax().kind()
    }
}
impl HasTextRange for RetKeywordToken{
    fn get_text_range(&self) -> TextRange{
        self.syntax().text_range()
    }
}

impl Display for RetKeywordToken {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.text())
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize)]
pub struct GenericTypeToken {
    pub(crate) syntax: SyntaxToken,
}
impl AstToken for GenericTypeToken {
    fn can_cast(kind: SyntaxKind) -> bool { kind == SyntaxKind::GenericType }
    fn cast(syntax: SyntaxToken) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxToken { &self.syntax }
}
impl HasSyntaxKind for GenericTypeToken{
    fn get_syntax_kind(&self) -> SyntaxKind{
        self.syntax().kind()
    }
}
impl HasTextRange for GenericTypeToken{
    fn get_text_range(&self) -> TextRange{
        self.syntax().text_range()
    }
}

impl Display for GenericTypeToken {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.text())
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize)]
pub struct ArrayTypeNode {
    pub(crate) syntax: SyntaxNode,
}
impl AstNode for ArrayTypeNode {
    fn can_cast(kind: SyntaxKind) -> bool { kind == SyntaxKind::ArrayType }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode { &self.syntax }
}
impl HasSyntaxKind for ArrayTypeNode{
    fn get_syntax_kind(&self) -> SyntaxKind{
        self.syntax().kind()
    }
}
impl HasTextRange for ArrayTypeNode{
    fn get_text_range(&self) -> TextRange{
        self.syntax().text_range()
    }
}


use lu_parser::grammar::ArrayTypeRule;
impl HasRule for ArrayTypeNode{
    fn get_belonging_rule() -> Box<dyn Rule>{
        Box::new(ArrayTypeRule{})
    }
}
impl Display for ArrayTypeNode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.text())
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize)]
pub struct FnTypeNode {
    pub(crate) syntax: SyntaxNode,
}
impl AstNode for FnTypeNode {
    fn can_cast(kind: SyntaxKind) -> bool { kind == SyntaxKind::FnType }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode { &self.syntax }
}
impl HasSyntaxKind for FnTypeNode{
    fn get_syntax_kind(&self) -> SyntaxKind{
        self.syntax().kind()
    }
}
impl HasTextRange for FnTypeNode{
    fn get_text_range(&self) -> TextRange{
        self.syntax().text_range()
    }
}


use lu_parser::grammar::FnTypeRule;
impl HasRule for FnTypeNode{
    fn get_belonging_rule() -> Box<dyn Rule>{
        Box::new(FnTypeRule{})
    }
}
impl Display for FnTypeNode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.text())
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize)]
pub struct OptModifierToken {
    pub(crate) syntax: SyntaxToken,
}
impl AstToken for OptModifierToken {
    fn can_cast(kind: SyntaxKind) -> bool { kind == SyntaxKind::OptModifier }
    fn cast(syntax: SyntaxToken) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxToken { &self.syntax }
}
impl HasSyntaxKind for OptModifierToken{
    fn get_syntax_kind(&self) -> SyntaxKind{
        self.syntax().kind()
    }
}
impl HasTextRange for OptModifierToken{
    fn get_text_range(&self) -> TextRange{
        self.syntax().text_range()
    }
}

impl Display for OptModifierToken {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.text())
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize)]
pub struct LeftParenthesisToken {
    pub(crate) syntax: SyntaxToken,
}
impl AstToken for LeftParenthesisToken {
    fn can_cast(kind: SyntaxKind) -> bool { kind == SyntaxKind::LeftParenthesis }
    fn cast(syntax: SyntaxToken) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxToken { &self.syntax }
}
impl HasSyntaxKind for LeftParenthesisToken{
    fn get_syntax_kind(&self) -> SyntaxKind{
        self.syntax().kind()
    }
}
impl HasTextRange for LeftParenthesisToken{
    fn get_text_range(&self) -> TextRange{
        self.syntax().text_range()
    }
}

impl Display for LeftParenthesisToken {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.text())
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize)]
pub struct RightParenthesisToken {
    pub(crate) syntax: SyntaxToken,
}
impl AstToken for RightParenthesisToken {
    fn can_cast(kind: SyntaxKind) -> bool { kind == SyntaxKind::RightParenthesis }
    fn cast(syntax: SyntaxToken) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxToken { &self.syntax }
}
impl HasSyntaxKind for RightParenthesisToken{
    fn get_syntax_kind(&self) -> SyntaxKind{
        self.syntax().kind()
    }
}
impl HasTextRange for RightParenthesisToken{
    fn get_text_range(&self) -> TextRange{
        self.syntax().text_range()
    }
}

impl Display for RightParenthesisToken {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.text())
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize)]
pub struct LeftCurlyBracketsToken {
    pub(crate) syntax: SyntaxToken,
}
impl AstToken for LeftCurlyBracketsToken {
    fn can_cast(kind: SyntaxKind) -> bool { kind == SyntaxKind::LeftCurlyBrackets }
    fn cast(syntax: SyntaxToken) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxToken { &self.syntax }
}
impl HasSyntaxKind for LeftCurlyBracketsToken{
    fn get_syntax_kind(&self) -> SyntaxKind{
        self.syntax().kind()
    }
}
impl HasTextRange for LeftCurlyBracketsToken{
    fn get_text_range(&self) -> TextRange{
        self.syntax().text_range()
    }
}

impl Display for LeftCurlyBracketsToken {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.text())
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize)]
pub struct RightCurlyBracketsToken {
    pub(crate) syntax: SyntaxToken,
}
impl AstToken for RightCurlyBracketsToken {
    fn can_cast(kind: SyntaxKind) -> bool { kind == SyntaxKind::RightCurlyBrackets }
    fn cast(syntax: SyntaxToken) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxToken { &self.syntax }
}
impl HasSyntaxKind for RightCurlyBracketsToken{
    fn get_syntax_kind(&self) -> SyntaxKind{
        self.syntax().kind()
    }
}
impl HasTextRange for RightCurlyBracketsToken{
    fn get_text_range(&self) -> TextRange{
        self.syntax().text_range()
    }
}

impl Display for RightCurlyBracketsToken {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.text())
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize)]
pub struct LeftRectangularBracketsToken {
    pub(crate) syntax: SyntaxToken,
}
impl AstToken for LeftRectangularBracketsToken {
    fn can_cast(kind: SyntaxKind) -> bool { kind == SyntaxKind::LeftRectangularBrackets }
    fn cast(syntax: SyntaxToken) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxToken { &self.syntax }
}
impl HasSyntaxKind for LeftRectangularBracketsToken{
    fn get_syntax_kind(&self) -> SyntaxKind{
        self.syntax().kind()
    }
}
impl HasTextRange for LeftRectangularBracketsToken{
    fn get_text_range(&self) -> TextRange{
        self.syntax().text_range()
    }
}

impl Display for LeftRectangularBracketsToken {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.text())
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize)]
pub struct RightRectangularBracketsToken {
    pub(crate) syntax: SyntaxToken,
}
impl AstToken for RightRectangularBracketsToken {
    fn can_cast(kind: SyntaxKind) -> bool { kind == SyntaxKind::RightRectangularBrackets }
    fn cast(syntax: SyntaxToken) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxToken { &self.syntax }
}
impl HasSyntaxKind for RightRectangularBracketsToken{
    fn get_syntax_kind(&self) -> SyntaxKind{
        self.syntax().kind()
    }
}
impl HasTextRange for RightRectangularBracketsToken{
    fn get_text_range(&self) -> TextRange{
        self.syntax().text_range()
    }
}

impl Display for RightRectangularBracketsToken {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.text())
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize)]
pub struct PlusSignToken {
    pub(crate) syntax: SyntaxToken,
}
impl AstToken for PlusSignToken {
    fn can_cast(kind: SyntaxKind) -> bool { kind == SyntaxKind::PlusSign }
    fn cast(syntax: SyntaxToken) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxToken { &self.syntax }
}
impl HasSyntaxKind for PlusSignToken{
    fn get_syntax_kind(&self) -> SyntaxKind{
        self.syntax().kind()
    }
}
impl HasTextRange for PlusSignToken{
    fn get_text_range(&self) -> TextRange{
        self.syntax().text_range()
    }
}

impl Display for PlusSignToken {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.text())
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize)]
pub struct MinusSignToken {
    pub(crate) syntax: SyntaxToken,
}
impl AstToken for MinusSignToken {
    fn can_cast(kind: SyntaxKind) -> bool { kind == SyntaxKind::MinusSign }
    fn cast(syntax: SyntaxToken) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxToken { &self.syntax }
}
impl HasSyntaxKind for MinusSignToken{
    fn get_syntax_kind(&self) -> SyntaxKind{
        self.syntax().kind()
    }
}
impl HasTextRange for MinusSignToken{
    fn get_text_range(&self) -> TextRange{
        self.syntax().text_range()
    }
}

impl Display for MinusSignToken {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.text())
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize)]
pub struct MultSignToken {
    pub(crate) syntax: SyntaxToken,
}
impl AstToken for MultSignToken {
    fn can_cast(kind: SyntaxKind) -> bool { kind == SyntaxKind::MultSign }
    fn cast(syntax: SyntaxToken) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxToken { &self.syntax }
}
impl HasSyntaxKind for MultSignToken{
    fn get_syntax_kind(&self) -> SyntaxKind{
        self.syntax().kind()
    }
}
impl HasTextRange for MultSignToken{
    fn get_text_range(&self) -> TextRange{
        self.syntax().text_range()
    }
}

impl Display for MultSignToken {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.text())
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize)]
pub struct FilePathSepToken {
    pub(crate) syntax: SyntaxToken,
}
impl AstToken for FilePathSepToken {
    fn can_cast(kind: SyntaxKind) -> bool { kind == SyntaxKind::FilePathSep }
    fn cast(syntax: SyntaxToken) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxToken { &self.syntax }
}
impl HasSyntaxKind for FilePathSepToken{
    fn get_syntax_kind(&self) -> SyntaxKind{
        self.syntax().kind()
    }
}
impl HasTextRange for FilePathSepToken{
    fn get_text_range(&self) -> TextRange{
        self.syntax().text_range()
    }
}

impl Display for FilePathSepToken {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.text())
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize)]
pub struct DivSignToken {
    pub(crate) syntax: SyntaxToken,
}
impl AstToken for DivSignToken {
    fn can_cast(kind: SyntaxKind) -> bool { kind == SyntaxKind::DivSign }
    fn cast(syntax: SyntaxToken) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxToken { &self.syntax }
}
impl HasSyntaxKind for DivSignToken{
    fn get_syntax_kind(&self) -> SyntaxKind{
        self.syntax().kind()
    }
}
impl HasTextRange for DivSignToken{
    fn get_text_range(&self) -> TextRange{
        self.syntax().text_range()
    }
}

impl Display for DivSignToken {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.text())
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize)]
pub struct LessThanSignToken {
    pub(crate) syntax: SyntaxToken,
}
impl AstToken for LessThanSignToken {
    fn can_cast(kind: SyntaxKind) -> bool { kind == SyntaxKind::LessThanSign }
    fn cast(syntax: SyntaxToken) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxToken { &self.syntax }
}
impl HasSyntaxKind for LessThanSignToken{
    fn get_syntax_kind(&self) -> SyntaxKind{
        self.syntax().kind()
    }
}
impl HasTextRange for LessThanSignToken{
    fn get_text_range(&self) -> TextRange{
        self.syntax().text_range()
    }
}

impl Display for LessThanSignToken {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.text())
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize)]
pub struct LessOrEqualSignToken {
    pub(crate) syntax: SyntaxToken,
}
impl AstToken for LessOrEqualSignToken {
    fn can_cast(kind: SyntaxKind) -> bool { kind == SyntaxKind::LessOrEqualSign }
    fn cast(syntax: SyntaxToken) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxToken { &self.syntax }
}
impl HasSyntaxKind for LessOrEqualSignToken{
    fn get_syntax_kind(&self) -> SyntaxKind{
        self.syntax().kind()
    }
}
impl HasTextRange for LessOrEqualSignToken{
    fn get_text_range(&self) -> TextRange{
        self.syntax().text_range()
    }
}

impl Display for LessOrEqualSignToken {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.text())
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize)]
pub struct EqualitySignToken {
    pub(crate) syntax: SyntaxToken,
}
impl AstToken for EqualitySignToken {
    fn can_cast(kind: SyntaxKind) -> bool { kind == SyntaxKind::EqualitySign }
    fn cast(syntax: SyntaxToken) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxToken { &self.syntax }
}
impl HasSyntaxKind for EqualitySignToken{
    fn get_syntax_kind(&self) -> SyntaxKind{
        self.syntax().kind()
    }
}
impl HasTextRange for EqualitySignToken{
    fn get_text_range(&self) -> TextRange{
        self.syntax().text_range()
    }
}

impl Display for EqualitySignToken {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.text())
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize)]
pub struct InequalitySignToken {
    pub(crate) syntax: SyntaxToken,
}
impl AstToken for InequalitySignToken {
    fn can_cast(kind: SyntaxKind) -> bool { kind == SyntaxKind::InequalitySign }
    fn cast(syntax: SyntaxToken) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxToken { &self.syntax }
}
impl HasSyntaxKind for InequalitySignToken{
    fn get_syntax_kind(&self) -> SyntaxKind{
        self.syntax().kind()
    }
}
impl HasTextRange for InequalitySignToken{
    fn get_text_range(&self) -> TextRange{
        self.syntax().text_range()
    }
}

impl Display for InequalitySignToken {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.text())
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize)]
pub struct BiggerThanSignToken {
    pub(crate) syntax: SyntaxToken,
}
impl AstToken for BiggerThanSignToken {
    fn can_cast(kind: SyntaxKind) -> bool { kind == SyntaxKind::BiggerThanSign }
    fn cast(syntax: SyntaxToken) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxToken { &self.syntax }
}
impl HasSyntaxKind for BiggerThanSignToken{
    fn get_syntax_kind(&self) -> SyntaxKind{
        self.syntax().kind()
    }
}
impl HasTextRange for BiggerThanSignToken{
    fn get_text_range(&self) -> TextRange{
        self.syntax().text_range()
    }
}

impl Display for BiggerThanSignToken {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.text())
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize)]
pub struct BiggerOrEqualSignToken {
    pub(crate) syntax: SyntaxToken,
}
impl AstToken for BiggerOrEqualSignToken {
    fn can_cast(kind: SyntaxKind) -> bool { kind == SyntaxKind::BiggerOrEqualSign }
    fn cast(syntax: SyntaxToken) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxToken { &self.syntax }
}
impl HasSyntaxKind for BiggerOrEqualSignToken{
    fn get_syntax_kind(&self) -> SyntaxKind{
        self.syntax().kind()
    }
}
impl HasTextRange for BiggerOrEqualSignToken{
    fn get_text_range(&self) -> TextRange{
        self.syntax().text_range()
    }
}

impl Display for BiggerOrEqualSignToken {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.text())
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize)]
pub struct RightStreamToken {
    pub(crate) syntax: SyntaxToken,
}
impl AstToken for RightStreamToken {
    fn can_cast(kind: SyntaxKind) -> bool { kind == SyntaxKind::RightStream }
    fn cast(syntax: SyntaxToken) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxToken { &self.syntax }
}
impl HasSyntaxKind for RightStreamToken{
    fn get_syntax_kind(&self) -> SyntaxKind{
        self.syntax().kind()
    }
}
impl HasTextRange for RightStreamToken{
    fn get_text_range(&self) -> TextRange{
        self.syntax().text_range()
    }
}

impl Display for RightStreamToken {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.text())
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize)]
pub struct DivAssignSignToken {
    pub(crate) syntax: SyntaxToken,
}
impl AstToken for DivAssignSignToken {
    fn can_cast(kind: SyntaxKind) -> bool { kind == SyntaxKind::DivAssignSign }
    fn cast(syntax: SyntaxToken) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxToken { &self.syntax }
}
impl HasSyntaxKind for DivAssignSignToken{
    fn get_syntax_kind(&self) -> SyntaxKind{
        self.syntax().kind()
    }
}
impl HasTextRange for DivAssignSignToken{
    fn get_text_range(&self) -> TextRange{
        self.syntax().text_range()
    }
}

impl Display for DivAssignSignToken {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.text())
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize)]
pub struct MulAssignSignToken {
    pub(crate) syntax: SyntaxToken,
}
impl AstToken for MulAssignSignToken {
    fn can_cast(kind: SyntaxKind) -> bool { kind == SyntaxKind::MulAssignSign }
    fn cast(syntax: SyntaxToken) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxToken { &self.syntax }
}
impl HasSyntaxKind for MulAssignSignToken{
    fn get_syntax_kind(&self) -> SyntaxKind{
        self.syntax().kind()
    }
}
impl HasTextRange for MulAssignSignToken{
    fn get_text_range(&self) -> TextRange{
        self.syntax().text_range()
    }
}

impl Display for MulAssignSignToken {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.text())
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize)]
pub struct AddAssignSignToken {
    pub(crate) syntax: SyntaxToken,
}
impl AstToken for AddAssignSignToken {
    fn can_cast(kind: SyntaxKind) -> bool { kind == SyntaxKind::AddAssignSign }
    fn cast(syntax: SyntaxToken) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxToken { &self.syntax }
}
impl HasSyntaxKind for AddAssignSignToken{
    fn get_syntax_kind(&self) -> SyntaxKind{
        self.syntax().kind()
    }
}
impl HasTextRange for AddAssignSignToken{
    fn get_text_range(&self) -> TextRange{
        self.syntax().text_range()
    }
}

impl Display for AddAssignSignToken {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.text())
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize)]
pub struct MinAssignSignToken {
    pub(crate) syntax: SyntaxToken,
}
impl AstToken for MinAssignSignToken {
    fn can_cast(kind: SyntaxKind) -> bool { kind == SyntaxKind::MinAssignSign }
    fn cast(syntax: SyntaxToken) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxToken { &self.syntax }
}
impl HasSyntaxKind for MinAssignSignToken{
    fn get_syntax_kind(&self) -> SyntaxKind{
        self.syntax().kind()
    }
}
impl HasTextRange for MinAssignSignToken{
    fn get_text_range(&self) -> TextRange{
        self.syntax().text_range()
    }
}

impl Display for MinAssignSignToken {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.text())
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize)]
pub struct AssignSignToken {
    pub(crate) syntax: SyntaxToken,
}
impl AstToken for AssignSignToken {
    fn can_cast(kind: SyntaxKind) -> bool { kind == SyntaxKind::AssignSign }
    fn cast(syntax: SyntaxToken) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxToken { &self.syntax }
}
impl HasSyntaxKind for AssignSignToken{
    fn get_syntax_kind(&self) -> SyntaxKind{
        self.syntax().kind()
    }
}
impl HasTextRange for AssignSignToken{
    fn get_text_range(&self) -> TextRange{
        self.syntax().text_range()
    }
}

impl Display for AssignSignToken {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.text())
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize)]
pub struct PipeToken {
    pub(crate) syntax: SyntaxToken,
}
impl AstToken for PipeToken {
    fn can_cast(kind: SyntaxKind) -> bool { kind == SyntaxKind::Pipe }
    fn cast(syntax: SyntaxToken) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxToken { &self.syntax }
}
impl HasSyntaxKind for PipeToken{
    fn get_syntax_kind(&self) -> SyntaxKind{
        self.syntax().kind()
    }
}
impl HasTextRange for PipeToken{
    fn get_text_range(&self) -> TextRange{
        self.syntax().text_range()
    }
}

impl Display for PipeToken {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.text())
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize)]
pub struct DollarToken {
    pub(crate) syntax: SyntaxToken,
}
impl AstToken for DollarToken {
    fn can_cast(kind: SyntaxKind) -> bool { kind == SyntaxKind::Dollar }
    fn cast(syntax: SyntaxToken) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxToken { &self.syntax }
}
impl HasSyntaxKind for DollarToken{
    fn get_syntax_kind(&self) -> SyntaxKind{
        self.syntax().kind()
    }
}
impl HasTextRange for DollarToken{
    fn get_text_range(&self) -> TextRange{
        self.syntax().text_range()
    }
}

impl Display for DollarToken {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.text())
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize)]
pub struct QuestionMarkToken {
    pub(crate) syntax: SyntaxToken,
}
impl AstToken for QuestionMarkToken {
    fn can_cast(kind: SyntaxKind) -> bool { kind == SyntaxKind::QuestionMark }
    fn cast(syntax: SyntaxToken) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxToken { &self.syntax }
}
impl HasSyntaxKind for QuestionMarkToken{
    fn get_syntax_kind(&self) -> SyntaxKind{
        self.syntax().kind()
    }
}
impl HasTextRange for QuestionMarkToken{
    fn get_text_range(&self) -> TextRange{
        self.syntax().text_range()
    }
}

impl Display for QuestionMarkToken {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.text())
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize)]
pub struct PointToken {
    pub(crate) syntax: SyntaxToken,
}
impl AstToken for PointToken {
    fn can_cast(kind: SyntaxKind) -> bool { kind == SyntaxKind::Point }
    fn cast(syntax: SyntaxToken) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxToken { &self.syntax }
}
impl HasSyntaxKind for PointToken{
    fn get_syntax_kind(&self) -> SyntaxKind{
        self.syntax().kind()
    }
}
impl HasTextRange for PointToken{
    fn get_text_range(&self) -> TextRange{
        self.syntax().text_range()
    }
}

impl Display for PointToken {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.text())
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize)]
pub struct DoublePointToken {
    pub(crate) syntax: SyntaxToken,
}
impl AstToken for DoublePointToken {
    fn can_cast(kind: SyntaxKind) -> bool { kind == SyntaxKind::DoublePoint }
    fn cast(syntax: SyntaxToken) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxToken { &self.syntax }
}
impl HasSyntaxKind for DoublePointToken{
    fn get_syntax_kind(&self) -> SyntaxKind{
        self.syntax().kind()
    }
}
impl HasTextRange for DoublePointToken{
    fn get_text_range(&self) -> TextRange{
        self.syntax().text_range()
    }
}

impl Display for DoublePointToken {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.text())
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize)]
pub struct DoubleQuoteToken {
    pub(crate) syntax: SyntaxToken,
}
impl AstToken for DoubleQuoteToken {
    fn can_cast(kind: SyntaxKind) -> bool { kind == SyntaxKind::DoubleQuote }
    fn cast(syntax: SyntaxToken) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxToken { &self.syntax }
}
impl HasSyntaxKind for DoubleQuoteToken{
    fn get_syntax_kind(&self) -> SyntaxKind{
        self.syntax().kind()
    }
}
impl HasTextRange for DoubleQuoteToken{
    fn get_text_range(&self) -> TextRange{
        self.syntax().text_range()
    }
}

impl Display for DoubleQuoteToken {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.text())
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize)]
pub struct SingleQuoteToken {
    pub(crate) syntax: SyntaxToken,
}
impl AstToken for SingleQuoteToken {
    fn can_cast(kind: SyntaxKind) -> bool { kind == SyntaxKind::SingleQuote }
    fn cast(syntax: SyntaxToken) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxToken { &self.syntax }
}
impl HasSyntaxKind for SingleQuoteToken{
    fn get_syntax_kind(&self) -> SyntaxKind{
        self.syntax().kind()
    }
}
impl HasTextRange for SingleQuoteToken{
    fn get_text_range(&self) -> TextRange{
        self.syntax().text_range()
    }
}

impl Display for SingleQuoteToken {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.text())
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize)]
pub struct ErrorToken {
    pub(crate) syntax: SyntaxToken,
}
impl AstToken for ErrorToken {
    fn can_cast(kind: SyntaxKind) -> bool { kind == SyntaxKind::Error }
    fn cast(syntax: SyntaxToken) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxToken { &self.syntax }
}
impl HasSyntaxKind for ErrorToken{
    fn get_syntax_kind(&self) -> SyntaxKind{
        self.syntax().kind()
    }
}
impl HasTextRange for ErrorToken{
    fn get_text_range(&self) -> TextRange{
        self.syntax().text_range()
    }
}

impl Display for ErrorToken {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.text())
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize)]
pub struct ParserInternalToken {
    pub(crate) syntax: SyntaxToken,
}
impl AstToken for ParserInternalToken {
    fn can_cast(kind: SyntaxKind) -> bool { kind == SyntaxKind::ParserInternal }
    fn cast(syntax: SyntaxToken) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxToken { &self.syntax }
}
impl HasSyntaxKind for ParserInternalToken{
    fn get_syntax_kind(&self) -> SyntaxKind{
        self.syntax().kind()
    }
}
impl HasTextRange for ParserInternalToken{
    fn get_text_range(&self) -> TextRange{
        self.syntax().text_range()
    }
}

impl Display for ParserInternalToken {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.text())
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize)]
pub struct EofNode {
    pub(crate) syntax: SyntaxNode,
}
impl AstNode for EofNode {
    fn can_cast(kind: SyntaxKind) -> bool { kind == SyntaxKind::Eof }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode { &self.syntax }
}
impl HasSyntaxKind for EofNode{
    fn get_syntax_kind(&self) -> SyntaxKind{
        self.syntax().kind()
    }
}
impl HasTextRange for EofNode{
    fn get_text_range(&self) -> TextRange{
        self.syntax().text_range()
    }
}

impl Display for EofNode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.text())
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize)]
pub struct TombstoneNode {
    pub(crate) syntax: SyntaxNode,
}
impl AstNode for TombstoneNode {
    fn can_cast(kind: SyntaxKind) -> bool { kind == SyntaxKind::Tombstone }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode { &self.syntax }
}
impl HasSyntaxKind for TombstoneNode{
    fn get_syntax_kind(&self) -> SyntaxKind{
        self.syntax().kind()
    }
}
impl HasTextRange for TombstoneNode{
    fn get_text_range(&self) -> TextRange{
        self.syntax().text_range()
    }
}

impl Display for TombstoneNode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.text())
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize)]
pub struct WhitespaceToken {
    pub(crate) syntax: SyntaxToken,
}
impl AstToken for WhitespaceToken {
    fn can_cast(kind: SyntaxKind) -> bool { kind == SyntaxKind::Whitespace }
    fn cast(syntax: SyntaxToken) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxToken { &self.syntax }
}
impl HasSyntaxKind for WhitespaceToken{
    fn get_syntax_kind(&self) -> SyntaxKind{
        self.syntax().kind()
    }
}
impl HasTextRange for WhitespaceToken{
    fn get_text_range(&self) -> TextRange{
        self.syntax().text_range()
    }
}

impl Display for WhitespaceToken {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.text())
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize)]
pub struct CommentToken {
    pub(crate) syntax: SyntaxToken,
}
impl AstToken for CommentToken {
    fn can_cast(kind: SyntaxKind) -> bool { kind == SyntaxKind::Comment }
    fn cast(syntax: SyntaxToken) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxToken { &self.syntax }
}
impl HasSyntaxKind for CommentToken{
    fn get_syntax_kind(&self) -> SyntaxKind{
        self.syntax().kind()
    }
}
impl HasTextRange for CommentToken{
    fn get_text_range(&self) -> TextRange{
        self.syntax().text_range()
    }
}

impl Display for CommentToken {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.text())
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize)]
pub struct NewlineToken {
    pub(crate) syntax: SyntaxToken,
}
impl AstToken for NewlineToken {
    fn can_cast(kind: SyntaxKind) -> bool { kind == SyntaxKind::Newline }
    fn cast(syntax: SyntaxToken) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxToken { &self.syntax }
}
impl HasSyntaxKind for NewlineToken{
    fn get_syntax_kind(&self) -> SyntaxKind{
        self.syntax().kind()
    }
}
impl HasTextRange for NewlineToken{
    fn get_text_range(&self) -> TextRange{
        self.syntax().text_range()
    }
}

impl Display for NewlineToken {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.text())
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize)]
pub struct BareWordToken {
    pub(crate) syntax: SyntaxToken,
}
impl AstToken for BareWordToken {
    fn can_cast(kind: SyntaxKind) -> bool { kind == SyntaxKind::BareWord }
    fn cast(syntax: SyntaxToken) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxToken { &self.syntax }
}
impl HasSyntaxKind for BareWordToken{
    fn get_syntax_kind(&self) -> SyntaxKind{
        self.syntax().kind()
    }
}
impl HasTextRange for BareWordToken{
    fn get_text_range(&self) -> TextRange{
        self.syntax().text_range()
    }
}

impl Display for BareWordToken {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.text())
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize)]
pub struct StringContentToken {
    pub(crate) syntax: SyntaxToken,
}
impl AstToken for StringContentToken {
    fn can_cast(kind: SyntaxKind) -> bool { kind == SyntaxKind::StringContent }
    fn cast(syntax: SyntaxToken) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxToken { &self.syntax }
}
impl HasSyntaxKind for StringContentToken{
    fn get_syntax_kind(&self) -> SyntaxKind{
        self.syntax().kind()
    }
}
impl HasTextRange for StringContentToken{
    fn get_text_range(&self) -> TextRange{
        self.syntax().text_range()
    }
}

impl Display for StringContentToken {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.text())
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize)]
pub struct VarDeclNameToken {
    pub(crate) syntax: SyntaxToken,
}
impl AstToken for VarDeclNameToken {
    fn can_cast(kind: SyntaxKind) -> bool { kind == SyntaxKind::VarDeclName }
    fn cast(syntax: SyntaxToken) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxToken { &self.syntax }
}
impl HasSyntaxKind for VarDeclNameToken{
    fn get_syntax_kind(&self) -> SyntaxKind{
        self.syntax().kind()
    }
}
impl HasTextRange for VarDeclNameToken{
    fn get_text_range(&self) -> TextRange{
        self.syntax().text_range()
    }
}

impl Display for VarDeclNameToken {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.text())
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize)]
pub struct FnDeclNameToken {
    pub(crate) syntax: SyntaxToken,
}
impl AstToken for FnDeclNameToken {
    fn can_cast(kind: SyntaxKind) -> bool { kind == SyntaxKind::FnDeclName }
    fn cast(syntax: SyntaxToken) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxToken { &self.syntax }
}
impl HasSyntaxKind for FnDeclNameToken{
    fn get_syntax_kind(&self) -> SyntaxKind{
        self.syntax().kind()
    }
}
impl HasTextRange for FnDeclNameToken{
    fn get_text_range(&self) -> TextRange{
        self.syntax().text_range()
    }
}

impl Display for FnDeclNameToken {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.text())
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize)]
pub struct ArgNameToken {
    pub(crate) syntax: SyntaxToken,
}
impl AstToken for ArgNameToken {
    fn can_cast(kind: SyntaxKind) -> bool { kind == SyntaxKind::ArgName }
    fn cast(syntax: SyntaxToken) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxToken { &self.syntax }
}
impl HasSyntaxKind for ArgNameToken{
    fn get_syntax_kind(&self) -> SyntaxKind{
        self.syntax().kind()
    }
}
impl HasTextRange for ArgNameToken{
    fn get_text_range(&self) -> TextRange{
        self.syntax().text_range()
    }
}

impl Display for ArgNameToken {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.text())
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize)]
pub struct VarArgNameToken {
    pub(crate) syntax: SyntaxToken,
}
impl AstToken for VarArgNameToken {
    fn can_cast(kind: SyntaxKind) -> bool { kind == SyntaxKind::VarArgName }
    fn cast(syntax: SyntaxToken) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxToken { &self.syntax }
}
impl HasSyntaxKind for VarArgNameToken{
    fn get_syntax_kind(&self) -> SyntaxKind{
        self.syntax().kind()
    }
}
impl HasTextRange for VarArgNameToken{
    fn get_text_range(&self) -> TextRange{
        self.syntax().text_range()
    }
}

impl Display for VarArgNameToken {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.text())
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize)]
pub struct LongFlagToken {
    pub(crate) syntax: SyntaxToken,
}
impl AstToken for LongFlagToken {
    fn can_cast(kind: SyntaxKind) -> bool { kind == SyntaxKind::LongFlag }
    fn cast(syntax: SyntaxToken) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxToken { &self.syntax }
}
impl HasSyntaxKind for LongFlagToken{
    fn get_syntax_kind(&self) -> SyntaxKind{
        self.syntax().kind()
    }
}
impl HasTextRange for LongFlagToken{
    fn get_text_range(&self) -> TextRange{
        self.syntax().text_range()
    }
}

impl Display for LongFlagToken {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.text())
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize)]
pub struct ShortFlagToken {
    pub(crate) syntax: SyntaxToken,
}
impl AstToken for ShortFlagToken {
    fn can_cast(kind: SyntaxKind) -> bool { kind == SyntaxKind::ShortFlag }
    fn cast(syntax: SyntaxToken) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxToken { &self.syntax }
}
impl HasSyntaxKind for ShortFlagToken{
    fn get_syntax_kind(&self) -> SyntaxKind{
        self.syntax().kind()
    }
}
impl HasTextRange for ShortFlagToken{
    fn get_text_range(&self) -> TextRange{
        self.syntax().text_range()
    }
}

impl Display for ShortFlagToken {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.text())
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize)]
pub struct NumberToken {
    pub(crate) syntax: SyntaxToken,
}
impl AstToken for NumberToken {
    fn can_cast(kind: SyntaxKind) -> bool { kind == SyntaxKind::Number }
    fn cast(syntax: SyntaxToken) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxToken { &self.syntax }
}
impl HasSyntaxKind for NumberToken{
    fn get_syntax_kind(&self) -> SyntaxKind{
        self.syntax().kind()
    }
}
impl HasTextRange for NumberToken{
    fn get_text_range(&self) -> TextRange{
        self.syntax().text_range()
    }
}

impl Display for NumberToken {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.text())
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize)]
pub struct StrctNameToken {
    pub(crate) syntax: SyntaxToken,
}
impl AstToken for StrctNameToken {
    fn can_cast(kind: SyntaxKind) -> bool { kind == SyntaxKind::StrctName }
    fn cast(syntax: SyntaxToken) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxToken { &self.syntax }
}
impl HasSyntaxKind for StrctNameToken{
    fn get_syntax_kind(&self) -> SyntaxKind{
        self.syntax().kind()
    }
}
impl HasTextRange for StrctNameToken{
    fn get_text_range(&self) -> TextRange{
        self.syntax().text_range()
    }
}

impl Display for StrctNameToken {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.text())
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize)]
pub struct StrctFieldNameToken {
    pub(crate) syntax: SyntaxToken,
}
impl AstToken for StrctFieldNameToken {
    fn can_cast(kind: SyntaxKind) -> bool { kind == SyntaxKind::StrctFieldName }
    fn cast(syntax: SyntaxToken) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxToken { &self.syntax }
}
impl HasSyntaxKind for StrctFieldNameToken{
    fn get_syntax_kind(&self) -> SyntaxKind{
        self.syntax().kind()
    }
}
impl HasTextRange for StrctFieldNameToken{
    fn get_text_range(&self) -> TextRange{
        self.syntax().text_range()
    }
}

impl Display for StrctFieldNameToken {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.text())
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize)]
pub struct AbsFileNameNode {
    pub(crate) syntax: SyntaxNode,
}
impl AstNode for AbsFileNameNode {
    fn can_cast(kind: SyntaxKind) -> bool { kind == SyntaxKind::AbsFileName }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode { &self.syntax }
}
impl HasSyntaxKind for AbsFileNameNode{
    fn get_syntax_kind(&self) -> SyntaxKind{
        self.syntax().kind()
    }
}
impl HasTextRange for AbsFileNameNode{
    fn get_text_range(&self) -> TextRange{
        self.syntax().text_range()
    }
}

impl Display for AbsFileNameNode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.text())
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize)]
pub struct RelFileNameNode {
    pub(crate) syntax: SyntaxNode,
}
impl AstNode for RelFileNameNode {
    fn can_cast(kind: SyntaxKind) -> bool { kind == SyntaxKind::RelFileName }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode { &self.syntax }
}
impl HasSyntaxKind for RelFileNameNode{
    fn get_syntax_kind(&self) -> SyntaxKind{
        self.syntax().kind()
    }
}
impl HasTextRange for RelFileNameNode{
    fn get_text_range(&self) -> TextRange{
        self.syntax().text_range()
    }
}

impl Display for RelFileNameNode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.text())
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize)]
pub struct SourceFileNode {
    pub(crate) syntax: SyntaxNode,
}
impl AstNode for SourceFileNode {
    fn can_cast(kind: SyntaxKind) -> bool { kind == SyntaxKind::SourceFile }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode { &self.syntax }
}
impl HasSyntaxKind for SourceFileNode{
    fn get_syntax_kind(&self) -> SyntaxKind{
        self.syntax().kind()
    }
}
impl HasTextRange for SourceFileNode{
    fn get_text_range(&self) -> TextRange{
        self.syntax().text_range()
    }
}


use lu_parser::grammar::SourceFileRule;
impl HasRule for SourceFileNode{
    fn get_belonging_rule() -> Box<dyn Rule>{
        Box::new(SourceFileRule{})
    }
}
impl Display for SourceFileNode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.text())
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize)]
pub struct StrctStmtNode {
    pub(crate) syntax: SyntaxNode,
}
impl AstNode for StrctStmtNode {
    fn can_cast(kind: SyntaxKind) -> bool { kind == SyntaxKind::StrctStmt }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode { &self.syntax }
}
impl HasSyntaxKind for StrctStmtNode{
    fn get_syntax_kind(&self) -> SyntaxKind{
        self.syntax().kind()
    }
}
impl HasTextRange for StrctStmtNode{
    fn get_text_range(&self) -> TextRange{
        self.syntax().text_range()
    }
}

impl Display for StrctStmtNode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.text())
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize)]
pub struct RedirStmtNode {
    pub(crate) syntax: SyntaxNode,
}
impl AstNode for RedirStmtNode {
    fn can_cast(kind: SyntaxKind) -> bool { kind == SyntaxKind::RedirStmt }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode { &self.syntax }
}
impl HasSyntaxKind for RedirStmtNode{
    fn get_syntax_kind(&self) -> SyntaxKind{
        self.syntax().kind()
    }
}
impl HasTextRange for RedirStmtNode{
    fn get_text_range(&self) -> TextRange{
        self.syntax().text_range()
    }
}

impl Display for RedirStmtNode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.text())
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize)]
pub struct UseStmtNode {
    pub(crate) syntax: SyntaxNode,
}
impl AstNode for UseStmtNode {
    fn can_cast(kind: SyntaxKind) -> bool { kind == SyntaxKind::UseStmt }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode { &self.syntax }
}
impl HasSyntaxKind for UseStmtNode{
    fn get_syntax_kind(&self) -> SyntaxKind{
        self.syntax().kind()
    }
}
impl HasTextRange for UseStmtNode{
    fn get_text_range(&self) -> TextRange{
        self.syntax().text_range()
    }
}

impl Display for UseStmtNode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.text())
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize)]
pub struct PluginUseStmtNode {
    pub(crate) syntax: SyntaxNode,
}
impl AstNode for PluginUseStmtNode {
    fn can_cast(kind: SyntaxKind) -> bool { kind == SyntaxKind::PluginUseStmt }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode { &self.syntax }
}
impl HasSyntaxKind for PluginUseStmtNode{
    fn get_syntax_kind(&self) -> SyntaxKind{
        self.syntax().kind()
    }
}
impl HasTextRange for PluginUseStmtNode{
    fn get_text_range(&self) -> TextRange{
        self.syntax().text_range()
    }
}

impl Display for PluginUseStmtNode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.text())
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize)]
pub struct StrctFieldNode {
    pub(crate) syntax: SyntaxNode,
}
impl AstNode for StrctFieldNode {
    fn can_cast(kind: SyntaxKind) -> bool { kind == SyntaxKind::StrctField }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode { &self.syntax }
}
impl HasSyntaxKind for StrctFieldNode{
    fn get_syntax_kind(&self) -> SyntaxKind{
        self.syntax().kind()
    }
}
impl HasTextRange for StrctFieldNode{
    fn get_text_range(&self) -> TextRange{
        self.syntax().text_range()
    }
}

impl Display for StrctFieldNode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.text())
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize)]
pub struct StrctCtorExprNode {
    pub(crate) syntax: SyntaxNode,
}
impl AstNode for StrctCtorExprNode {
    fn can_cast(kind: SyntaxKind) -> bool { kind == SyntaxKind::StrctCtorExpr }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode { &self.syntax }
}
impl HasSyntaxKind for StrctCtorExprNode{
    fn get_syntax_kind(&self) -> SyntaxKind{
        self.syntax().kind()
    }
}
impl HasTextRange for StrctCtorExprNode{
    fn get_text_range(&self) -> TextRange{
        self.syntax().text_range()
    }
}

impl Display for StrctCtorExprNode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.text())
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize)]
pub struct StrctFieldCtorStmtNode {
    pub(crate) syntax: SyntaxNode,
}
impl AstNode for StrctFieldCtorStmtNode {
    fn can_cast(kind: SyntaxKind) -> bool { kind == SyntaxKind::StrctFieldCtorStmt }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode { &self.syntax }
}
impl HasSyntaxKind for StrctFieldCtorStmtNode{
    fn get_syntax_kind(&self) -> SyntaxKind{
        self.syntax().kind()
    }
}
impl HasTextRange for StrctFieldCtorStmtNode{
    fn get_text_range(&self) -> TextRange{
        self.syntax().text_range()
    }
}

impl Display for StrctFieldCtorStmtNode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.text())
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize)]
pub struct LetStmtNode {
    pub(crate) syntax: SyntaxNode,
}
impl AstNode for LetStmtNode {
    fn can_cast(kind: SyntaxKind) -> bool { kind == SyntaxKind::LetStmt }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode { &self.syntax }
}
impl HasSyntaxKind for LetStmtNode{
    fn get_syntax_kind(&self) -> SyntaxKind{
        self.syntax().kind()
    }
}
impl HasTextRange for LetStmtNode{
    fn get_text_range(&self) -> TextRange{
        self.syntax().text_range()
    }
}


use lu_parser::grammar::LetStmtRule;
impl HasRule for LetStmtNode{
    fn get_belonging_rule() -> Box<dyn Rule>{
        Box::new(LetStmtRule{})
    }
}
impl Display for LetStmtNode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.text())
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize)]
pub struct FnStmtNode {
    pub(crate) syntax: SyntaxNode,
}
impl AstNode for FnStmtNode {
    fn can_cast(kind: SyntaxKind) -> bool { kind == SyntaxKind::FnStmt }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode { &self.syntax }
}
impl HasSyntaxKind for FnStmtNode{
    fn get_syntax_kind(&self) -> SyntaxKind{
        self.syntax().kind()
    }
}
impl HasTextRange for FnStmtNode{
    fn get_text_range(&self) -> TextRange{
        self.syntax().text_range()
    }
}


use lu_parser::grammar::FnStmtRule;
impl HasRule for FnStmtNode{
    fn get_belonging_rule() -> Box<dyn Rule>{
        Box::new(FnStmtRule{})
    }
}
impl Display for FnStmtNode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.text())
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize)]
pub struct RetStmtNode {
    pub(crate) syntax: SyntaxNode,
}
impl AstNode for RetStmtNode {
    fn can_cast(kind: SyntaxKind) -> bool { kind == SyntaxKind::RetStmt }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode { &self.syntax }
}
impl HasSyntaxKind for RetStmtNode{
    fn get_syntax_kind(&self) -> SyntaxKind{
        self.syntax().kind()
    }
}
impl HasTextRange for RetStmtNode{
    fn get_text_range(&self) -> TextRange{
        self.syntax().text_range()
    }
}


use lu_parser::grammar::RetStmtRule;
impl HasRule for RetStmtNode{
    fn get_belonging_rule() -> Box<dyn Rule>{
        Box::new(RetStmtRule{})
    }
}
impl Display for RetStmtNode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.text())
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize)]
pub struct IfStmtNode {
    pub(crate) syntax: SyntaxNode,
}
impl AstNode for IfStmtNode {
    fn can_cast(kind: SyntaxKind) -> bool { kind == SyntaxKind::IfStmt }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode { &self.syntax }
}
impl HasSyntaxKind for IfStmtNode{
    fn get_syntax_kind(&self) -> SyntaxKind{
        self.syntax().kind()
    }
}
impl HasTextRange for IfStmtNode{
    fn get_text_range(&self) -> TextRange{
        self.syntax().text_range()
    }
}


use lu_parser::grammar::IfStmtRule;
impl HasRule for IfStmtNode{
    fn get_belonging_rule() -> Box<dyn Rule>{
        Box::new(IfStmtRule{})
    }
}
impl Display for IfStmtNode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.text())
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize)]
pub struct IfBlockNode {
    pub(crate) syntax: SyntaxNode,
}
impl AstNode for IfBlockNode {
    fn can_cast(kind: SyntaxKind) -> bool { kind == SyntaxKind::IfBlock }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode { &self.syntax }
}
impl HasSyntaxKind for IfBlockNode{
    fn get_syntax_kind(&self) -> SyntaxKind{
        self.syntax().kind()
    }
}
impl HasTextRange for IfBlockNode{
    fn get_text_range(&self) -> TextRange{
        self.syntax().text_range()
    }
}

impl Display for IfBlockNode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.text())
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize)]
pub struct ElifBlockNode {
    pub(crate) syntax: SyntaxNode,
}
impl AstNode for ElifBlockNode {
    fn can_cast(kind: SyntaxKind) -> bool { kind == SyntaxKind::ElifBlock }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode { &self.syntax }
}
impl HasSyntaxKind for ElifBlockNode{
    fn get_syntax_kind(&self) -> SyntaxKind{
        self.syntax().kind()
    }
}
impl HasTextRange for ElifBlockNode{
    fn get_text_range(&self) -> TextRange{
        self.syntax().text_range()
    }
}

impl Display for ElifBlockNode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.text())
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize)]
pub struct ElseBlockNode {
    pub(crate) syntax: SyntaxNode,
}
impl AstNode for ElseBlockNode {
    fn can_cast(kind: SyntaxKind) -> bool { kind == SyntaxKind::ElseBlock }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode { &self.syntax }
}
impl HasSyntaxKind for ElseBlockNode{
    fn get_syntax_kind(&self) -> SyntaxKind{
        self.syntax().kind()
    }
}
impl HasTextRange for ElseBlockNode{
    fn get_text_range(&self) -> TextRange{
        self.syntax().text_range()
    }
}

impl Display for ElseBlockNode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.text())
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize)]
pub struct ForStmtNode {
    pub(crate) syntax: SyntaxNode,
}
impl AstNode for ForStmtNode {
    fn can_cast(kind: SyntaxKind) -> bool { kind == SyntaxKind::ForStmt }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode { &self.syntax }
}
impl HasSyntaxKind for ForStmtNode{
    fn get_syntax_kind(&self) -> SyntaxKind{
        self.syntax().kind()
    }
}
impl HasTextRange for ForStmtNode{
    fn get_text_range(&self) -> TextRange{
        self.syntax().text_range()
    }
}


use lu_parser::grammar::ForStmtRule;
impl HasRule for ForStmtNode{
    fn get_belonging_rule() -> Box<dyn Rule>{
        Box::new(ForStmtRule{})
    }
}
impl Display for ForStmtNode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.text())
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize)]
pub struct CmdStmtNode {
    pub(crate) syntax: SyntaxNode,
}
impl AstNode for CmdStmtNode {
    fn can_cast(kind: SyntaxKind) -> bool { kind == SyntaxKind::CmdStmt }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode { &self.syntax }
}
impl HasSyntaxKind for CmdStmtNode{
    fn get_syntax_kind(&self) -> SyntaxKind{
        self.syntax().kind()
    }
}
impl HasTextRange for CmdStmtNode{
    fn get_text_range(&self) -> TextRange{
        self.syntax().text_range()
    }
}


use lu_parser::grammar::CmdStmtRule;
impl HasRule for CmdStmtNode{
    fn get_belonging_rule() -> Box<dyn Rule>{
        Box::new(CmdStmtRule{})
    }
}
impl Display for CmdStmtNode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.text())
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize)]
pub struct PipedCmdsStmtNode {
    pub(crate) syntax: SyntaxNode,
}
impl AstNode for PipedCmdsStmtNode {
    fn can_cast(kind: SyntaxKind) -> bool { kind == SyntaxKind::PipedCmdsStmt }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode { &self.syntax }
}
impl HasSyntaxKind for PipedCmdsStmtNode{
    fn get_syntax_kind(&self) -> SyntaxKind{
        self.syntax().kind()
    }
}
impl HasTextRange for PipedCmdsStmtNode{
    fn get_text_range(&self) -> TextRange{
        self.syntax().text_range()
    }
}


use lu_parser::grammar::PipedCmdsStmtRule;
impl HasRule for PipedCmdsStmtNode{
    fn get_belonging_rule() -> Box<dyn Rule>{
        Box::new(PipedCmdsStmtRule{})
    }
}
impl Display for PipedCmdsStmtNode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.text())
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize)]
pub struct BlockStmtNode {
    pub(crate) syntax: SyntaxNode,
}
impl AstNode for BlockStmtNode {
    fn can_cast(kind: SyntaxKind) -> bool { kind == SyntaxKind::BlockStmt }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode { &self.syntax }
}
impl HasSyntaxKind for BlockStmtNode{
    fn get_syntax_kind(&self) -> SyntaxKind{
        self.syntax().kind()
    }
}
impl HasTextRange for BlockStmtNode{
    fn get_text_range(&self) -> TextRange{
        self.syntax().text_range()
    }
}

impl Display for BlockStmtNode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.text())
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize)]
pub struct SignatureNode {
    pub(crate) syntax: SyntaxNode,
}
impl AstNode for SignatureNode {
    fn can_cast(kind: SyntaxKind) -> bool { kind == SyntaxKind::Signature }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode { &self.syntax }
}
impl HasSyntaxKind for SignatureNode{
    fn get_syntax_kind(&self) -> SyntaxKind{
        self.syntax().kind()
    }
}
impl HasTextRange for SignatureNode{
    fn get_text_range(&self) -> TextRange{
        self.syntax().text_range()
    }
}


use lu_parser::grammar::SignatureRule;
impl HasRule for SignatureNode{
    fn get_belonging_rule() -> Box<dyn Rule>{
        Box::new(SignatureRule{})
    }
}
impl Display for SignatureNode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.text())
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize)]
pub struct FlagSignatureNode {
    pub(crate) syntax: SyntaxNode,
}
impl AstNode for FlagSignatureNode {
    fn can_cast(kind: SyntaxKind) -> bool { kind == SyntaxKind::FlagSignature }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode { &self.syntax }
}
impl HasSyntaxKind for FlagSignatureNode{
    fn get_syntax_kind(&self) -> SyntaxKind{
        self.syntax().kind()
    }
}
impl HasTextRange for FlagSignatureNode{
    fn get_text_range(&self) -> TextRange{
        self.syntax().text_range()
    }
}

impl Display for FlagSignatureNode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.text())
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize)]
pub struct ArgSignatureNode {
    pub(crate) syntax: SyntaxNode,
}
impl AstNode for ArgSignatureNode {
    fn can_cast(kind: SyntaxKind) -> bool { kind == SyntaxKind::ArgSignature }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode { &self.syntax }
}
impl HasSyntaxKind for ArgSignatureNode{
    fn get_syntax_kind(&self) -> SyntaxKind{
        self.syntax().kind()
    }
}
impl HasTextRange for ArgSignatureNode{
    fn get_text_range(&self) -> TextRange{
        self.syntax().text_range()
    }
}

impl Display for ArgSignatureNode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.text())
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize)]
pub struct LuTypeNode {
    pub(crate) syntax: SyntaxNode,
}
impl AstNode for LuTypeNode {
    fn can_cast(kind: SyntaxKind) -> bool { kind == SyntaxKind::LuType }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode { &self.syntax }
}
impl HasSyntaxKind for LuTypeNode{
    fn get_syntax_kind(&self) -> SyntaxKind{
        self.syntax().kind()
    }
}
impl HasTextRange for LuTypeNode{
    fn get_text_range(&self) -> TextRange{
        self.syntax().text_range()
    }
}


use lu_parser::grammar::LuTypeRule;
impl HasRule for LuTypeNode{
    fn get_belonging_rule() -> Box<dyn Rule>{
        Box::new(LuTypeRule{})
    }
}
impl Display for LuTypeNode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.text())
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize)]
pub struct MathExprNode {
    pub(crate) syntax: SyntaxNode,
}
impl AstNode for MathExprNode {
    fn can_cast(kind: SyntaxKind) -> bool { kind == SyntaxKind::MathExpr }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode { &self.syntax }
}
impl HasSyntaxKind for MathExprNode{
    fn get_syntax_kind(&self) -> SyntaxKind{
        self.syntax().kind()
    }
}
impl HasTextRange for MathExprNode{
    fn get_text_range(&self) -> TextRange{
        self.syntax().text_range()
    }
}

impl Display for MathExprNode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.text())
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize)]
pub struct ParentherizedExprNode {
    pub(crate) syntax: SyntaxNode,
}
impl AstNode for ParentherizedExprNode {
    fn can_cast(kind: SyntaxKind) -> bool { kind == SyntaxKind::ParentherizedExpr }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode { &self.syntax }
}
impl HasSyntaxKind for ParentherizedExprNode{
    fn get_syntax_kind(&self) -> SyntaxKind{
        self.syntax().kind()
    }
}
impl HasTextRange for ParentherizedExprNode{
    fn get_text_range(&self) -> TextRange{
        self.syntax().text_range()
    }
}

impl Display for ParentherizedExprNode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.text())
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize)]
pub struct StringExprNode {
    pub(crate) syntax: SyntaxNode,
}
impl AstNode for StringExprNode {
    fn can_cast(kind: SyntaxKind) -> bool { kind == SyntaxKind::StringExpr }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode { &self.syntax }
}
impl HasSyntaxKind for StringExprNode{
    fn get_syntax_kind(&self) -> SyntaxKind{
        self.syntax().kind()
    }
}
impl HasTextRange for StringExprNode{
    fn get_text_range(&self) -> TextRange{
        self.syntax().text_range()
    }
}


use lu_parser::grammar::StringExprRule;
impl HasRule for StringExprNode{
    fn get_belonging_rule() -> Box<dyn Rule>{
        Box::new(StringExprRule{})
    }
}
impl Display for StringExprNode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.text())
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize)]
pub struct NumberExprNode {
    pub(crate) syntax: SyntaxNode,
}
impl AstNode for NumberExprNode {
    fn can_cast(kind: SyntaxKind) -> bool { kind == SyntaxKind::NumberExpr }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode { &self.syntax }
}
impl HasSyntaxKind for NumberExprNode{
    fn get_syntax_kind(&self) -> SyntaxKind{
        self.syntax().kind()
    }
}
impl HasTextRange for NumberExprNode{
    fn get_text_range(&self) -> TextRange{
        self.syntax().text_range()
    }
}


use lu_parser::grammar::NumberExprRule;
impl HasRule for NumberExprNode{
    fn get_belonging_rule() -> Box<dyn Rule>{
        Box::new(NumberExprRule{})
    }
}
impl Display for NumberExprNode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.text())
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize)]
pub struct ValuePathExprNode {
    pub(crate) syntax: SyntaxNode,
}
impl AstNode for ValuePathExprNode {
    fn can_cast(kind: SyntaxKind) -> bool { kind == SyntaxKind::ValuePathExpr }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode { &self.syntax }
}
impl HasSyntaxKind for ValuePathExprNode{
    fn get_syntax_kind(&self) -> SyntaxKind{
        self.syntax().kind()
    }
}
impl HasTextRange for ValuePathExprNode{
    fn get_text_range(&self) -> TextRange{
        self.syntax().text_range()
    }
}


use lu_parser::grammar::ValuePathExprRule;
impl HasRule for ValuePathExprNode{
    fn get_belonging_rule() -> Box<dyn Rule>{
        Box::new(ValuePathExprRule{})
    }
}
impl Display for ValuePathExprNode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.text())
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize)]
pub struct ArrayExprNode {
    pub(crate) syntax: SyntaxNode,
}
impl AstNode for ArrayExprNode {
    fn can_cast(kind: SyntaxKind) -> bool { kind == SyntaxKind::ArrayExpr }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode { &self.syntax }
}
impl HasSyntaxKind for ArrayExprNode{
    fn get_syntax_kind(&self) -> SyntaxKind{
        self.syntax().kind()
    }
}
impl HasTextRange for ArrayExprNode{
    fn get_text_range(&self) -> TextRange{
        self.syntax().text_range()
    }
}


use lu_parser::grammar::ArrayExprRule;
impl HasRule for ArrayExprNode{
    fn get_belonging_rule() -> Box<dyn Rule>{
        Box::new(ArrayExprRule{})
    }
}
impl Display for ArrayExprNode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.text())
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize)]
pub struct TableExprNode {
    pub(crate) syntax: SyntaxNode,
}
impl AstNode for TableExprNode {
    fn can_cast(kind: SyntaxKind) -> bool { kind == SyntaxKind::TableExpr }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode { &self.syntax }
}
impl HasSyntaxKind for TableExprNode{
    fn get_syntax_kind(&self) -> SyntaxKind{
        self.syntax().kind()
    }
}
impl HasTextRange for TableExprNode{
    fn get_text_range(&self) -> TextRange{
        self.syntax().text_range()
    }
}


use lu_parser::grammar::TableExprRule;
impl HasRule for TableExprNode{
    fn get_belonging_rule() -> Box<dyn Rule>{
        Box::new(TableExprRule{})
    }
}
impl Display for TableExprNode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.text())
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize)]
pub struct BooleanExprNode {
    pub(crate) syntax: SyntaxNode,
}
impl AstNode for BooleanExprNode {
    fn can_cast(kind: SyntaxKind) -> bool { kind == SyntaxKind::BooleanExpr }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode { &self.syntax }
}
impl HasSyntaxKind for BooleanExprNode{
    fn get_syntax_kind(&self) -> SyntaxKind{
        self.syntax().kind()
    }
}
impl HasTextRange for BooleanExprNode{
    fn get_text_range(&self) -> TextRange{
        self.syntax().text_range()
    }
}

impl Display for BooleanExprNode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.text())
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, EnumAsInner)]
pub enum CmdOrValueExprElement {
    CmdStmt(CmdStmtNode),
    ValueExpr(ValueExprElement),
    }

impl CmdOrValueExprElement {
}

impl AstElement for CmdOrValueExprElement {
    fn can_cast(kind: SyntaxKind) -> bool { 
        
        
        ValueExprElement::can_cast(kind) ||
        
        
        match kind{
            CmdStmt | ValueExpr => true,
            _ => false,
        }
    }
    fn cast(syntax: SyntaxElement) -> Option<Self> {
        
        if let Some(casted) = ValueExprElement::cast(syntax.clone()){
                return Some(Self::ValueExpr(casted));
            }
        
        
        let res = match syntax.kind() {
            CmdStmt => CmdOrValueExprElement::CmdStmt(CmdStmtNode { syntax: syntax.into_node().unwrap() }),
            _ => return None,
        };
        Some(res)
    }

    fn syntax(&self) -> SyntaxElement {
        match self {
            
            CmdOrValueExprElement::CmdStmt(it) => it.syntax.clone().into(),
            
            
            CmdOrValueExprElement::ValueExpr(it) => it.syntax().clone().into(),
            
            }
    }
}
impl HasSyntaxKind for CmdOrValueExprElement{
    fn get_syntax_kind(&self) -> SyntaxKind{
        match self {
            CmdOrValueExprElement::CmdStmt(it) => it.get_syntax_kind(),
            CmdOrValueExprElement::ValueExpr(it) => it.get_syntax_kind(),
            }
    }
}

impl HasTextRange for CmdOrValueExprElement{
    fn get_text_range(&self) -> TextRange{
        self.syntax().text_range()
    }
}

impl Display for CmdOrValueExprElement {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.text())
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, EnumAsInner)]
pub enum ValueExprElement {
    BareWord(BareWordToken),
    NumberExpr(NumberExprNode),
    BooleanExpr(BooleanExprNode),
    MathExpr(MathExprNode),
    StringExpr(StringExprNode),
    ValuePathExpr(ValuePathExprNode),
    StrctCtorExpr(StrctCtorExprNode),
    ArrayExpr(ArrayExprNode),
    TableExpr(TableExprNode),
    CmdStmt(CmdStmtNode),
    }

impl ValueExprElement {
}

impl AstElement for ValueExprElement {
    fn can_cast(kind: SyntaxKind) -> bool { 
        
        
        
        
        
        
        
        
        
        
        
        match kind{
            BareWord | NumberExpr | BooleanExpr | MathExpr | StringExpr | ValuePathExpr | StrctCtorExpr | ArrayExpr | TableExpr | CmdStmt => true,
            _ => false,
        }
    }
    fn cast(syntax: SyntaxElement) -> Option<Self> {
        
        
        
        
        
        
        
        
        
        
        
        let res = match syntax.kind() {
            BareWord => ValueExprElement::BareWord(BareWordToken { syntax: syntax.into_token().unwrap() }),
            NumberExpr => ValueExprElement::NumberExpr(NumberExprNode { syntax: syntax.into_node().unwrap() }),
            BooleanExpr => ValueExprElement::BooleanExpr(BooleanExprNode { syntax: syntax.into_node().unwrap() }),
            MathExpr => ValueExprElement::MathExpr(MathExprNode { syntax: syntax.into_node().unwrap() }),
            StringExpr => ValueExprElement::StringExpr(StringExprNode { syntax: syntax.into_node().unwrap() }),
            ValuePathExpr => ValueExprElement::ValuePathExpr(ValuePathExprNode { syntax: syntax.into_node().unwrap() }),
            StrctCtorExpr => ValueExprElement::StrctCtorExpr(StrctCtorExprNode { syntax: syntax.into_node().unwrap() }),
            ArrayExpr => ValueExprElement::ArrayExpr(ArrayExprNode { syntax: syntax.into_node().unwrap() }),
            TableExpr => ValueExprElement::TableExpr(TableExprNode { syntax: syntax.into_node().unwrap() }),
            CmdStmt => ValueExprElement::CmdStmt(CmdStmtNode { syntax: syntax.into_node().unwrap() }),
            _ => return None,
        };
        Some(res)
    }

    fn syntax(&self) -> SyntaxElement {
        match self {
            
            ValueExprElement::BareWord(it) => it.syntax.clone().into(),
            
            
            ValueExprElement::NumberExpr(it) => it.syntax.clone().into(),
            
            
            ValueExprElement::BooleanExpr(it) => it.syntax.clone().into(),
            
            
            ValueExprElement::MathExpr(it) => it.syntax.clone().into(),
            
            
            ValueExprElement::StringExpr(it) => it.syntax.clone().into(),
            
            
            ValueExprElement::ValuePathExpr(it) => it.syntax.clone().into(),
            
            
            ValueExprElement::StrctCtorExpr(it) => it.syntax.clone().into(),
            
            
            ValueExprElement::ArrayExpr(it) => it.syntax.clone().into(),
            
            
            ValueExprElement::TableExpr(it) => it.syntax.clone().into(),
            
            
            ValueExprElement::CmdStmt(it) => it.syntax.clone().into(),
            
            }
    }
}
impl HasSyntaxKind for ValueExprElement{
    fn get_syntax_kind(&self) -> SyntaxKind{
        match self {
            ValueExprElement::BareWord(it) => it.get_syntax_kind(),
            ValueExprElement::NumberExpr(it) => it.get_syntax_kind(),
            ValueExprElement::BooleanExpr(it) => it.get_syntax_kind(),
            ValueExprElement::MathExpr(it) => it.get_syntax_kind(),
            ValueExprElement::StringExpr(it) => it.get_syntax_kind(),
            ValueExprElement::ValuePathExpr(it) => it.get_syntax_kind(),
            ValueExprElement::StrctCtorExpr(it) => it.get_syntax_kind(),
            ValueExprElement::ArrayExpr(it) => it.get_syntax_kind(),
            ValueExprElement::TableExpr(it) => it.get_syntax_kind(),
            ValueExprElement::CmdStmt(it) => it.get_syntax_kind(),
            }
    }
}

impl HasTextRange for ValueExprElement{
    fn get_text_range(&self) -> TextRange{
        self.syntax().text_range()
    }
}


use lu_parser::grammar::ValueExprRule;
impl HasRule for ValueExprElement{
    fn get_belonging_rule() -> Box<dyn Rule>{
        Box::new(ValueExprRule{})
    }
}
impl Display for ValueExprElement {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.text())
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, EnumAsInner)]
pub enum StatementElement {
    RetStmt(RetStmtNode),
    ForStmt(ForStmtNode),
    LetStmt(LetStmtNode),
    FnStmt(FnStmtNode),
    IfStmt(IfStmtNode),
    CmdStmt(CmdStmtNode),
    PipedCmdsStmt(PipedCmdsStmtNode),
    ValueExpr(ValueExprElement),
    }

impl StatementElement {
}

impl AstElement for StatementElement {
    fn can_cast(kind: SyntaxKind) -> bool { 
        
        
        
        
        
        
        
        
        ValueExprElement::can_cast(kind) ||
        
        
        match kind{
            RetStmt | ForStmt | LetStmt | FnStmt | IfStmt | CmdStmt | PipedCmdsStmt | ValueExpr => true,
            _ => false,
        }
    }
    fn cast(syntax: SyntaxElement) -> Option<Self> {
        
        
        
        
        
        
        
        if let Some(casted) = ValueExprElement::cast(syntax.clone()){
                return Some(Self::ValueExpr(casted));
            }
        
        
        let res = match syntax.kind() {
            RetStmt => StatementElement::RetStmt(RetStmtNode { syntax: syntax.into_node().unwrap() }),
            ForStmt => StatementElement::ForStmt(ForStmtNode { syntax: syntax.into_node().unwrap() }),
            LetStmt => StatementElement::LetStmt(LetStmtNode { syntax: syntax.into_node().unwrap() }),
            FnStmt => StatementElement::FnStmt(FnStmtNode { syntax: syntax.into_node().unwrap() }),
            IfStmt => StatementElement::IfStmt(IfStmtNode { syntax: syntax.into_node().unwrap() }),
            CmdStmt => StatementElement::CmdStmt(CmdStmtNode { syntax: syntax.into_node().unwrap() }),
            PipedCmdsStmt => StatementElement::PipedCmdsStmt(PipedCmdsStmtNode { syntax: syntax.into_node().unwrap() }),
            _ => return None,
        };
        Some(res)
    }

    fn syntax(&self) -> SyntaxElement {
        match self {
            
            StatementElement::RetStmt(it) => it.syntax.clone().into(),
            
            
            StatementElement::ForStmt(it) => it.syntax.clone().into(),
            
            
            StatementElement::LetStmt(it) => it.syntax.clone().into(),
            
            
            StatementElement::FnStmt(it) => it.syntax.clone().into(),
            
            
            StatementElement::IfStmt(it) => it.syntax.clone().into(),
            
            
            StatementElement::CmdStmt(it) => it.syntax.clone().into(),
            
            
            StatementElement::PipedCmdsStmt(it) => it.syntax.clone().into(),
            
            
            StatementElement::ValueExpr(it) => it.syntax().clone().into(),
            
            }
    }
}
impl HasSyntaxKind for StatementElement{
    fn get_syntax_kind(&self) -> SyntaxKind{
        match self {
            StatementElement::RetStmt(it) => it.get_syntax_kind(),
            StatementElement::ForStmt(it) => it.get_syntax_kind(),
            StatementElement::LetStmt(it) => it.get_syntax_kind(),
            StatementElement::FnStmt(it) => it.get_syntax_kind(),
            StatementElement::IfStmt(it) => it.get_syntax_kind(),
            StatementElement::CmdStmt(it) => it.get_syntax_kind(),
            StatementElement::PipedCmdsStmt(it) => it.get_syntax_kind(),
            StatementElement::ValueExpr(it) => it.get_syntax_kind(),
            }
    }
}

impl HasTextRange for StatementElement{
    fn get_text_range(&self) -> TextRange{
        self.syntax().text_range()
    }
}

impl Display for StatementElement {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.text())
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, EnumAsInner)]
pub enum ConditionElement {
    CmdStmt(CmdStmtNode),
    ValueExpr(ValueExprElement),
    }

impl ConditionElement {
}

impl AstElement for ConditionElement {
    fn can_cast(kind: SyntaxKind) -> bool { 
        
        
        ValueExprElement::can_cast(kind) ||
        
        
        match kind{
            CmdStmt | ValueExpr => true,
            _ => false,
        }
    }
    fn cast(syntax: SyntaxElement) -> Option<Self> {
        
        if let Some(casted) = ValueExprElement::cast(syntax.clone()){
                return Some(Self::ValueExpr(casted));
            }
        
        
        let res = match syntax.kind() {
            CmdStmt => ConditionElement::CmdStmt(CmdStmtNode { syntax: syntax.into_node().unwrap() }),
            _ => return None,
        };
        Some(res)
    }

    fn syntax(&self) -> SyntaxElement {
        match self {
            
            ConditionElement::CmdStmt(it) => it.syntax.clone().into(),
            
            
            ConditionElement::ValueExpr(it) => it.syntax().clone().into(),
            
            }
    }
}
impl HasSyntaxKind for ConditionElement{
    fn get_syntax_kind(&self) -> SyntaxKind{
        match self {
            ConditionElement::CmdStmt(it) => it.get_syntax_kind(),
            ConditionElement::ValueExpr(it) => it.get_syntax_kind(),
            }
    }
}

impl HasTextRange for ConditionElement{
    fn get_text_range(&self) -> TextRange{
        self.syntax().text_range()
    }
}

impl Display for ConditionElement {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.text())
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, EnumAsInner)]
pub enum OperatorExprElement {
    PlusSign(PlusSignToken),
    MinusSign(MinusSignToken),
    MultSign(MultSignToken),
    DivSign(DivSignToken),
    LessThanSign(LessThanSignToken),
    LessOrEqualSign(LessOrEqualSignToken),
    EqualitySign(EqualitySignToken),
    InequalitySign(InequalitySignToken),
    BiggerThanSign(BiggerThanSignToken),
    BiggerOrEqualSign(BiggerOrEqualSignToken),
    DivAssignSign(DivAssignSignToken),
    MulAssignSign(MulAssignSignToken),
    AddAssignSign(AddAssignSignToken),
    MinAssignSign(MinAssignSignToken),
    AssignSign(AssignSignToken),
    }

impl OperatorExprElement {
}

impl AstElement for OperatorExprElement {
    fn can_cast(kind: SyntaxKind) -> bool { 
        
        
        
        
        
        
        
        
        
        
        
        
        
        
        
        
        match kind{
            PlusSign | MinusSign | MultSign | DivSign | LessThanSign | LessOrEqualSign | EqualitySign | InequalitySign | BiggerThanSign | BiggerOrEqualSign | DivAssignSign | MulAssignSign | AddAssignSign | MinAssignSign | AssignSign => true,
            _ => false,
        }
    }
    fn cast(syntax: SyntaxElement) -> Option<Self> {
        
        
        
        
        
        
        
        
        
        
        
        
        
        
        
        
        let res = match syntax.kind() {
            PlusSign => OperatorExprElement::PlusSign(PlusSignToken { syntax: syntax.into_token().unwrap() }),
            MinusSign => OperatorExprElement::MinusSign(MinusSignToken { syntax: syntax.into_token().unwrap() }),
            MultSign => OperatorExprElement::MultSign(MultSignToken { syntax: syntax.into_token().unwrap() }),
            DivSign => OperatorExprElement::DivSign(DivSignToken { syntax: syntax.into_token().unwrap() }),
            LessThanSign => OperatorExprElement::LessThanSign(LessThanSignToken { syntax: syntax.into_token().unwrap() }),
            LessOrEqualSign => OperatorExprElement::LessOrEqualSign(LessOrEqualSignToken { syntax: syntax.into_token().unwrap() }),
            EqualitySign => OperatorExprElement::EqualitySign(EqualitySignToken { syntax: syntax.into_token().unwrap() }),
            InequalitySign => OperatorExprElement::InequalitySign(InequalitySignToken { syntax: syntax.into_token().unwrap() }),
            BiggerThanSign => OperatorExprElement::BiggerThanSign(BiggerThanSignToken { syntax: syntax.into_token().unwrap() }),
            BiggerOrEqualSign => OperatorExprElement::BiggerOrEqualSign(BiggerOrEqualSignToken { syntax: syntax.into_token().unwrap() }),
            DivAssignSign => OperatorExprElement::DivAssignSign(DivAssignSignToken { syntax: syntax.into_token().unwrap() }),
            MulAssignSign => OperatorExprElement::MulAssignSign(MulAssignSignToken { syntax: syntax.into_token().unwrap() }),
            AddAssignSign => OperatorExprElement::AddAssignSign(AddAssignSignToken { syntax: syntax.into_token().unwrap() }),
            MinAssignSign => OperatorExprElement::MinAssignSign(MinAssignSignToken { syntax: syntax.into_token().unwrap() }),
            AssignSign => OperatorExprElement::AssignSign(AssignSignToken { syntax: syntax.into_token().unwrap() }),
            _ => return None,
        };
        Some(res)
    }

    fn syntax(&self) -> SyntaxElement {
        match self {
            
            OperatorExprElement::PlusSign(it) => it.syntax.clone().into(),
            
            
            OperatorExprElement::MinusSign(it) => it.syntax.clone().into(),
            
            
            OperatorExprElement::MultSign(it) => it.syntax.clone().into(),
            
            
            OperatorExprElement::DivSign(it) => it.syntax.clone().into(),
            
            
            OperatorExprElement::LessThanSign(it) => it.syntax.clone().into(),
            
            
            OperatorExprElement::LessOrEqualSign(it) => it.syntax.clone().into(),
            
            
            OperatorExprElement::EqualitySign(it) => it.syntax.clone().into(),
            
            
            OperatorExprElement::InequalitySign(it) => it.syntax.clone().into(),
            
            
            OperatorExprElement::BiggerThanSign(it) => it.syntax.clone().into(),
            
            
            OperatorExprElement::BiggerOrEqualSign(it) => it.syntax.clone().into(),
            
            
            OperatorExprElement::DivAssignSign(it) => it.syntax.clone().into(),
            
            
            OperatorExprElement::MulAssignSign(it) => it.syntax.clone().into(),
            
            
            OperatorExprElement::AddAssignSign(it) => it.syntax.clone().into(),
            
            
            OperatorExprElement::MinAssignSign(it) => it.syntax.clone().into(),
            
            
            OperatorExprElement::AssignSign(it) => it.syntax.clone().into(),
            
            }
    }
}
impl HasSyntaxKind for OperatorExprElement{
    fn get_syntax_kind(&self) -> SyntaxKind{
        match self {
            OperatorExprElement::PlusSign(it) => it.get_syntax_kind(),
            OperatorExprElement::MinusSign(it) => it.get_syntax_kind(),
            OperatorExprElement::MultSign(it) => it.get_syntax_kind(),
            OperatorExprElement::DivSign(it) => it.get_syntax_kind(),
            OperatorExprElement::LessThanSign(it) => it.get_syntax_kind(),
            OperatorExprElement::LessOrEqualSign(it) => it.get_syntax_kind(),
            OperatorExprElement::EqualitySign(it) => it.get_syntax_kind(),
            OperatorExprElement::InequalitySign(it) => it.get_syntax_kind(),
            OperatorExprElement::BiggerThanSign(it) => it.get_syntax_kind(),
            OperatorExprElement::BiggerOrEqualSign(it) => it.get_syntax_kind(),
            OperatorExprElement::DivAssignSign(it) => it.get_syntax_kind(),
            OperatorExprElement::MulAssignSign(it) => it.get_syntax_kind(),
            OperatorExprElement::AddAssignSign(it) => it.get_syntax_kind(),
            OperatorExprElement::MinAssignSign(it) => it.get_syntax_kind(),
            OperatorExprElement::AssignSign(it) => it.get_syntax_kind(),
            }
    }
}

impl HasTextRange for OperatorExprElement{
    fn get_text_range(&self) -> TextRange{
        self.syntax().text_range()
    }
}

impl Display for OperatorExprElement {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.text())
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, EnumAsInner)]
pub enum LuTypeSpecifierElement {
    NumberKeyword(NumberKeywordToken),
    AnyKeyword(AnyKeywordToken),
    NilKeyword(NilKeywordToken),
    BoolKeyword(BoolKeywordToken),
    StringKeyword(StringKeywordToken),
    GenericType(GenericTypeToken),
    BareWord(BareWordToken),
    StrctName(StrctNameToken),
    ArrayType(ArrayTypeNode),
    FnType(FnTypeNode),
    }

impl LuTypeSpecifierElement {
}

impl AstElement for LuTypeSpecifierElement {
    fn can_cast(kind: SyntaxKind) -> bool { 
        
        
        
        
        
        
        
        
        
        
        
        match kind{
            NumberKeyword | AnyKeyword | NilKeyword | BoolKeyword | StringKeyword | GenericType | BareWord | StrctName | ArrayType | FnType => true,
            _ => false,
        }
    }
    fn cast(syntax: SyntaxElement) -> Option<Self> {
        
        
        
        
        
        
        
        
        
        
        
        let res = match syntax.kind() {
            NumberKeyword => LuTypeSpecifierElement::NumberKeyword(NumberKeywordToken { syntax: syntax.into_token().unwrap() }),
            AnyKeyword => LuTypeSpecifierElement::AnyKeyword(AnyKeywordToken { syntax: syntax.into_token().unwrap() }),
            NilKeyword => LuTypeSpecifierElement::NilKeyword(NilKeywordToken { syntax: syntax.into_token().unwrap() }),
            BoolKeyword => LuTypeSpecifierElement::BoolKeyword(BoolKeywordToken { syntax: syntax.into_token().unwrap() }),
            StringKeyword => LuTypeSpecifierElement::StringKeyword(StringKeywordToken { syntax: syntax.into_token().unwrap() }),
            GenericType => LuTypeSpecifierElement::GenericType(GenericTypeToken { syntax: syntax.into_token().unwrap() }),
            BareWord => LuTypeSpecifierElement::BareWord(BareWordToken { syntax: syntax.into_token().unwrap() }),
            StrctName => LuTypeSpecifierElement::StrctName(StrctNameToken { syntax: syntax.into_token().unwrap() }),
            ArrayType => LuTypeSpecifierElement::ArrayType(ArrayTypeNode { syntax: syntax.into_node().unwrap() }),
            FnType => LuTypeSpecifierElement::FnType(FnTypeNode { syntax: syntax.into_node().unwrap() }),
            _ => return None,
        };
        Some(res)
    }

    fn syntax(&self) -> SyntaxElement {
        match self {
            
            LuTypeSpecifierElement::NumberKeyword(it) => it.syntax.clone().into(),
            
            
            LuTypeSpecifierElement::AnyKeyword(it) => it.syntax.clone().into(),
            
            
            LuTypeSpecifierElement::NilKeyword(it) => it.syntax.clone().into(),
            
            
            LuTypeSpecifierElement::BoolKeyword(it) => it.syntax.clone().into(),
            
            
            LuTypeSpecifierElement::StringKeyword(it) => it.syntax.clone().into(),
            
            
            LuTypeSpecifierElement::GenericType(it) => it.syntax.clone().into(),
            
            
            LuTypeSpecifierElement::BareWord(it) => it.syntax.clone().into(),
            
            
            LuTypeSpecifierElement::StrctName(it) => it.syntax.clone().into(),
            
            
            LuTypeSpecifierElement::ArrayType(it) => it.syntax.clone().into(),
            
            
            LuTypeSpecifierElement::FnType(it) => it.syntax.clone().into(),
            
            }
    }
}
impl HasSyntaxKind for LuTypeSpecifierElement{
    fn get_syntax_kind(&self) -> SyntaxKind{
        match self {
            LuTypeSpecifierElement::NumberKeyword(it) => it.get_syntax_kind(),
            LuTypeSpecifierElement::AnyKeyword(it) => it.get_syntax_kind(),
            LuTypeSpecifierElement::NilKeyword(it) => it.get_syntax_kind(),
            LuTypeSpecifierElement::BoolKeyword(it) => it.get_syntax_kind(),
            LuTypeSpecifierElement::StringKeyword(it) => it.get_syntax_kind(),
            LuTypeSpecifierElement::GenericType(it) => it.get_syntax_kind(),
            LuTypeSpecifierElement::BareWord(it) => it.get_syntax_kind(),
            LuTypeSpecifierElement::StrctName(it) => it.get_syntax_kind(),
            LuTypeSpecifierElement::ArrayType(it) => it.get_syntax_kind(),
            LuTypeSpecifierElement::FnType(it) => it.get_syntax_kind(),
            }
    }
}

impl HasTextRange for LuTypeSpecifierElement{
    fn get_text_range(&self) -> TextRange{
        self.syntax().text_range()
    }
}

impl Display for LuTypeSpecifierElement {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.text())
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, EnumAsInner)]
pub enum CmdArgElement {
    ShortFlag(ShortFlagToken),
    LongFlag(LongFlagToken),
    ValueExpr(ValueExprElement),
    }

impl CmdArgElement {
}

impl AstElement for CmdArgElement {
    fn can_cast(kind: SyntaxKind) -> bool { 
        
        
        
        ValueExprElement::can_cast(kind) ||
        
        
        match kind{
            ShortFlag | LongFlag | ValueExpr => true,
            _ => false,
        }
    }
    fn cast(syntax: SyntaxElement) -> Option<Self> {
        
        
        if let Some(casted) = ValueExprElement::cast(syntax.clone()){
                return Some(Self::ValueExpr(casted));
            }
        
        
        let res = match syntax.kind() {
            ShortFlag => CmdArgElement::ShortFlag(ShortFlagToken { syntax: syntax.into_token().unwrap() }),
            LongFlag => CmdArgElement::LongFlag(LongFlagToken { syntax: syntax.into_token().unwrap() }),
            _ => return None,
        };
        Some(res)
    }

    fn syntax(&self) -> SyntaxElement {
        match self {
            
            CmdArgElement::ShortFlag(it) => it.syntax.clone().into(),
            
            
            CmdArgElement::LongFlag(it) => it.syntax.clone().into(),
            
            
            CmdArgElement::ValueExpr(it) => it.syntax().clone().into(),
            
            }
    }
}
impl HasSyntaxKind for CmdArgElement{
    fn get_syntax_kind(&self) -> SyntaxKind{
        match self {
            CmdArgElement::ShortFlag(it) => it.get_syntax_kind(),
            CmdArgElement::LongFlag(it) => it.get_syntax_kind(),
            CmdArgElement::ValueExpr(it) => it.get_syntax_kind(),
            }
    }
}

impl HasTextRange for CmdArgElement{
    fn get_text_range(&self) -> TextRange{
        self.syntax().text_range()
    }
}

impl Display for CmdArgElement {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.text())
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, EnumAsInner)]
pub enum FlagElement {
    ShortFlag(ShortFlagToken),
    LongFlag(LongFlagToken),
    }

impl FlagElement {
}

impl AstElement for FlagElement {
    fn can_cast(kind: SyntaxKind) -> bool { 
        
        
        
        match kind{
            ShortFlag | LongFlag => true,
            _ => false,
        }
    }
    fn cast(syntax: SyntaxElement) -> Option<Self> {
        
        
        
        let res = match syntax.kind() {
            ShortFlag => FlagElement::ShortFlag(ShortFlagToken { syntax: syntax.into_token().unwrap() }),
            LongFlag => FlagElement::LongFlag(LongFlagToken { syntax: syntax.into_token().unwrap() }),
            _ => return None,
        };
        Some(res)
    }

    fn syntax(&self) -> SyntaxElement {
        match self {
            
            FlagElement::ShortFlag(it) => it.syntax.clone().into(),
            
            
            FlagElement::LongFlag(it) => it.syntax.clone().into(),
            
            }
    }
}
impl HasSyntaxKind for FlagElement{
    fn get_syntax_kind(&self) -> SyntaxKind{
        match self {
            FlagElement::ShortFlag(it) => it.get_syntax_kind(),
            FlagElement::LongFlag(it) => it.get_syntax_kind(),
            }
    }
}

impl HasTextRange for FlagElement{
    fn get_text_range(&self) -> TextRange{
        self.syntax().text_range()
    }
}

impl Display for FlagElement {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.text())
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, EnumAsInner)]
pub enum RedirToValueElement {
    ValuePathExpr(ValuePathExprNode),
    BareWord(BareWordToken),
    }

impl RedirToValueElement {
}

impl AstElement for RedirToValueElement {
    fn can_cast(kind: SyntaxKind) -> bool { 
        
        
        
        match kind{
            ValuePathExpr | BareWord => true,
            _ => false,
        }
    }
    fn cast(syntax: SyntaxElement) -> Option<Self> {
        
        
        
        let res = match syntax.kind() {
            ValuePathExpr => RedirToValueElement::ValuePathExpr(ValuePathExprNode { syntax: syntax.into_node().unwrap() }),
            BareWord => RedirToValueElement::BareWord(BareWordToken { syntax: syntax.into_token().unwrap() }),
            _ => return None,
        };
        Some(res)
    }

    fn syntax(&self) -> SyntaxElement {
        match self {
            
            RedirToValueElement::ValuePathExpr(it) => it.syntax.clone().into(),
            
            
            RedirToValueElement::BareWord(it) => it.syntax.clone().into(),
            
            }
    }
}
impl HasSyntaxKind for RedirToValueElement{
    fn get_syntax_kind(&self) -> SyntaxKind{
        match self {
            RedirToValueElement::ValuePathExpr(it) => it.get_syntax_kind(),
            RedirToValueElement::BareWord(it) => it.get_syntax_kind(),
            }
    }
}

impl HasTextRange for RedirToValueElement{
    fn get_text_range(&self) -> TextRange{
        self.syntax().text_range()
    }
}

impl Display for RedirToValueElement {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.text())
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, EnumAsInner)]
pub enum FileNameElement {
    AbsFileName(AbsFileNameNode),
    RelFileName(RelFileNameNode),
    }

impl FileNameElement {
}

impl AstElement for FileNameElement {
    fn can_cast(kind: SyntaxKind) -> bool { 
        
        
        
        match kind{
            AbsFileName | RelFileName => true,
            _ => false,
        }
    }
    fn cast(syntax: SyntaxElement) -> Option<Self> {
        
        
        
        let res = match syntax.kind() {
            AbsFileName => FileNameElement::AbsFileName(AbsFileNameNode { syntax: syntax.into_node().unwrap() }),
            RelFileName => FileNameElement::RelFileName(RelFileNameNode { syntax: syntax.into_node().unwrap() }),
            _ => return None,
        };
        Some(res)
    }

    fn syntax(&self) -> SyntaxElement {
        match self {
            
            FileNameElement::AbsFileName(it) => it.syntax.clone().into(),
            
            
            FileNameElement::RelFileName(it) => it.syntax.clone().into(),
            
            }
    }
}
impl HasSyntaxKind for FileNameElement{
    fn get_syntax_kind(&self) -> SyntaxKind{
        match self {
            FileNameElement::AbsFileName(it) => it.get_syntax_kind(),
            FileNameElement::RelFileName(it) => it.get_syntax_kind(),
            }
    }
}

impl HasTextRange for FileNameElement{
    fn get_text_range(&self) -> TextRange{
        self.syntax().text_range()
    }
}

impl Display for FileNameElement {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.text())
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, EnumAsInner)]
pub enum FileNamePartElement {
    BareWord(BareWordToken),
    DivSign(DivSignToken),
    Point(PointToken),
    MinusSign(MinusSignToken),
    DoublePoint(DoublePointToken),
    Number(NumberToken),
    }

impl FileNamePartElement {
}

impl AstElement for FileNamePartElement {
    fn can_cast(kind: SyntaxKind) -> bool { 
        
        
        
        
        
        
        
        match kind{
            BareWord | DivSign | Point | MinusSign | DoublePoint | Number => true,
            _ => false,
        }
    }
    fn cast(syntax: SyntaxElement) -> Option<Self> {
        
        
        
        
        
        
        
        let res = match syntax.kind() {
            BareWord => FileNamePartElement::BareWord(BareWordToken { syntax: syntax.into_token().unwrap() }),
            DivSign => FileNamePartElement::DivSign(DivSignToken { syntax: syntax.into_token().unwrap() }),
            Point => FileNamePartElement::Point(PointToken { syntax: syntax.into_token().unwrap() }),
            MinusSign => FileNamePartElement::MinusSign(MinusSignToken { syntax: syntax.into_token().unwrap() }),
            DoublePoint => FileNamePartElement::DoublePoint(DoublePointToken { syntax: syntax.into_token().unwrap() }),
            Number => FileNamePartElement::Number(NumberToken { syntax: syntax.into_token().unwrap() }),
            _ => return None,
        };
        Some(res)
    }

    fn syntax(&self) -> SyntaxElement {
        match self {
            
            FileNamePartElement::BareWord(it) => it.syntax.clone().into(),
            
            
            FileNamePartElement::DivSign(it) => it.syntax.clone().into(),
            
            
            FileNamePartElement::Point(it) => it.syntax.clone().into(),
            
            
            FileNamePartElement::MinusSign(it) => it.syntax.clone().into(),
            
            
            FileNamePartElement::DoublePoint(it) => it.syntax.clone().into(),
            
            
            FileNamePartElement::Number(it) => it.syntax.clone().into(),
            
            }
    }
}
impl HasSyntaxKind for FileNamePartElement{
    fn get_syntax_kind(&self) -> SyntaxKind{
        match self {
            FileNamePartElement::BareWord(it) => it.get_syntax_kind(),
            FileNamePartElement::DivSign(it) => it.get_syntax_kind(),
            FileNamePartElement::Point(it) => it.get_syntax_kind(),
            FileNamePartElement::MinusSign(it) => it.get_syntax_kind(),
            FileNamePartElement::DoublePoint(it) => it.get_syntax_kind(),
            FileNamePartElement::Number(it) => it.get_syntax_kind(),
            }
    }
}

impl HasTextRange for FileNamePartElement{
    fn get_text_range(&self) -> TextRange{
        self.syntax().text_range()
    }
}

impl Display for FileNamePartElement {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.text())
    }
}

