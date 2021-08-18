#[allow(unused_imports)]

use crate::{
    Rule,
    ast::{self, support, AstNodeChildren, AstElementChildren, AstNode, AstToken, AstElement, HasRule},
    SyntaxKind::{self, *},
    SyntaxNode, SyntaxToken, SyntaxElement
};




#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct LetKeywordToken {
    pub(crate) syntax: SyntaxToken,
}

impl LetKeywordToken {
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




#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct FnKeywordToken {
    pub(crate) syntax: SyntaxToken,
}

impl FnKeywordToken {
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




#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ForKeywordToken {
    pub(crate) syntax: SyntaxToken,
}

impl ForKeywordToken {
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




#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ElifKeywordToken {
    pub(crate) syntax: SyntaxToken,
}

impl ElifKeywordToken {
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




#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ElseKeywordToken {
    pub(crate) syntax: SyntaxToken,
}

impl ElseKeywordToken {
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




#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct IfKeywordToken {
    pub(crate) syntax: SyntaxToken,
}

impl IfKeywordToken {
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




#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct WhileKeywordToken {
    pub(crate) syntax: SyntaxToken,
}

impl WhileKeywordToken {
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




#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct EndKeywordToken {
    pub(crate) syntax: SyntaxToken,
}

impl EndKeywordToken {
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




#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct InKeywordToken {
    pub(crate) syntax: SyntaxToken,
}

impl InKeywordToken {
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




#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct LeftParenthesisToken {
    pub(crate) syntax: SyntaxToken,
}

impl LeftParenthesisToken {
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




#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct RightParenthesisToken {
    pub(crate) syntax: SyntaxToken,
}

impl RightParenthesisToken {
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




#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct LeftCurlyBracketsToken {
    pub(crate) syntax: SyntaxToken,
}

impl LeftCurlyBracketsToken {
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




#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct RightCurlyBracketsToken {
    pub(crate) syntax: SyntaxToken,
}

impl RightCurlyBracketsToken {
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




#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct LeftRectangularBracketsToken {
    pub(crate) syntax: SyntaxToken,
}

impl LeftRectangularBracketsToken {
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




#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct RightRectangularBracketsToken {
    pub(crate) syntax: SyntaxToken,
}

impl RightRectangularBracketsToken {
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




#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct PlusSignToken {
    pub(crate) syntax: SyntaxToken,
}

impl PlusSignToken {
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




#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct MinusSignToken {
    pub(crate) syntax: SyntaxToken,
}

impl MinusSignToken {
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




#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct MultSignToken {
    pub(crate) syntax: SyntaxToken,
}

impl MultSignToken {
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




#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct DivSignToken {
    pub(crate) syntax: SyntaxToken,
}

impl DivSignToken {
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




#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct LessThanSignToken {
    pub(crate) syntax: SyntaxToken,
}

impl LessThanSignToken {
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




#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct LessOrEqualSignToken {
    pub(crate) syntax: SyntaxToken,
}

impl LessOrEqualSignToken {
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




#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct EqualitySignToken {
    pub(crate) syntax: SyntaxToken,
}

impl EqualitySignToken {
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




#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct InequalitySignToken {
    pub(crate) syntax: SyntaxToken,
}

impl InequalitySignToken {
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




#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct BiggerThanSignToken {
    pub(crate) syntax: SyntaxToken,
}

impl BiggerThanSignToken {
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




#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct BiggerOrEqualSignToken {
    pub(crate) syntax: SyntaxToken,
}

impl BiggerOrEqualSignToken {
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




#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct AssignSignToken {
    pub(crate) syntax: SyntaxToken,
}

impl AssignSignToken {
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




#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct RightStreamToken {
    pub(crate) syntax: SyntaxToken,
}

impl RightStreamToken {
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




#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct PipeToken {
    pub(crate) syntax: SyntaxToken,
}

impl PipeToken {
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




#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct DollarToken {
    pub(crate) syntax: SyntaxToken,
}

impl DollarToken {
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




#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct PointToken {
    pub(crate) syntax: SyntaxToken,
}

