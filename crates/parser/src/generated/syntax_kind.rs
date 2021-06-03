use logos::Logos;

const KEYWORD_STRINGS: &'static [&'static str] =
    &["let", "fn", "elif", "else", "if", "while", "end"];

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
    SourceFile,
    Tombstone,
    #[regex("[ ]+")]
    Whitespace,
    #[regex("#.*\n")]
    Comment,
    #[regex("\n")]
    Newline,
    LetStmt,
    FnStmt,
    CmdStmt,
    MathExpr,
    DoubleQuotedString,
    SingleQuotedString,
    ValuePath,
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
    #[token("end")]
    End,
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
    #[token("\"")]
    DoubleQuote,
    #[token("'")]
    SingleQuote,
}

impl SyntaxKind {
    pub fn is_keyword(kw: &str) -> bool {
        KEYWORD_STRINGS.contains(&kw)
    }
}

#[macro_export]
macro_rules! T {
    [Number] => {$crate::SyntaxKind::Number };
    [BareWord] => {$crate::SyntaxKind::BareWord };
    [Eof] => {$crate::SyntaxKind::Eof };
    [SourceFile] => {$crate::SyntaxKind::SourceFile };
    [Tombstone] => {$crate::SyntaxKind::Tombstone };
    [Whitespace] => {$crate::SyntaxKind::Whitespace };
    [Comment] => {$crate::SyntaxKind::Comment };
    [Newline] => {$crate::SyntaxKind::Newline };
    [LetStmt] => {$crate::SyntaxKind::LetStmt };
    [FnStmt] => {$crate::SyntaxKind::FnStmt };
    [CmdStmt] => {$crate::SyntaxKind::CmdStmt };
    [MathExpr] => {$crate::SyntaxKind::MathExpr };
    [DoubleQuotedString] => {$crate::SyntaxKind::DoubleQuotedString };
    [SingleQuotedString] => {$crate::SyntaxKind::SingleQuotedString };
    [ValuePath] => {$crate::SyntaxKind::ValuePath };
    [let] => {$crate::SyntaxKind::Let };
    [fn] => {$crate::SyntaxKind::Fn };
    [elif] => {$crate::SyntaxKind::Elif };
    [else] => {$crate::SyntaxKind::Else };
    [if] => {$crate::SyntaxKind::If };
    [while] => {$crate::SyntaxKind::While };
    [end] => {$crate::SyntaxKind::End };
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
    [DoubleQuote] => {$crate::SyntaxKind::DoubleQuote };
    [SingleQuote] => {$crate::SyntaxKind::SingleQuote };
    }
