use std::borrow::Cow;

pub use crate::{
    prelude::*,
    keywords::Keyword,
};

#[derive(Eq, PartialEq, Debug)]
pub enum BracketDirection {
    Open,
    Close,
}

#[derive(Eq, PartialEq, Debug)]
pub enum BracketType {
    Round,
    Curly,
    Square,
    Angle,
}

#[derive(Eq, PartialEq, Debug)]
pub enum SentinelType {
    EndOfFile,
}

#[derive(Eq, PartialEq, Debug)]
pub enum TokenKind {
    Comment(String),
    Whitespace,
    Sentinel(SentinelType),
    Bracket(BracketType, BracketDirection),
    Keyword(Keyword),
    Invalid,
}

impl TokenKind {

    pub fn is_invalid(&self) -> bool {
        match self {
            TokenKind::Invalid => true,
            _ => false,
        }
    }

    pub fn short_name<'a>(&'a self) -> Cow<'static, str> {
        use TokenKind::*;
        match self {
            Comment(_) => Cow::Borrowed("comment"),
            Whitespace => Cow::Borrowed("whitepsace"),
            Sentinel(SentinelType::EndOfFile) => Cow::Borrowed("eof"),
            Bracket(_, _) => Cow::Borrowed("bracket"),
            Keyword(keyword) => 
                Cow::Owned(format!("keyword({})", keyword.to_code())),
            Invalid => Cow::Borrowed("invalid"),
        }
    }
}

#[derive(Eq, PartialEq, Debug)]
pub struct Token {
    pub kind: TokenKind,
    pub span: Span,
}

impl Token {
    pub fn create(span_start: usize, span_end: usize, kind: TokenKind) -> Self {
        let span = Span::new(span_start, span_end);
        Self { span, kind }
    }

    pub fn create_keyword(span_start: usize, keyword: Keyword) -> Self {
        let span_end = span_start + keyword.to_code().len();
        Token::create(span_start, span_end, TokenKind::Keyword(keyword))
    }

    pub fn create_invalid(span_start: usize) -> Self {
        Token::create(span_start, span_start, TokenKind::Invalid)
    }
}
