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

impl Keyword {
    pub fn from_code(code: impl AsRef<str>) -> Option<Keyword> {
        match code.as_ref() {
            "cst" => Some(Keyword::Const),
            "var" => Some(Keyword::Var),
            "record" => Some(Keyword::Record),
            "union" => Some(Keyword::Union),
            "impl" => Some(Keyword::Impl),
            "trait" => Some(Keyword::Trait),
            "mod" => Some(Keyword::Mod),
            "if" => Some(Keyword::If),
            "else" => Some(Keyword::Else),
            "for" => Some(Keyword::For),
            "in" => Some(Keyword::In),
            "match" => Some(Keyword::Match),
            "return" => Some(Keyword::Return),
            "u32" => Some(Keyword::U32),
            "u16" => Some(Keyword::U16),
            "u8" => Some(Keyword::U8),
            "i32" => Some(Keyword::I32),
            "i16" => Some(Keyword::I16),
            "i8" => Some(Keyword::I8),
            "f32" => Some(Keyword::F32),
            "f16" => Some(Keyword::F16),
            _ => None,
        }
    }
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
