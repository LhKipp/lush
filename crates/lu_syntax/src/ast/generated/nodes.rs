#[allow(unused_imports)]
use crate::{
    Rule,
    ast::{self, support, AstNodeChildren, AstElementChildren, AstNode, AstToken, AstElement, HasRule, HasSyntaxKind},
    SyntaxKind::{self, *},
    SyntaxNode, SyntaxToken, SyntaxElement
};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
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

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
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

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
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

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
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

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
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

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
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

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
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

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
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

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
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

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
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

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
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

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
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

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
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

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
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

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
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

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
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

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
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


use lu_parser::grammar::ArrayTypeRule;
impl HasRule for ArrayTypeNode{
    fn get_belonging_rule() -> Box<dyn Rule>{
        Box::new(ArrayTypeRule{})
    }
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
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

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
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

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
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

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
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

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
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

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
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

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
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

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
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

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
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

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
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

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
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

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
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

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
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

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
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

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
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

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
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

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
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

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
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

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
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

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
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

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
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

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
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

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
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

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
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

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
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

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
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

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
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

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
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

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
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

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
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

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
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

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
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

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
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

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
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

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
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

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
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

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
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

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
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

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
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

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
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

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
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

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
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

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
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


use lu_parser::grammar::SourceFileRule;
impl HasRule for SourceFileNode{
    fn get_belonging_rule() -> Box<dyn Rule>{
        Box::new(SourceFileRule{})
    }
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
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


use lu_parser::grammar::LetStmtRule;
impl HasRule for LetStmtNode{
    fn get_belonging_rule() -> Box<dyn Rule>{
        Box::new(LetStmtRule{})
    }
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
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


use lu_parser::grammar::FnStmtRule;
impl HasRule for FnStmtNode{
    fn get_belonging_rule() -> Box<dyn Rule>{
        Box::new(FnStmtRule{})
    }
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
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


use lu_parser::grammar::RetStmtRule;
impl HasRule for RetStmtNode{
    fn get_belonging_rule() -> Box<dyn Rule>{
        Box::new(RetStmtRule{})
    }
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
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


use lu_parser::grammar::IfStmtRule;
impl HasRule for IfStmtNode{
    fn get_belonging_rule() -> Box<dyn Rule>{
        Box::new(IfStmtRule{})
    }
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
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

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
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

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
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

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
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


use lu_parser::grammar::ForStmtRule;
impl HasRule for ForStmtNode{
    fn get_belonging_rule() -> Box<dyn Rule>{
        Box::new(ForStmtRule{})
    }
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
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


use lu_parser::grammar::CmdStmtRule;
impl HasRule for CmdStmtNode{
    fn get_belonging_rule() -> Box<dyn Rule>{
        Box::new(CmdStmtRule{})
    }
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
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


use lu_parser::grammar::PipedCmdsStmtRule;
impl HasRule for PipedCmdsStmtNode{
    fn get_belonging_rule() -> Box<dyn Rule>{
        Box::new(PipedCmdsStmtRule{})
    }
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
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

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
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


use lu_parser::grammar::SignatureRule;
impl HasRule for SignatureNode{
    fn get_belonging_rule() -> Box<dyn Rule>{
        Box::new(SignatureRule{})
    }
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
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

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
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

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
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


use lu_parser::grammar::LuTypeRule;
impl HasRule for LuTypeNode{
    fn get_belonging_rule() -> Box<dyn Rule>{
        Box::new(LuTypeRule{})
    }
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
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

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
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


use lu_parser::grammar::StringExprRule;
impl HasRule for StringExprNode{
    fn get_belonging_rule() -> Box<dyn Rule>{
        Box::new(StringExprRule{})
    }
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
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


use lu_parser::grammar::NumberExprRule;
impl HasRule for NumberExprNode{
    fn get_belonging_rule() -> Box<dyn Rule>{
        Box::new(NumberExprRule{})
    }
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
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


use lu_parser::grammar::ValuePathExprRule;
impl HasRule for ValuePathExprNode{
    fn get_belonging_rule() -> Box<dyn Rule>{
        Box::new(ValuePathExprRule{})
    }
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
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


use lu_parser::grammar::ArrayExprRule;
impl HasRule for ArrayExprNode{
    fn get_belonging_rule() -> Box<dyn Rule>{
        Box::new(ArrayExprRule{})
    }
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
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


use lu_parser::grammar::TableExprRule;
impl HasRule for TableExprNode{
    fn get_belonging_rule() -> Box<dyn Rule>{
        Box::new(TableExprRule{})
    }
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum CmdOrValueExprElement {
    CmdStmt(CmdStmtNode),
    PipedCmdsStmt(PipedCmdsStmtNode),
    ValueExpr(ValueExprElement),
    }

impl CmdOrValueExprElement {
}

impl AstElement for CmdOrValueExprElement {
    fn can_cast(kind: SyntaxKind) -> bool { 
        
        
        
        ValueExprElement::can_cast(kind) ||
        
        
        match kind{
            CmdStmt | PipedCmdsStmt | ValueExpr => true,
            _ => false,
        }
    }
    fn cast(syntax: SyntaxElement) -> Option<Self> {
        
        
        if let Some(casted) = ValueExprElement::cast(syntax.clone()){
                return Some(Self::ValueExpr(casted));
            }
        
        
        let res = match syntax.kind() {
            CmdStmt => CmdOrValueExprElement::CmdStmt(CmdStmtNode { syntax: syntax.into_node().unwrap() }),
            PipedCmdsStmt => CmdOrValueExprElement::PipedCmdsStmt(PipedCmdsStmtNode { syntax: syntax.into_node().unwrap() }),
            _ => return None,
        };
        Some(res)
    }

