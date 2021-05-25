use logos::Logos;

#[allow(bad_style, missing_docs, unreachable_pub)]
#[derive(Logos, Copy, Clone, Debug, Eq, PartialEq, Hash)]
pub enum SyntaxKind {
    #[error]
    Error,
    #[regex("[+-]?([0-9]+([.][0-9]*)?|[.][0-9]+)")]
    Number,
    #[regex("[.]+")]
    BareWord,
    Eof,
    Tombstone,
    #[regex("[ ]+")]
    Whitespace,
    #[regex("\n")]
    Newline,
    #[token("let")]
    Let,
    #[token("fn")]
    Fn,
    #[token("elif")]
    Elif,
    #[token("else")]
    Else,
    #[token("if")]
    If,
    #[token("while")]
    While,
    #[token("(")]
    LeftParenthesis,
    #[token(")")]
    RightParenthesis,
    #[token("{")]
    LeftCurlyBrackets,
    #[token("}")]
    RightCurlyBrackets,
    #[token("+")]
    PlusSign,
    #[token("-")]
    Minus,
    #[token("<")]
    LessThanSign,
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
    #[token("/")]
    FileSep,
    }

#[macro_export]
macro_rules! T {
    [Number] => {$crate::SyntaxKind::Number };
    [BareWord] => {$crate::SyntaxKind::BareWord };
    [Eof] => {$crate::SyntaxKind::Eof };
    [Tombstone] => {$crate::SyntaxKind::Tombstone };
    [Whitespace] => {$crate::SyntaxKind::Whitespace };
    [Newline] => {$crate::SyntaxKind::Newline };
    [let] => {$crate::SyntaxKind::Let };
    [fn] => {$crate::SyntaxKind::Fn };
    [elif] => {$crate::SyntaxKind::Elif };
    [else] => {$crate::SyntaxKind::Else };
    [if] => {$crate::SyntaxKind::If };
    [while] => {$crate::SyntaxKind::While };
    ["("] => {$crate::SyntaxKind::LeftParenthesis };
    [")"] => {$crate::SyntaxKind::RightParenthesis };
    ["{"] => {$crate::SyntaxKind::LeftCurlyBrackets };
    ["}"] => {$crate::SyntaxKind::RightCurlyBrackets };
    [+] => {$crate::SyntaxKind::PlusSign };
    [-] => {$crate::SyntaxKind::Minus };
    [<] => {$crate::SyntaxKind::LessThanSign };
    [=] => {$crate::SyntaxKind::AssignSign };
    [>>] => {$crate::SyntaxKind::RightStream };
    [|] => {$crate::SyntaxKind::Pipe };
    [$] => {$crate::SyntaxKind::Dollar };
    [.] => {$crate::SyntaxKind::Point };
    [/] => {$crate::SyntaxKind::FileSep };
    }
