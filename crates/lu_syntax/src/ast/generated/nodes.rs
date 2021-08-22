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
pub enum StatementElement {
    ForStmt(ForStmtNode),
    LetStmt(LetStmtNode),
    FnStmt(FnStmtNode),
    IfStmt(IfStmtNode),
    CmdStmt(CmdStmtNode),
    }

impl StatementElement {
}

impl AstElement for StatementElement {
    fn can_cast(kind: SyntaxKind) -> bool { 
        
        
        
        
        
        
        match kind{
            ForStmt | LetStmt | FnStmt | IfStmt | CmdStmt => true,
            _ => false,
        }
    }
    fn cast(syntax: SyntaxElement) -> Option<Self> {
        
        
        
        
        
        
        let res = match syntax.kind() {
            ForStmt => StatementElement::ForStmt(ForStmtNode { syntax: syntax.into_node().unwrap() }),
            LetStmt => StatementElement::LetStmt(LetStmtNode { syntax: syntax.into_node().unwrap() }),
            FnStmt => StatementElement::FnStmt(FnStmtNode { syntax: syntax.into_node().unwrap() }),
            IfStmt => StatementElement::IfStmt(IfStmtNode { syntax: syntax.into_node().unwrap() }),
            CmdStmt => StatementElement::CmdStmt(CmdStmtNode { syntax: syntax.into_node().unwrap() }),
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
            }
    }
}

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

