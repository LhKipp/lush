#[allow(unused_imports)]


use crate::{
    ast::{self, support, AstChildren, AstNode},
    SyntaxKind::{self, *},
    SyntaxNode
};



#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct NumberNode {
    pub(crate) syntax: SyntaxNode,
}

impl NumberNode {
}

impl AstNode for NumberNode {
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


#[derive(Debug, Clone, PartialEq, Eq, Hash)]
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


#[derive(Debug, Clone, PartialEq, Eq, Hash)]
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


#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct WhitespaceNode {
    pub(crate) syntax: SyntaxNode,
}

impl WhitespaceNode {
}

impl AstNode for WhitespaceNode {
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
pub struct CommentNode {
    pub(crate) syntax: SyntaxNode,
}

impl CommentNode {
}

impl AstNode for CommentNode {
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
pub struct NewlineNode {
    pub(crate) syntax: SyntaxNode,
}

impl NewlineNode {
}

impl AstNode for NewlineNode {
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


#[derive(Debug, Clone, PartialEq, Eq, Hash)]
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


#[derive(Debug, Clone, PartialEq, Eq, Hash)]
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


#[derive(Debug, Clone, PartialEq, Eq, Hash)]
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


#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct DoubleQuotedStringNode {
    pub(crate) syntax: SyntaxNode,
}

impl DoubleQuotedStringNode {
}

impl AstNode for DoubleQuotedStringNode {
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
pub struct SingleQuotedStringNode {
    pub(crate) syntax: SyntaxNode,
}

impl SingleQuotedStringNode {
}

impl AstNode for SingleQuotedStringNode {
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
pub struct ValuePathNode {
    pub(crate) syntax: SyntaxNode,
}

impl ValuePathNode {
}

impl AstNode for ValuePathNode {
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


#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ArrayNode {
    pub(crate) syntax: SyntaxNode,
}

impl ArrayNode {
}

impl AstNode for ArrayNode {
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
pub struct TableNode {
    pub(crate) syntax: SyntaxNode,
}

impl TableNode {
}

impl AstNode for TableNode {
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
pub struct ValueExprNode {
    pub(crate) syntax: SyntaxNode,
}

impl ValueExprNode {
}

impl AstNode for ValueExprNode {
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
