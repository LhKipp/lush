use logos::Logos;
use ::serde::{Deserialize, Serialize};

#[allow(bad_style, missing_docs, unreachable_pub)]
#[derive(Logos, Copy, Clone, Eq, PartialEq, Hash, Debug, Serialize, Deserialize)]
#[repr(u16)]
pub enum SyntaxKind {
    
    #[token("let")]
    LetKeyword,
    #[token("fn")]
    FnKeyword,
    #[token("for")]
    ForKeyword,
    #[token("elif")]
    ElifKeyword,
    #[token("else")]
    ElseKeyword,
    #[token("if")]
    IfKeyword,
    #[token("while")]
    WhileKeyword,
    #[token("end")]
    EndKeyword,
    #[token("begin")]
    BeginKeyword,
    #[token("in")]
    InKeyword,
    #[token("(")]
    LeftParenthesis,
    #[token(")")]
    RightParenthesis,
    #[token("{")]
    LeftCurlyBrackets,
    #[token("}")]
    RightCurlyBrackets,
    #[token("[")]
    LeftRectangularBrackets,
    #[token("]")]
    RightRectangularBrackets,
    #[token("+")]
    PlusSign,
    #[token("-")]
    MinusSign,
    #[token("*")]
    MultSign,
    #[token("/")]
    DivSign,
    #[token("<")]
    LessThanSign,
    #[token("<=")]
    LessOrEqualSign,
    #[token("==")]
    EqualitySign,
    #[token("!=")]
    InequalitySign,
    #[token(">")]
    BiggerThanSign,
    #[token(">=")]
    BiggerOrEqualSign,
    #[token("=")]
    AssignSign,
    #[token(">>")]
    RightStream,
    #[token("|")]
    Pipe,
    #[token("$")]
    Dollar,
    #[token(".")]
    Point,
    #[token("\"")]
    DoubleQuote,
    #[token("'")]
    SingleQuote,
    #[error]
    Error,
    ParserInternal,
    #[regex("[_a-zA-Z]+[_a-zA-Z0-9]*", priority = 0)]
    BareWord,
    #[regex("[ ]+")]
    Whitespace,
    #[regex("#.*\n")]
    Comment,
    #[regex("\n")]
    Newline,
    #[regex("[+-]?([0-9]+([.][0-9]*)?|[.][0-9]+)")]
    Number,
    VarDeclName,
    FnDeclName,
    Eof,
    SourceFile,
    Tombstone,
    LetStmt,
    FnStmt,
    IfStmt,
    IfBlock,
    ElifBlock,
    ElseBlock,
    ForStmt,
    CmdStmt,
    PipedCmdsStmt,
    BlockStmt,
    Signature,
    MathExpr,
    StringExpr,
    NumberExpr,
    StringContent,
    ValuePathExpr,
    ArrayExpr,
    TableExpr,
    ValueExpr,
    Statement,
    Condition,
    OperatorExpr,
    __LAST,
}