    fn syntax(&self) -> SyntaxElement {
        match self {
            
            CmdOrValueExprElement::CmdStmt(it) => it.syntax.clone().into(),
            
            
            CmdOrValueExprElement::PipedCmdsStmt(it) => it.syntax.clone().into(),
            
            
            CmdOrValueExprElement::ValueExpr(it) => it.syntax().clone().into(),
            
            }
    }
}
impl HasSyntaxKind for CmdOrValueExprElement{
    fn get_syntax_kind(&self) -> SyntaxKind{
        match self {
            CmdOrValueExprElement::CmdStmt(it) => it.get_syntax_kind(),
            CmdOrValueExprElement::PipedCmdsStmt(it) => it.get_syntax_kind(),
            CmdOrValueExprElement::ValueExpr(it) => it.get_syntax_kind(),
            }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum ValueExprElement {
    BareWord(BareWordToken),
    NumberExpr(NumberExprNode),
    MathExpr(MathExprNode),
    StringExpr(StringExprNode),
    ValuePathExpr(ValuePathExprNode),
    ArrayExpr(ArrayExprNode),
    TableExpr(TableExprNode),
    }

impl ValueExprElement {
}

impl AstElement for ValueExprElement {
    fn can_cast(kind: SyntaxKind) -> bool { 
        
        
        
        
        
        
        
        
        match kind{
            BareWord | NumberExpr | MathExpr | StringExpr | ValuePathExpr | ArrayExpr | TableExpr => true,
            _ => false,
        }
    }
    fn cast(syntax: SyntaxElement) -> Option<Self> {
        
        
        
        
        
        
        
        
        let res = match syntax.kind() {
            BareWord => ValueExprElement::BareWord(BareWordToken { syntax: syntax.into_token().unwrap() }),
            NumberExpr => ValueExprElement::NumberExpr(NumberExprNode { syntax: syntax.into_node().unwrap() }),
            MathExpr => ValueExprElement::MathExpr(MathExprNode { syntax: syntax.into_node().unwrap() }),
            StringExpr => ValueExprElement::StringExpr(StringExprNode { syntax: syntax.into_node().unwrap() }),
            ValuePathExpr => ValueExprElement::ValuePathExpr(ValuePathExprNode { syntax: syntax.into_node().unwrap() }),
            ArrayExpr => ValueExprElement::ArrayExpr(ArrayExprNode { syntax: syntax.into_node().unwrap() }),
            TableExpr => ValueExprElement::TableExpr(TableExprNode { syntax: syntax.into_node().unwrap() }),
            _ => return None,
        };
        Some(res)
    }

    fn syntax(&self) -> SyntaxElement {
        match self {
            
            ValueExprElement::BareWord(it) => it.syntax.clone().into(),
            
            
            ValueExprElement::NumberExpr(it) => it.syntax.clone().into(),
            
            
            ValueExprElement::MathExpr(it) => it.syntax.clone().into(),
            
            
            ValueExprElement::StringExpr(it) => it.syntax.clone().into(),
            
            
            ValueExprElement::ValuePathExpr(it) => it.syntax.clone().into(),
            
            
            ValueExprElement::ArrayExpr(it) => it.syntax.clone().into(),
            
            
            ValueExprElement::TableExpr(it) => it.syntax.clone().into(),
            
            }
    }
}
impl HasSyntaxKind for ValueExprElement{
    fn get_syntax_kind(&self) -> SyntaxKind{
        match self {
            ValueExprElement::BareWord(it) => it.get_syntax_kind(),
            ValueExprElement::NumberExpr(it) => it.get_syntax_kind(),
            ValueExprElement::MathExpr(it) => it.get_syntax_kind(),
            ValueExprElement::StringExpr(it) => it.get_syntax_kind(),
            ValueExprElement::ValuePathExpr(it) => it.get_syntax_kind(),
            ValueExprElement::ArrayExpr(it) => it.get_syntax_kind(),
            ValueExprElement::TableExpr(it) => it.get_syntax_kind(),
            }
    }
}


use lu_parser::grammar::ValueExprRule;
impl HasRule for ValueExprElement{
    fn get_belonging_rule() -> Box<dyn Rule>{
        Box::new(ValueExprRule{})
    }
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum StatementElement {
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
            ForStmt | LetStmt | FnStmt | IfStmt | CmdStmt | PipedCmdsStmt | ValueExpr => true,
            _ => false,
        }
    }
    fn cast(syntax: SyntaxElement) -> Option<Self> {
        
        
        
        
        
        
        if let Some(casted) = ValueExprElement::cast(syntax.clone()){
                return Some(Self::ValueExpr(casted));
            }
        
        
        let res = match syntax.kind() {
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

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
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

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
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
    AssignSign(AssignSignToken),
    RightStream(RightStreamToken),
    }

impl OperatorExprElement {
}

impl AstElement for OperatorExprElement {
    fn can_cast(kind: SyntaxKind) -> bool { 
        
        
        
        
        
        
        
        
        
        
        
        
        
        match kind{
            PlusSign | MinusSign | MultSign | DivSign | LessThanSign | LessOrEqualSign | EqualitySign | InequalitySign | BiggerThanSign | BiggerOrEqualSign | AssignSign | RightStream => true,
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
            AssignSign => OperatorExprElement::AssignSign(AssignSignToken { syntax: syntax.into_token().unwrap() }),
            RightStream => OperatorExprElement::RightStream(RightStreamToken { syntax: syntax.into_token().unwrap() }),
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
            
            
            OperatorExprElement::AssignSign(it) => it.syntax.clone().into(),
            
            
            OperatorExprElement::RightStream(it) => it.syntax.clone().into(),
            
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
            OperatorExprElement::AssignSign(it) => it.get_syntax_kind(),
            OperatorExprElement::RightStream(it) => it.get_syntax_kind(),
            }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum LuTypeSpecifierElement {
    NumberKeyword(NumberKeywordToken),
    AnyKeyword(AnyKeywordToken),
    NilKeyword(NilKeywordToken),
    BoolKeyword(BoolKeywordToken),
    StringKeyword(StringKeywordToken),
    FnKeyword(FnKeywordToken),
    ArrayType(ArrayTypeNode),
    BareWord(BareWordToken),
    }

impl LuTypeSpecifierElement {
}

impl AstElement for LuTypeSpecifierElement {
    fn can_cast(kind: SyntaxKind) -> bool { 
        
        
        
        
        
        
        
        
        
        match kind{
            NumberKeyword | AnyKeyword | NilKeyword | BoolKeyword | StringKeyword | FnKeyword | ArrayType | BareWord => true,
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
            FnKeyword => LuTypeSpecifierElement::FnKeyword(FnKeywordToken { syntax: syntax.into_token().unwrap() }),
            ArrayType => LuTypeSpecifierElement::ArrayType(ArrayTypeNode { syntax: syntax.into_node().unwrap() }),
            BareWord => LuTypeSpecifierElement::BareWord(BareWordToken { syntax: syntax.into_token().unwrap() }),
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
            
            
            LuTypeSpecifierElement::FnKeyword(it) => it.syntax.clone().into(),
            
            
            LuTypeSpecifierElement::ArrayType(it) => it.syntax.clone().into(),
            
            
            LuTypeSpecifierElement::BareWord(it) => it.syntax.clone().into(),
            
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
            LuTypeSpecifierElement::FnKeyword(it) => it.get_syntax_kind(),
            LuTypeSpecifierElement::ArrayType(it) => it.get_syntax_kind(),
            LuTypeSpecifierElement::BareWord(it) => it.get_syntax_kind(),
            }
    }
}

