pub use crate::prelude::*;

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
    Invalid,
}

impl TokenKind {
    pub fn short_name(&self) -> &'static str {
        use TokenKind::*;
        match self {
            Comment(_) => "comment",
            Whitespace => "whitepsace",
            Sentinel(SentinelType::EndOfFile) => "eof",
            Bracket(_, _) => "bracket",
            Invalid => "invalid",
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

    pub fn create_invalid(span_start: usize) -> Self {
        Token::create(span_start, span_start, TokenKind::Invalid)
    }
}
