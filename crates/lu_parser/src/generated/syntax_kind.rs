use logos::Logos;
use derive_more::Display;
use ::serde::{Deserialize, Serialize};

#[allow(bad_style, missing_docs, unreachable_pub)]
#[derive(Logos, Copy, Clone, Eq, PartialEq, Hash, Debug, Serialize, Deserialize, Display)]
#[repr(u16)]
pub enum SyntaxKind {
    
    #[token("impure")]
    ImpureKeyword,
    #[token("struct")]
    StrctKeyword,
    #[token("Opt")]
    OptionalTyKeyword,
    #[token("Some")]
    SomeKeyword,
    #[token("None")]
    NoneKeyword,
    #[token("as")]
    AsKeyword,
    #[token("req")]
    ReqKeyword,
    #[token("use")]
    UseKeyword,
    #[token("let")]
    LetKeyword,
    #[token("fn")]
    FnKeyword,
    #[token("for")]
    ForKeyword,
    #[token("if")]
    IfKeyword,
    #[token("ifopt")]
    IfOptKeyword,
    #[token("elif")]
    ElifKeyword,
    #[token("elifopt")]
    ElifOptKeyword,
    #[token("else")]
    ElseKeyword,
    #[token("while")]
    WhileKeyword,
    #[token("end")]
    EndKeyword,
    #[token("begin")]
    BeginKeyword,
    #[token("in")]
    InKeyword,
    #[token("any")]
    AnyKeyword,
    #[token("path")]
    PathKeyword,
    #[token("nil")]
    NilKeyword,
    #[token("bool")]
    BoolKeyword,
    #[token("true")]
    TrueKeyword,
    #[token("false")]
    FalseKeyword,
    #[token("num")]
    NumberKeyword,
    #[token("str")]
    StringKeyword,
    #[token("ret")]
    RetKeyword,
    #[regex("[TU][0-9]?", priority = 1000)]
    GenericType,
    ArrayType,
    FnType,
    OptModifier,
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
    #[token("*", priority = 3)]
    MultSign,
    #[token("/")]
    FilePathSep,
    #[token("//")]
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
    #[token(">>")]
    RightStream,
    #[token("/=")]
    DivAssignSign,
    #[token("*=")]
    MulAssignSign,
    #[token("+=")]
    AddAssignSign,
    #[token("-=")]
    MinAssignSign,
    #[token("=")]
    AssignSign,
    #[token("|")]
    Pipe,
    #[token("$")]
    Dollar,
    #[token("?")]
    QuestionMark,
    #[token(".")]
    Point,
    #[token(":")]
    DoublePoint,
    #[token("\"")]
    DoubleQuote,
    #[token("'")]
    SingleQuote,
    #[error]
    Error,
    ParserInternal,
    Eof,
    Tombstone,
    #[regex("[ ]+")]
    Whitespace,
    #[regex("#[^\n]*")]
    Comment,
    #[regex("\n")]
    Newline,
    #[regex("[_a-zA-Z]+[_a-zA-Z0-9]*", priority = 0)]
    BareWord,
    StringContent,
    VarDeclName,
    FnDeclName,
    ArgName,
    #[token("...[_a-zA-Z]+[_a-zA-Z0-9]*")]
    VarArgName,
    #[regex("--[_a-zA-Z]+[_a-zA-Z0-9]*")]
    LongFlag,
    #[regex("-[_a-zA-Z]+[_a-zA-Z0-9]*")]
    ShortFlag,
    #[regex("[+-]?([0-9]+([.][0-9]*)?|[.][0-9]+)", priority = 3)]
    Number,
    #[regex("[A-Z]+[_a-zA-Z0-9]*")]
    StrctName,
    StrctFieldName,
    AbsFileName,
    RelFileName,
    SourceFile,
    CliLine,
    Offset,
    StrctStmt,
    RedirStmt,
    UseStmt,
    PluginUseStmt,
    StrctField,
    StrctCtorExpr,
    StrctFieldCtorStmt,
    LetStmt,
    FnStmt,
    RetStmt,
    IfElifElseStmt,
    IfOptElifOptStmt,
    IfElifStmt,
    ElseStmt,
    IfBlock,
    ElifBlock,
    ElseBlock,
    ForStmt,
    CmdStmt,
    PipedCmdsStmt,
    BlockStmt,
    Signature,
    FlagSignature,
    ArgSignature,
    LuType,
    MathExpr,
    ParentherizedExpr,
    StringExpr,
    NumberExpr,
    ValuePathExpr,
    ArrayExpr,
    OptionalExpr,
    TableExpr,
    BooleanExpr,
    ValueExpr,
    Statement,
    Condition,
    OperatorExpr,
    LuTypeSpecifier,
    CmdArg,
    Flag,
    RedirToValue,
    FileName,
    FileNamePart,
    IfElifElseStmtPart,
    __LAST,
}

