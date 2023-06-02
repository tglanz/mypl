use crate::span::Span;

#[derive(Clone, PartialEq, Debug)]
pub enum BinOp {
    // +
    Plus,
    // -
    Minus,
    // *
    Star,
    // /
    Slash,
    // %
    Percent,
    // ^
    Caret,
    // &
    And,
    // |
    Or,
    // <<
    Shl,
    // >>
    Shr,
}

#[derive(Clone, PartialEq, Debug)]
pub enum Literal {
    Identifier(String),
    String(String),
    Number(String),
}

#[derive(Clone, PartialEq, Debug)]
pub enum Delimiter {
    // ()
    Paren,
    // {}
    Brace,
    // []
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
}

#[derive(Clone, PartialEq, Debug)]
pub enum TokenKind {
    // =
    Eq,
    // <
    Lt,
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
    // &&
    AndAnd,
    // ||
    OrOr,
    // !
    Not,

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

    BinOp(BinOp),
    BinOpEq(BinOp),

    OpenDelim(Delimiter),
    CloseDelim(Delimiter),

    Literal(Literal),

    Eof,

    Unknown(String),
    Undefined,
}

#[derive(Clone, PartialEq, Debug)]
pub struct Token {
    pub kind: TokenKind,
    pub span: Span,
}
