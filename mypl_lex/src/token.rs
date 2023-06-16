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
    Println,
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

impl TokenKind {
    pub fn unwrap_identifier(&self) -> &String {
        match self {
            TokenKind::Identifier(identifier) => identifier,
            _ => panic!("attempted to unwrap_identifier but is {:?}", self),
        }
    }
    
    pub fn unwrap_literal(&self) -> &Literal {
        match self {
            TokenKind::Literal(literal) => literal,
            _ => panic!("attempted to unwrap_literal but is {:?}", self),
        }
    }
}

#[derive(Clone, PartialEq, Debug)]
pub struct Token {
    pub kind: TokenKind,
    pub span: Span,
}
