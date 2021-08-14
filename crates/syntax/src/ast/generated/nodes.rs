#[allow(unused_imports)]


use crate::{
    ast::{self, support, AstChildren, AstNode},
    SyntaxKind::{self, *},
    SyntaxNode
};


#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Number {
    pub(crate) syntax: SyntaxNode,
}

impl Number {
}

impl AstNode for Number {
    fn can_cast(kind: SyntaxKind) -> bool { kind == SyntaxKind::Number }
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
pub struct Eof {
    pub(crate) syntax: SyntaxNode,
}

impl Eof {
}

impl AstNode for Eof {
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

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct SourceFile {
    pub(crate) syntax: SyntaxNode,
}

impl SourceFile {
}

impl AstNode for SourceFile {
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

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Tombstone {
    pub(crate) syntax: SyntaxNode,
}

impl Tombstone {
}

impl AstNode for Tombstone {
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

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Whitespace {
    pub(crate) syntax: SyntaxNode,
}

impl Whitespace {
}

impl AstNode for Whitespace {
    fn can_cast(kind: SyntaxKind) -> bool { kind == SyntaxKind::Whitespace }
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
pub struct Comment {
    pub(crate) syntax: SyntaxNode,
}

impl Comment {
}

impl AstNode for Comment {
    fn can_cast(kind: SyntaxKind) -> bool { kind == SyntaxKind::Comment }
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
pub struct Newline {
    pub(crate) syntax: SyntaxNode,
}

impl Newline {
}

impl AstNode for Newline {
    fn can_cast(kind: SyntaxKind) -> bool { kind == SyntaxKind::Newline }
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
pub struct LetStmt {
    pub(crate) syntax: SyntaxNode,
}

impl LetStmt {
}

impl AstNode for LetStmt {
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

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct FnStmt {
    pub(crate) syntax: SyntaxNode,
}

impl FnStmt {
}

impl AstNode for FnStmt {
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

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct CmdStmt {
    pub(crate) syntax: SyntaxNode,
}

impl CmdStmt {
}

impl AstNode for CmdStmt {
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

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct MathExpr {
    pub(crate) syntax: SyntaxNode,
}

impl MathExpr {
}

impl AstNode for MathExpr {
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

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct DoubleQuotedString {
    pub(crate) syntax: SyntaxNode,
}

impl DoubleQuotedString {
}

impl AstNode for DoubleQuotedString {
    fn can_cast(kind: SyntaxKind) -> bool { kind == SyntaxKind::DoubleQuotedString }
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
pub struct SingleQuotedString {
    pub(crate) syntax: SyntaxNode,
}

impl SingleQuotedString {
}

impl AstNode for SingleQuotedString {
    fn can_cast(kind: SyntaxKind) -> bool { kind == SyntaxKind::SingleQuotedString }
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
pub struct ValuePath {
    pub(crate) syntax: SyntaxNode,
}

impl ValuePath {
}

impl AstNode for ValuePath {
    fn can_cast(kind: SyntaxKind) -> bool { kind == SyntaxKind::ValuePath }
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
pub struct Signature {
    pub(crate) syntax: SyntaxNode,
}

impl Signature {
}

impl AstNode for Signature {
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

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Array {
    pub(crate) syntax: SyntaxNode,
}

impl Array {
}

impl AstNode for Array {
    fn can_cast(kind: SyntaxKind) -> bool { kind == SyntaxKind::Array }
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
pub struct Table {
    pub(crate) syntax: SyntaxNode,
}

impl Table {
}

impl AstNode for Table {
    fn can_cast(kind: SyntaxKind) -> bool { kind == SyntaxKind::Table }
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
pub struct ValueExpr {
    pub(crate) syntax: SyntaxNode,
}

impl ValueExpr {
}

impl AstNode for ValueExpr {
    fn can_cast(kind: SyntaxKind) -> bool { kind == SyntaxKind::ValueExpr }
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
pub struct StringExpr {
    pub(crate) syntax: SyntaxNode,
}

impl StringExpr {
}

impl AstNode for StringExpr {
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