impl SyntaxKind{
    pub const fn name(self) -> &'static str {
        match self {
            SyntaxKind::ImpureKeyword => "ImpureKeyword",
            SyntaxKind::StrctKeyword => "StrctKeyword",
            SyntaxKind::OptionalTyKeyword => "OptionalTyKeyword",
            SyntaxKind::SomeKeyword => "SomeKeyword",
            SyntaxKind::NoneKeyword => "NoneKeyword",
            SyntaxKind::AsKeyword => "AsKeyword",
            SyntaxKind::ReqKeyword => "ReqKeyword",
            SyntaxKind::UseKeyword => "UseKeyword",
            SyntaxKind::LetKeyword => "LetKeyword",
            SyntaxKind::FnKeyword => "FnKeyword",
            SyntaxKind::ForKeyword => "ForKeyword",
            SyntaxKind::IfKeyword => "IfKeyword",
            SyntaxKind::IfOptKeyword => "IfOptKeyword",
            SyntaxKind::ElifKeyword => "ElifKeyword",
            SyntaxKind::ElifOptKeyword => "ElifOptKeyword",
            SyntaxKind::ElseKeyword => "ElseKeyword",
            SyntaxKind::WhileKeyword => "WhileKeyword",
            SyntaxKind::EndKeyword => "EndKeyword",
            SyntaxKind::BeginKeyword => "BeginKeyword",
            SyntaxKind::InKeyword => "InKeyword",
            SyntaxKind::AnyKeyword => "AnyKeyword",
            SyntaxKind::PathKeyword => "PathKeyword",
            SyntaxKind::NilKeyword => "NilKeyword",
            SyntaxKind::BoolKeyword => "BoolKeyword",
            SyntaxKind::TrueKeyword => "TrueKeyword",
            SyntaxKind::FalseKeyword => "FalseKeyword",
            SyntaxKind::NumberKeyword => "NumberKeyword",
            SyntaxKind::StringKeyword => "StringKeyword",
            SyntaxKind::RetKeyword => "RetKeyword",
            SyntaxKind::GenericType => "GenericType",
            SyntaxKind::ArrayType => "ArrayType",
            SyntaxKind::FnType => "FnType",
            SyntaxKind::OptModifier => "OptModifier",
            SyntaxKind::LeftParenthesis => "LeftParenthesis",
            SyntaxKind::RightParenthesis => "RightParenthesis",
            SyntaxKind::LeftCurlyBrackets => "LeftCurlyBrackets",
            SyntaxKind::RightCurlyBrackets => "RightCurlyBrackets",
            SyntaxKind::LeftRectangularBrackets => "LeftRectangularBrackets",
            SyntaxKind::RightRectangularBrackets => "RightRectangularBrackets",
            SyntaxKind::PlusSign => "PlusSign",
            SyntaxKind::MinusSign => "MinusSign",
            SyntaxKind::MultSign => "MultSign",
            SyntaxKind::FilePathSep => "FilePathSep",
            SyntaxKind::DivSign => "DivSign",
            SyntaxKind::LessThanSign => "LessThanSign",
            SyntaxKind::LessOrEqualSign => "LessOrEqualSign",
            SyntaxKind::EqualitySign => "EqualitySign",
            SyntaxKind::InequalitySign => "InequalitySign",
            SyntaxKind::BiggerThanSign => "BiggerThanSign",
            SyntaxKind::BiggerOrEqualSign => "BiggerOrEqualSign",
            SyntaxKind::RightStream => "RightStream",
            SyntaxKind::DivAssignSign => "DivAssignSign",
            SyntaxKind::MulAssignSign => "MulAssignSign",
            SyntaxKind::AddAssignSign => "AddAssignSign",
            SyntaxKind::MinAssignSign => "MinAssignSign",
            SyntaxKind::AssignSign => "AssignSign",
            SyntaxKind::Pipe => "Pipe",
            SyntaxKind::Dollar => "Dollar",
            SyntaxKind::QuestionMark => "QuestionMark",
            SyntaxKind::Point => "Point",
            SyntaxKind::DoublePoint => "DoublePoint",
            SyntaxKind::DoubleQuote => "DoubleQuote",
            SyntaxKind::SingleQuote => "SingleQuote",
            SyntaxKind::Error => "Error",
            SyntaxKind::ParserInternal => "ParserInternal",
            SyntaxKind::Eof => "Eof",
            SyntaxKind::Tombstone => "Tombstone",
            SyntaxKind::Whitespace => "Whitespace",
            SyntaxKind::Comment => "Comment",
            SyntaxKind::Newline => "Newline",
            SyntaxKind::BareWord => "BareWord",
            SyntaxKind::StringContent => "StringContent",
            SyntaxKind::VarDeclName => "VarDeclName",
            SyntaxKind::FnDeclName => "FnDeclName",
            SyntaxKind::ArgName => "ArgName",
            SyntaxKind::VarArgName => "VarArgName",
            SyntaxKind::LongFlag => "LongFlag",
            SyntaxKind::ShortFlag => "ShortFlag",
            SyntaxKind::Number => "Number",
            SyntaxKind::StrctName => "StrctName",
            SyntaxKind::StrctFieldName => "StrctFieldName",
            SyntaxKind::AbsFileName => "AbsFileName",
            SyntaxKind::RelFileName => "RelFileName",
            SyntaxKind::SourceFile => "SourceFile",
            SyntaxKind::CliLine => "CliLine",
            SyntaxKind::Offset => "Offset",
            SyntaxKind::StrctStmt => "StrctStmt",
            SyntaxKind::RedirStmt => "RedirStmt",
            SyntaxKind::UseStmt => "UseStmt",
            SyntaxKind::PluginUseStmt => "PluginUseStmt",
            SyntaxKind::StrctField => "StrctField",
            SyntaxKind::StrctCtorExpr => "StrctCtorExpr",
            SyntaxKind::StrctFieldCtorStmt => "StrctFieldCtorStmt",
            SyntaxKind::LetStmt => "LetStmt",
            SyntaxKind::FnStmt => "FnStmt",
            SyntaxKind::RetStmt => "RetStmt",
            SyntaxKind::IfElifElseStmt => "IfElifElseStmt",
            SyntaxKind::IfOptElifOptStmt => "IfOptElifOptStmt",
            SyntaxKind::IfElifStmt => "IfElifStmt",
            SyntaxKind::ElseStmt => "ElseStmt",
            SyntaxKind::IfBlock => "IfBlock",
            SyntaxKind::ElifBlock => "ElifBlock",
            SyntaxKind::ElseBlock => "ElseBlock",
            SyntaxKind::ForStmt => "ForStmt",
            SyntaxKind::CmdStmt => "CmdStmt",
            SyntaxKind::PipedCmdsStmt => "PipedCmdsStmt",
            SyntaxKind::BlockStmt => "BlockStmt",
            SyntaxKind::Signature => "Signature",
            SyntaxKind::FlagSignature => "FlagSignature",
            SyntaxKind::ArgSignature => "ArgSignature",
            SyntaxKind::LuType => "LuType",
            SyntaxKind::MathExpr => "MathExpr",
            SyntaxKind::ParentherizedExpr => "ParentherizedExpr",
            SyntaxKind::StringExpr => "StringExpr",
            SyntaxKind::NumberExpr => "NumberExpr",
            SyntaxKind::ValuePathExpr => "ValuePathExpr",
            SyntaxKind::ArrayExpr => "ArrayExpr",
            SyntaxKind::OptionalExpr => "OptionalExpr",
            SyntaxKind::TableExpr => "TableExpr",
            SyntaxKind::BooleanExpr => "BooleanExpr",
            SyntaxKind::ValueExpr => "ValueExpr",
            SyntaxKind::Statement => "Statement",
            SyntaxKind::Condition => "Condition",
            SyntaxKind::OperatorExpr => "OperatorExpr",
            SyntaxKind::LuTypeSpecifier => "LuTypeSpecifier",
            SyntaxKind::CmdArg => "CmdArg",
            SyntaxKind::Flag => "Flag",
            SyntaxKind::RedirToValue => "RedirToValue",
            SyntaxKind::FileName => "FileName",
            SyntaxKind::FileNamePart => "FileNamePart",
            SyntaxKind::IfElifElseStmtPart => "IfElifElseStmtPart",
            #[allow(unreachable_patterns)]
            _ => "", // For the future
        }
    }
}

