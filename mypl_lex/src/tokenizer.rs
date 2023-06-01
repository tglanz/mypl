use crate::token::{Token, TokenKind, Keyword, BinOp, Delimiter};
use crate::span::Span;

use regex::Regex;

// A idea I had is to have a trait of a "Rule".
// Such rules act like Regexes - they take a string and return Option<usize>.
// This will enable us to create functions instead of Regexes if needed.
// I had trouble of creating an efficient abstraction for it due to need of boxing and dynamic
// dispatch.
//
// trait Rule {
//     fn try_match<'a>(&self, testee: &'a str) -> Option<usize>;
// }
// 
// impl Rule for Regex {
//     fn try_match<'a>(&self, testee: &'a str) -> Option<usize> {
//         self.captures(testee)
//             .map(|captures| captures.len())
//     }
// }

struct Patterns {
    whitespace: Regex,
    comment: Regex,
    keyword: Regex,
}

impl Default for Patterns {
    fn default() -> Self {
        Self {
            whitespace: Regex::new(r"^[\t\n\r ]+").unwrap(),
            comment: Regex::new("^//.*").unwrap(),
            keyword: Regex::new("^(cst|var|record|union|impl|trait|mod|if|else|for|in|match|return|u32|u16|u8|i32|i16|i8|f32|f6)").unwrap(),
        }
    }
}

pub struct SourceReader<'a> {
    position: usize,
    source: &'a str,
}

impl<'a> SourceReader<'a> {

    pub fn is_eof(&self) -> bool {
        self.position == self.source.len()
    }

    pub fn did_pass_eof(&self) -> bool {
        self.position > self.source.len()
    }

    pub fn get_position(&self) -> usize {
        self.position 
    }

    pub fn advance(&mut self, size: usize) {
        self.position += size
    }

    fn substring(&self, size: usize) -> &'a str {
        &self.source[self.position..self.position+size]
    }

    fn into_substring(&self, size: usize) -> String {
        self.substring(size).into()
    }
}

impl<'a> AsRef<str> for SourceReader<'a> {
    fn as_ref(&self) -> &str {
        return &self.source[self.position..]
    }
}

impl<'a> From<&'a str> for SourceReader<'a> {
    fn from(source: &'a str) -> Self {
        Self {
            position: 0,
            source,
        }
    }
}

pub struct Tokenizer<'a> {
    source: SourceReader<'a>,
    patterns: Patterns,
}

impl<'a> Tokenizer<'a> {

    pub fn new(source: &'a str) -> Self {
        Self {
            source: source.into(),
            patterns: Default::default(),
        }
    }
 
    fn make_span(&self, size: usize) -> Span {
        Span::new(self.source.get_position(), self.source.get_position() + size)
    }

    pub fn next_token(&mut self) -> Option<Token> {

        if self.source.did_pass_eof() {
            return None;
        }

        self.advance_whitespace();

        // chain attempts of tokenizations.
        // the first tokenization success breaks the chain.
        None
            .or_else(|| self.tokenize_eof())
            .or_else(|| self.tokenize_comment())
            .or_else(|| self.tokenize_keyword())
            .or_else(|| self.tokenize_double_char())
            .or_else(|| self.tokenize_single_char())
            .or_else(|| Some(Token {
                kind: TokenKind::Unknown(self.source.into_substring(1)),
                span: self.make_span(1),
            }))
            .map(|token| {
                self.source.advance(token.span.get_size());
                token
            })
    }


    fn advance_whitespace(&mut self) {
        if let Some(c) = self.patterns.whitespace.captures(self.source.as_ref()) {
            self.source.advance(c.get(0).unwrap().len());
        }
    }

    fn tokenize_eof(&mut self) -> Option<Token> {
        if !self.source.is_eof() { 
            None
        } else {
            Some(Token {
                kind: TokenKind::Eof,
                span: self.make_span(1),
            })
        }
    }

    fn tokenize_comment(&mut self) -> Option<Token> {
        self.patterns.comment
            .captures(self.source.as_ref())
            .map(|c| Token {
                kind: TokenKind::Comment(self.source.into_substring(c.get(0).unwrap().len())),
                span: self.make_span(c.get(0).unwrap().len()),
            })
    }

    fn tokenize_single_char(&mut self) -> Option<Token> {

        let kind = match self.source.substring(1) {
            "=" => TokenKind::Eq,
            "<" => TokenKind::Lt,
            ">" => TokenKind::Gt,
            "!" => TokenKind::Not,
            "." => TokenKind::Dot,
            "," => TokenKind::Comma,
            ":" => TokenKind::Colon,
            ";" => TokenKind::SemiColon,
            "+" => TokenKind::BinOp(BinOp::Plus),
            "-" => TokenKind::BinOp(BinOp::Minus),
            "*" => TokenKind::BinOp(BinOp::Star),
            "/" => TokenKind::BinOp(BinOp::Slash),
            "%" => TokenKind::BinOp(BinOp::Percent),
            "^" => TokenKind::BinOp(BinOp::Caret),
            "&" => TokenKind::BinOp(BinOp::And),
            "|" => TokenKind::BinOp(BinOp::Or),
            "(" => TokenKind::OpenDelim(Delimiter::Paren),
            "{" => TokenKind::OpenDelim(Delimiter::Brace),
            "[" => TokenKind::OpenDelim(Delimiter::Brack),
            ")" => TokenKind::CloseDelim(Delimiter::Paren),
            "}" => TokenKind::CloseDelim(Delimiter::Brace),
            "]" => TokenKind::CloseDelim(Delimiter::Brack),

            _ => TokenKind::Undefined,
        };

        if kind == TokenKind::Undefined {
            return None;
        }

        let span = self.make_span(1);
        Some(Token { kind, span })
    }


    fn tokenize_double_char(&mut self) -> Option<Token> {

        let kind = match self.source.substring(2) {
            "==" => TokenKind::EqEq,
            "<=" => TokenKind::Le,
            ">=" => TokenKind::Ge,
            "!=" => TokenKind::Ne,
            ".." => TokenKind::DotDot,
            "&&" => TokenKind::AndAnd,
            "||" => TokenKind::OrOr,
            ">>" => TokenKind::BinOp(BinOp::Shr),
            "<<" => TokenKind::BinOp(BinOp::Shl),

            // TODO: compose BinOpEq
            "+=" => TokenKind::BinOpEq(BinOp::Plus),
            "-=" => TokenKind::BinOpEq(BinOp::Minus),
            "*=" => TokenKind::BinOpEq(BinOp::Star),
            "/=" => TokenKind::BinOpEq(BinOp::Slash),
            "%=" => TokenKind::BinOpEq(BinOp::Percent),
            "^=" => TokenKind::BinOpEq(BinOp::Caret),
            "&=" => TokenKind::BinOpEq(BinOp::And),
            "|=" => TokenKind::BinOpEq(BinOp::Or),

            _ => TokenKind::Undefined,
        };

        if kind == TokenKind::Undefined {
            return None;
        }

        let span = self.make_span(2);
        Some(Token { kind, span })
    }

    fn tokenize_keyword(&mut self) -> Option<Token> {
        self.patterns.keyword
            .captures(self.source.as_ref())
            .map(|c| {
                let len = c.get(0).unwrap().len();
                let code = self.source.substring(len);
                Token {
                    // If this unwrap panics, it doesn't mean syntax error.
                    // It means we set up the tokenization wrong.
                    kind: TokenKind::Keyword(Keyword::from_code(code).expect(code)),
                    span: self.make_span(len),
                }
            })
    }
}
