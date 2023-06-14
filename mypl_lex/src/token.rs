use crate::span::Span;

#[derive(Clone, PartialEq, Debug)]
pub enum Literal {
    String(String),
    Bool(bool),
    Integer(i128),
    Float(f64),
}

#[derive(Clone, PartialEq, Debug)]
pub enum DelimDir {
    Open,
    Close,
}

#[derive(Clone, PartialEq, Debug)]
pub enum DelimType {
    Paren,
    Brace,
    Brack,
}

#[derive(Clone, PartialEq, Debug)]
pub enum Keyword {
    Const,
    Var,
    Record,
    Union,
    Impl,
    Trait,
    Mod,
    If,
    Else,
    For,
    In,
    Match,
    Return,

    U32,
    U16,
    U8,

    I32,
    I16,
    I8,

    F32,
    F16,

    Print,
}

#[derive(Clone, PartialEq, Debug)]
pub enum TokenKind {
    // =
    Eq,
    // <
    Lt,
    // <<
    LtLt,
    // <=
    Le,
    // ==
    EqEq,
    // !=
    Ne,
    // >=
    Ge,
    // >
    Gt,
    // >
    GtGt,
    // &
    And,
    // &=
    AndEq,
    // &&
    AndAnd,
    // |
    Or,
    // |=
    OrEq,
    // ||
    OrOr,
    // !
    Not,

    // +
    Plus,
    // +=
    PlusEq,
    // -
    Minus,
    // -
    MinusEq,
    // *
    Star,
    // *=
    StarEq,
    // /
    Slash,
    // /=
    SlashEq,
    // %
    Percent,
    // ^
    Caret,

    // .
    Dot,
    // ..
    DotDot,
    // ,
    Comma,
    // :
    Colon,
    // ;
    SemiColon,

    Comment(String),

    Keyword(Keyword),
    Literal(Literal),
    Identifier(String),

    Delim(DelimDir, DelimType),

    Eof,

    Unknown(String),
    Undefined,
}

#[derive(Clone, PartialEq, Debug)]
pub struct Token {
    pub kind: TokenKind,
    pub span: Span,
}