impl SyntaxKind{
    pub const fn name(self) -> &'static str {
        match self {
            SyntaxKind::LetKeyword => "LetKeyword",
            SyntaxKind::FnKeyword => "FnKeyword",
            SyntaxKind::ForKeyword => "ForKeyword",
            SyntaxKind::ElifKeyword => "ElifKeyword",
            SyntaxKind::ElseKeyword => "ElseKeyword",
            SyntaxKind::IfKeyword => "IfKeyword",
            SyntaxKind::WhileKeyword => "WhileKeyword",
            SyntaxKind::EndKeyword => "EndKeyword",
            SyntaxKind::BeginKeyword => "BeginKeyword",
            SyntaxKind::InKeyword => "InKeyword",
            SyntaxKind::LeftParenthesis => "LeftParenthesis",
            SyntaxKind::RightParenthesis => "RightParenthesis",
            SyntaxKind::LeftCurlyBrackets => "LeftCurlyBrackets",
            SyntaxKind::RightCurlyBrackets => "RightCurlyBrackets",
            SyntaxKind::LeftRectangularBrackets => "LeftRectangularBrackets",
            SyntaxKind::RightRectangularBrackets => "RightRectangularBrackets",
            SyntaxKind::PlusSign => "PlusSign",
            SyntaxKind::MinusSign => "MinusSign",
            SyntaxKind::MultSign => "MultSign",
            SyntaxKind::DivSign => "DivSign",
            SyntaxKind::LessThanSign => "LessThanSign",
            SyntaxKind::LessOrEqualSign => "LessOrEqualSign",
            SyntaxKind::EqualitySign => "EqualitySign",
            SyntaxKind::InequalitySign => "InequalitySign",
            SyntaxKind::BiggerThanSign => "BiggerThanSign",
            SyntaxKind::BiggerOrEqualSign => "BiggerOrEqualSign",
            SyntaxKind::AssignSign => "AssignSign",
            SyntaxKind::RightStream => "RightStream",
            SyntaxKind::Pipe => "Pipe",
            SyntaxKind::Dollar => "Dollar",
            SyntaxKind::Point => "Point",
            SyntaxKind::DoubleQuote => "DoubleQuote",
            SyntaxKind::SingleQuote => "SingleQuote",
            SyntaxKind::Error => "Error",
            SyntaxKind::ParserInternal => "ParserInternal",
            SyntaxKind::BareWord => "BareWord",
            SyntaxKind::Whitespace => "Whitespace",
            SyntaxKind::Comment => "Comment",
            SyntaxKind::Newline => "Newline",
            SyntaxKind::Number => "Number",
            SyntaxKind::VarDeclName => "VarDeclName",
            SyntaxKind::FnDeclName => "FnDeclName",
            SyntaxKind::Eof => "Eof",
            SyntaxKind::SourceFile => "SourceFile",
            SyntaxKind::Tombstone => "Tombstone",
            SyntaxKind::LetStmt => "LetStmt",
            SyntaxKind::FnStmt => "FnStmt",
            SyntaxKind::IfStmt => "IfStmt",
            SyntaxKind::IfBlock => "IfBlock",
            SyntaxKind::ElifBlock => "ElifBlock",
            SyntaxKind::ElseBlock => "ElseBlock",
            SyntaxKind::ForStmt => "ForStmt",
            SyntaxKind::CmdStmt => "CmdStmt",
            SyntaxKind::PipedCmdsStmt => "PipedCmdsStmt",
            SyntaxKind::BlockStmt => "BlockStmt",
            SyntaxKind::Signature => "Signature",
            SyntaxKind::MathExpr => "MathExpr",
            SyntaxKind::StringExpr => "StringExpr",
            SyntaxKind::NumberExpr => "NumberExpr",
            SyntaxKind::StringContent => "StringContent",
            SyntaxKind::ValuePathExpr => "ValuePathExpr",
            SyntaxKind::ArrayExpr => "ArrayExpr",
            SyntaxKind::TableExpr => "TableExpr",
            SyntaxKind::ValueExpr => "ValueExpr",
            SyntaxKind::Statement => "Statement",
            SyntaxKind::Condition => "Condition",
            SyntaxKind::OperatorExpr => "OperatorExpr",
            #[allow(unreachable_patterns)]
            _ => "", // For the future
        }
    }
}

#[macro_export]
macro_rules! T {
    [let] => {$crate::SyntaxKind::LetKeyword };
    [fn] => {$crate::SyntaxKind::FnKeyword };
    [for] => {$crate::SyntaxKind::ForKeyword };
    [elif] => {$crate::SyntaxKind::ElifKeyword };
    [else] => {$crate::SyntaxKind::ElseKeyword };
    [if] => {$crate::SyntaxKind::IfKeyword };
    [while] => {$crate::SyntaxKind::WhileKeyword };
    [end] => {$crate::SyntaxKind::EndKeyword };
    [begin] => {$crate::SyntaxKind::BeginKeyword };
    [in] => {$crate::SyntaxKind::InKeyword };
    ["("] => {$crate::SyntaxKind::LeftParenthesis };
    [")"] => {$crate::SyntaxKind::RightParenthesis };
    ["{"] => {$crate::SyntaxKind::LeftCurlyBrackets };
    ["}"] => {$crate::SyntaxKind::RightCurlyBrackets };
    ["["] => {$crate::SyntaxKind::LeftRectangularBrackets };
    ["]"] => {$crate::SyntaxKind::RightRectangularBrackets };
    [+] => {$crate::SyntaxKind::PlusSign };
    [-] => {$crate::SyntaxKind::MinusSign };
    [*] => {$crate::SyntaxKind::MultSign };
    [/] => {$crate::SyntaxKind::DivSign };
    [<] => {$crate::SyntaxKind::LessThanSign };
    [<=] => {$crate::SyntaxKind::LessOrEqualSign };
    [==] => {$crate::SyntaxKind::EqualitySign };
    [!=] => {$crate::SyntaxKind::InequalitySign };
    [>] => {$crate::SyntaxKind::BiggerThanSign };
    [>=] => {$crate::SyntaxKind::BiggerOrEqualSign };
    [=] => {$crate::SyntaxKind::AssignSign };
    [>>] => {$crate::SyntaxKind::RightStream };
    [|] => {$crate::SyntaxKind::Pipe };
    [$] => {$crate::SyntaxKind::Dollar };
    [.] => {$crate::SyntaxKind::Point };
    [DoubleQuote] => {$crate::SyntaxKind::DoubleQuote };
    [SingleQuote] => {$crate::SyntaxKind::SingleQuote };
    }
