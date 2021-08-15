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
    #[regex("[ ]+")]
    Whitespace,
    #[regex("#.*\n")]
    Comment,
    #[regex("\n")]
    Newline,
    #[regex("[+-]?([0-9]+([.][0-9]*)?|[.][0-9]+)")]
    Number,
    #[error]
    Error,
    #[regex("[a-zA-Z]+", priority = 0)]
    BareWord,
    Eof,
    SourceFile,
    Tombstone,
    LetStmt,
    FnStmt,
    CmdStmt,
    Signature,
    MathExpr,
    StringExpr,
    ValuePathExpr,
    ArrayExpr,
    TableExpr,
    __LAST,
}

impl SyntaxKind{
    pub const fn name(self) -> &'static str {
        match self {
            SyntaxKind::LetKeyword => "LetKeyword",
            SyntaxKind::FnKeyword => "FnKeyword",
            SyntaxKind::ElifKeyword => "ElifKeyword",
            SyntaxKind::ElseKeyword => "ElseKeyword",
            SyntaxKind::IfKeyword => "IfKeyword",
            SyntaxKind::WhileKeyword => "WhileKeyword",
            SyntaxKind::EndKeyword => "EndKeyword",
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
            SyntaxKind::Whitespace => "Whitespace",
            SyntaxKind::Comment => "Comment",
            SyntaxKind::Newline => "Newline",
            SyntaxKind::Number => "Number",
            SyntaxKind::Error => "Error",
            SyntaxKind::BareWord => "BareWord",
            SyntaxKind::Eof => "Eof",
            SyntaxKind::SourceFile => "SourceFile",
            SyntaxKind::Tombstone => "Tombstone",
            SyntaxKind::LetStmt => "LetStmt",
            SyntaxKind::FnStmt => "FnStmt",
            SyntaxKind::CmdStmt => "CmdStmt",
            SyntaxKind::Signature => "Signature",
            SyntaxKind::MathExpr => "MathExpr",
            SyntaxKind::StringExpr => "StringExpr",
            SyntaxKind::ValuePathExpr => "ValuePathExpr",
            SyntaxKind::ArrayExpr => "ArrayExpr",
            SyntaxKind::TableExpr => "TableExpr",
            #[allow(unreachable_patterns)]
            _ => "", // For the future
        }
    }
}

#[macro_export]
macro_rules! T {
    [let] => {$crate::SyntaxKind::LetKeyword };
    [fn] => {$crate::SyntaxKind::FnKeyword };
    [elif] => {$crate::SyntaxKind::ElifKeyword };
    [else] => {$crate::SyntaxKind::ElseKeyword };
    [if] => {$crate::SyntaxKind::IfKeyword };
    [while] => {$crate::SyntaxKind::WhileKeyword };
    [end] => {$crate::SyntaxKind::EndKeyword };
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