#[macro_export]
macro_rules! T {
    [impure] => {$crate::SyntaxKind::ImpureKeyword };
    [struct] => {$crate::SyntaxKind::StrctKeyword };
    [Opt] => {$crate::SyntaxKind::OptionalTyKeyword };
    [Some] => {$crate::SyntaxKind::SomeKeyword };
    [None] => {$crate::SyntaxKind::NoneKeyword };
    [as] => {$crate::SyntaxKind::AsKeyword };
    [req] => {$crate::SyntaxKind::ReqKeyword };
    [use] => {$crate::SyntaxKind::UseKeyword };
    [let] => {$crate::SyntaxKind::LetKeyword };
    [fn] => {$crate::SyntaxKind::FnKeyword };
    [for] => {$crate::SyntaxKind::ForKeyword };
    [if] => {$crate::SyntaxKind::IfKeyword };
    [ifopt] => {$crate::SyntaxKind::IfOptKeyword };
    [elif] => {$crate::SyntaxKind::ElifKeyword };
    [elifopt] => {$crate::SyntaxKind::ElifOptKeyword };
    [else] => {$crate::SyntaxKind::ElseKeyword };
    [while] => {$crate::SyntaxKind::WhileKeyword };
    [end] => {$crate::SyntaxKind::EndKeyword };
    [begin] => {$crate::SyntaxKind::BeginKeyword };
    [in] => {$crate::SyntaxKind::InKeyword };
    [any] => {$crate::SyntaxKind::AnyKeyword };
    [path] => {$crate::SyntaxKind::PathKeyword };
    [nil] => {$crate::SyntaxKind::NilKeyword };
    [bool] => {$crate::SyntaxKind::BoolKeyword };
    [true] => {$crate::SyntaxKind::TrueKeyword };
    [false] => {$crate::SyntaxKind::FalseKeyword };
    [num] => {$crate::SyntaxKind::NumberKeyword };
    [str] => {$crate::SyntaxKind::StringKeyword };
    [ret] => {$crate::SyntaxKind::RetKeyword };
    ["("] => {$crate::SyntaxKind::LeftParenthesis };
    [")"] => {$crate::SyntaxKind::RightParenthesis };
    ["{"] => {$crate::SyntaxKind::LeftCurlyBrackets };
    ["}"] => {$crate::SyntaxKind::RightCurlyBrackets };
    ["["] => {$crate::SyntaxKind::LeftRectangularBrackets };
    ["]"] => {$crate::SyntaxKind::RightRectangularBrackets };
    [+] => {$crate::SyntaxKind::PlusSign };
    [-] => {$crate::SyntaxKind::MinusSign };
    [*] => {$crate::SyntaxKind::MultSign };
    [/] => {$crate::SyntaxKind::FilePathSep };
    ["//"] => {$crate::SyntaxKind::DivSign };
    [<] => {$crate::SyntaxKind::LessThanSign };
    [<=] => {$crate::SyntaxKind::LessOrEqualSign };
    [==] => {$crate::SyntaxKind::EqualitySign };
    [!=] => {$crate::SyntaxKind::InequalitySign };
    [>] => {$crate::SyntaxKind::BiggerThanSign };
    [>=] => {$crate::SyntaxKind::BiggerOrEqualSign };
    [>>] => {$crate::SyntaxKind::RightStream };
    [/=] => {$crate::SyntaxKind::DivAssignSign };
    [*=] => {$crate::SyntaxKind::MulAssignSign };
    [+=] => {$crate::SyntaxKind::AddAssignSign };
    [-=] => {$crate::SyntaxKind::MinAssignSign };
    [=] => {$crate::SyntaxKind::AssignSign };
    [|] => {$crate::SyntaxKind::Pipe };
    [$] => {$crate::SyntaxKind::Dollar };
    [?] => {$crate::SyntaxKind::QuestionMark };
    [.] => {$crate::SyntaxKind::Point };
    [:] => {$crate::SyntaxKind::DoublePoint };
    [DoubleQuote] => {$crate::SyntaxKind::DoubleQuote };
    [SingleQuote] => {$crate::SyntaxKind::SingleQuote };
    [...[_a-zA-Z]+[_a-zA-Z0-9]*] => {$crate::SyntaxKind::VarArgName };
    }