impl PointToken {
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




#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct DoubleQuoteToken {
    pub(crate) syntax: SyntaxToken,
}

impl DoubleQuoteToken {
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




#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct SingleQuoteToken {
    pub(crate) syntax: SyntaxToken,
}

impl SingleQuoteToken {
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




#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct WhitespaceToken {
    pub(crate) syntax: SyntaxToken,
}

impl WhitespaceToken {
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




#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct CommentToken {
    pub(crate) syntax: SyntaxToken,
}

impl CommentToken {
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




#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct NewlineToken {
    pub(crate) syntax: SyntaxToken,
}

impl NewlineToken {
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




use lu_parser::grammar::NumberRule;
impl HasRule for NumberToken{
    fn get_belonging_rule() -> Box<dyn Rule>{
        Box::new(NumberRule{})
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct NumberToken {
    pub(crate) syntax: SyntaxToken,
}

impl NumberToken {
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




#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ErrorToken {
    pub(crate) syntax: SyntaxToken,
}

impl ErrorToken {
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




#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct BareWordToken {
    pub(crate) syntax: SyntaxToken,
}

impl BareWordToken {
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




pub struct EofNode {
    pub(crate) syntax: SyntaxNode,
}

impl EofNode {
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



use lu_parser::grammar::SourceFileRule;
impl HasRule for SourceFileNode{
    fn get_belonging_rule() -> Box<dyn Rule>{
        Box::new(SourceFileRule{})
    }
}

pub struct SourceFileNode {
    pub(crate) syntax: SyntaxNode,
}

impl SourceFileNode {
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



pub struct TombstoneNode {
    pub(crate) syntax: SyntaxNode,
}

impl TombstoneNode {
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



use lu_parser::grammar::LetStmtRule;
impl HasRule for LetStmtNode{
    fn get_belonging_rule() -> Box<dyn Rule>{
        Box::new(LetStmtRule{})
    }
}

pub struct LetStmtNode {
    pub(crate) syntax: SyntaxNode,
}

impl LetStmtNode {
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



use lu_parser::grammar::FnStmtRule;
impl HasRule for FnStmtNode{
    fn get_belonging_rule() -> Box<dyn Rule>{
        Box::new(FnStmtRule{})
    }
}

pub struct FnStmtNode {
    pub(crate) syntax: SyntaxNode,
}

impl FnStmtNode {
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



use lu_parser::grammar::ForStmtRule;
impl HasRule for ForStmtNode{
    fn get_belonging_rule() -> Box<dyn Rule>{
        Box::new(ForStmtRule{})
    }
}

pub struct ForStmtNode {
    pub(crate) syntax: SyntaxNode,
}

impl ForStmtNode {
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



use lu_parser::grammar::CmdStmtRule;
impl HasRule for CmdStmtNode{
    fn get_belonging_rule() -> Box<dyn Rule>{
        Box::new(CmdStmtRule{})
    }
}

pub struct CmdStmtNode {
    pub(crate) syntax: SyntaxNode,
}

impl CmdStmtNode {
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



use lu_parser::grammar::SignatureRule;
impl HasRule for SignatureNode{
    fn get_belonging_rule() -> Box<dyn Rule>{
        Box::new(SignatureRule{})
    }
}

pub struct SignatureNode {
    pub(crate) syntax: SyntaxNode,
}

impl SignatureNode {
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



pub struct MathExprNode {
    pub(crate) syntax: SyntaxNode,
}

impl MathExprNode {
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



use lu_parser::grammar::StringExprRule;
impl HasRule for StringExprNode{
    fn get_belonging_rule() -> Box<dyn Rule>{
        Box::new(StringExprRule{})
    }
}

pub struct StringExprNode {
    pub(crate) syntax: SyntaxNode,
}

impl StringExprNode {
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



#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct StringContentToken {
    pub(crate) syntax: SyntaxToken,
}

impl StringContentToken {
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




use lu_parser::grammar::ValuePathExprRule;
impl HasRule for ValuePathExprNode{
    fn get_belonging_rule() -> Box<dyn Rule>{
        Box::new(ValuePathExprRule{})
    }
}

pub struct ValuePathExprNode {
    pub(crate) syntax: SyntaxNode,
}

impl ValuePathExprNode {
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



use lu_parser::grammar::ArrayExprRule;
impl HasRule for ArrayExprNode{
    fn get_belonging_rule() -> Box<dyn Rule>{
        Box::new(ArrayExprRule{})
    }
}

pub struct ArrayExprNode {
    pub(crate) syntax: SyntaxNode,
}

impl ArrayExprNode {
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



use lu_parser::grammar::TableExprRule;
impl HasRule for TableExprNode{
    fn get_belonging_rule() -> Box<dyn Rule>{
        Box::new(TableExprRule{})
    }
}

pub struct TableExprNode {
    pub(crate) syntax: SyntaxNode,
}

impl TableExprNode {
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
pub enum ValueExprNode {
    BareWord(BareWordToken),
    Number(NumberToken),
    MathExpr(MathExprNode),
    StringExpr(StringExprNode),
    ValuePathExpr(ValuePathExprNode),
    ArrayExpr(ArrayExprNode),
    TableExpr(TableExprNode),
    }

impl ValueExprNode {
}

impl AstElement for ValueExprNode {
    fn can_cast(kind: SyntaxKind) -> bool { 
        match kind{
            BareWord | Number | MathExpr | StringExpr | ValuePathExpr | ArrayExpr | TableExpr => true,
            _ => false,
        }
    }
    fn cast(syntax: SyntaxElement) -> Option<Self> {
        let res = match syntax.kind() {
            BareWord => ValueExprNode::BareWord(BareWordToken { syntax: syntax.into_token().unwrap() }),
            Number => ValueExprNode::Number(NumberToken { syntax: syntax.into_token().unwrap() }),
            MathExpr => ValueExprNode::MathExpr(MathExprNode { syntax: syntax.into_node().unwrap() }),
            StringExpr => ValueExprNode::StringExpr(StringExprNode { syntax: syntax.into_node().unwrap() }),
            ValuePathExpr => ValueExprNode::ValuePathExpr(ValuePathExprNode { syntax: syntax.into_node().unwrap() }),
            ArrayExpr => ValueExprNode::ArrayExpr(ArrayExprNode { syntax: syntax.into_node().unwrap() }),
            TableExpr => ValueExprNode::TableExpr(TableExprNode { syntax: syntax.into_node().unwrap() }),
            _ => return None,
        };
        Some(res)
    }

    fn syntax(&self) -> SyntaxElement {
        match self {
            ValueExprNode::BareWord(it) => it.syntax.clone().into(),
            ValueExprNode::Number(it) => it.syntax.clone().into(),
            ValueExprNode::MathExpr(it) => it.syntax.clone().into(),
            ValueExprNode::StringExpr(it) => it.syntax.clone().into(),
            ValueExprNode::ValuePathExpr(it) => it.syntax.clone().into(),
            ValueExprNode::ArrayExpr(it) => it.syntax.clone().into(),
            ValueExprNode::TableExpr(it) => it.syntax.clone().into(),
            }
    }
}
pub enum StatementNode {
    LetStmt(LetStmtNode),
    FnStmt(FnStmtNode),
    CmdStmt(CmdStmtNode),
    }

impl StatementNode {
}

impl AstNode for StatementNode {
    fn can_cast(kind: SyntaxKind) -> bool { 
        match kind{
            LetStmt | FnStmt | CmdStmt => true,
            _ => false,
        }
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        let res = match syntax.kind() {
            LetStmt => StatementNode::LetStmt(LetStmtNode { syntax }),
            FnStmt => StatementNode::FnStmt(FnStmtNode { syntax }),
            CmdStmt => StatementNode::CmdStmt(CmdStmtNode { syntax }),
            _ => return None,
        };
        Some(res)
    }

    fn syntax(&self) -> &SyntaxNode {
        match self {
            StatementNode::LetStmt(it) => &it.syntax,
            StatementNode::FnStmt(it) => &it.syntax,
            StatementNode::CmdStmt(it) => &it.syntax,
            }
    }
}
