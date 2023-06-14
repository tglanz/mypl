use anyhow::Result;

use crate::span::Span;
use crate::token::{Keyword, Token, TokenKind, DelimDir, DelimType, Literal};

use regex::Regex;

// Tokenization Rules

trait TokenizationRule {
    fn tokenize(&self, source: &SourceReader) -> Option<Token>;
}

struct SimpleTokenizationRule {
    create: fn(source: &SourceReader) -> Option<Token>,
}

impl SimpleTokenizationRule {
    fn boxed(create: fn(source: &SourceReader) -> Option<Token>) -> Box<Self> {
        Box::new(Self { create })
    }
}

struct ExactTokenizationRule {
    exact: String,
    kind: TokenKind,
}

impl ExactTokenizationRule {
    fn boxed(exact: impl AsRef<str>, kind: TokenKind) -> Box<Self> {
        Box::new(Self {
            exact: exact.as_ref().into(),
            kind,
        })
    }
}

impl TokenizationRule for ExactTokenizationRule {
    fn tokenize(&self, source: &SourceReader) -> Option<Token> {
        if !source.as_ref().starts_with(&self.exact) {
            None
        } else {
            Some(Token {
                kind: self.kind.clone(),
                span: source.make_span(self.exact.len()),
            })
        }
    }
}

impl TokenizationRule for SimpleTokenizationRule {
    fn tokenize(&self, source: &SourceReader) -> Option<Token> {
        (self.create)(source)
    }
}

struct RegexTokenizationRule {
    regex: Regex,
    create: fn(regex::Match) -> TokenKind,
}

impl RegexTokenizationRule {
    fn boxed(pattern: &str, create: fn(regex::Match) -> TokenKind) -> Result<Box<Self>> {
        let regex = Regex::new(pattern)?;
        Ok(Box::new(Self { regex, create }))
    }
}

impl TokenizationRule for RegexTokenizationRule {
    fn tokenize(&self, source: &SourceReader) -> Option<Token> {
        self.regex
            .captures(source.as_ref())
            .and_then(|captures| captures.get(0))
            .map(|capture| Token {
                kind: (self.create)(capture),
                span: Span::new(source.get_position(), source.get_position() + capture.len()),
            })
    }
}

fn create_tokenization_rules() -> Result<Vec<Box<dyn TokenizationRule>>> {
    Ok(vec![
        // Eof
        SimpleTokenizationRule::boxed(|source| {
            source.is_eof().then(|| Token {
                kind: TokenKind::Eof,
                span: source.make_span(1),
            })
        }),
        // Comments
        RegexTokenizationRule::boxed("^//.*", |capture| {
            TokenKind::Comment(capture.as_str().into())
        })?,

        // Keywords
        ExactTokenizationRule::boxed("const", TokenKind::Keyword(Keyword::Const)),
        ExactTokenizationRule::boxed("var", TokenKind::Keyword(Keyword::Var)),
        ExactTokenizationRule::boxed("record", TokenKind::Keyword(Keyword::Record)),
        ExactTokenizationRule::boxed("union", TokenKind::Keyword(Keyword::Union)),
        ExactTokenizationRule::boxed("impl", TokenKind::Keyword(Keyword::Impl)),
        ExactTokenizationRule::boxed("trait", TokenKind::Keyword(Keyword::Trait)),
        ExactTokenizationRule::boxed("mod", TokenKind::Keyword(Keyword::Mod)),
        ExactTokenizationRule::boxed("if", TokenKind::Keyword(Keyword::If)),
        ExactTokenizationRule::boxed("else", TokenKind::Keyword(Keyword::Else)),
        ExactTokenizationRule::boxed("for", TokenKind::Keyword(Keyword::For)),
        ExactTokenizationRule::boxed("in", TokenKind::Keyword(Keyword::In)),
        ExactTokenizationRule::boxed("match", TokenKind::Keyword(Keyword::Match)),
        ExactTokenizationRule::boxed("return", TokenKind::Keyword(Keyword::Return)),
        ExactTokenizationRule::boxed("u32", TokenKind::Keyword(Keyword::U32)),
        ExactTokenizationRule::boxed("u16", TokenKind::Keyword(Keyword::U16)),
        ExactTokenizationRule::boxed("u8", TokenKind::Keyword(Keyword::U8)),
        ExactTokenizationRule::boxed("i32", TokenKind::Keyword(Keyword::I32)),
        ExactTokenizationRule::boxed("i16", TokenKind::Keyword(Keyword::I16)),
        ExactTokenizationRule::boxed("i8", TokenKind::Keyword(Keyword::I8)),
        ExactTokenizationRule::boxed("f32", TokenKind::Keyword(Keyword::F32)),
        ExactTokenizationRule::boxed("f16", TokenKind::Keyword(Keyword::F16)),
        ExactTokenizationRule::boxed("print", TokenKind::Keyword(Keyword::Print)),
        
        // Double Character
        ExactTokenizationRule::boxed("==", TokenKind::EqEq),
        ExactTokenizationRule::boxed("<=", TokenKind::Le),
        ExactTokenizationRule::boxed(">=", TokenKind::Ge),
        ExactTokenizationRule::boxed("!=", TokenKind::Ne),
        ExactTokenizationRule::boxed("..", TokenKind::DotDot),
        ExactTokenizationRule::boxed("&&", TokenKind::AndAnd),
        ExactTokenizationRule::boxed("||", TokenKind::OrOr),
        ExactTokenizationRule::boxed(">>", TokenKind::GtGt),
        ExactTokenizationRule::boxed("<<", TokenKind::LtLt),
        ExactTokenizationRule::boxed("+=", TokenKind::PlusEq),
        ExactTokenizationRule::boxed("-=", TokenKind::MinusEq),
        ExactTokenizationRule::boxed("*=", TokenKind::StarEq),
        ExactTokenizationRule::boxed("/=", TokenKind::SlashEq),
        ExactTokenizationRule::boxed("&=", TokenKind::AndEq),
        ExactTokenizationRule::boxed("|=", TokenKind::OrEq),

        // Single Characters
        ExactTokenizationRule::boxed("=", TokenKind::Eq),
        ExactTokenizationRule::boxed("<", TokenKind::Lt),
        ExactTokenizationRule::boxed(">", TokenKind::Gt),
        ExactTokenizationRule::boxed("!", TokenKind::Not),
        ExactTokenizationRule::boxed(".", TokenKind::Dot),
        ExactTokenizationRule::boxed(",", TokenKind::Comma),
        ExactTokenizationRule::boxed(":", TokenKind::Colon),
        ExactTokenizationRule::boxed(";", TokenKind::SemiColon),
        ExactTokenizationRule::boxed("+", TokenKind::Plus),
        ExactTokenizationRule::boxed("-", TokenKind::Minus),
        ExactTokenizationRule::boxed("*", TokenKind::Star),
        ExactTokenizationRule::boxed("/", TokenKind::Slash),
        ExactTokenizationRule::boxed("%", TokenKind::Percent),
        ExactTokenizationRule::boxed("^", TokenKind::Caret),
        ExactTokenizationRule::boxed("&", TokenKind::And),
        ExactTokenizationRule::boxed("|", TokenKind::Or),
        ExactTokenizationRule::boxed("(", TokenKind::Delim(DelimDir::Open, DelimType::Paren)),
        ExactTokenizationRule::boxed("{", TokenKind::Delim(DelimDir::Open, DelimType::Brace)),
        ExactTokenizationRule::boxed("[", TokenKind::Delim(DelimDir::Open, DelimType::Brack)),
        ExactTokenizationRule::boxed(")", TokenKind::Delim(DelimDir::Close, DelimType::Paren)),
        ExactTokenizationRule::boxed("}", TokenKind::Delim(DelimDir::Close, DelimType::Brace)),
        ExactTokenizationRule::boxed("]", TokenKind::Delim(DelimDir::Close, DelimType::Brack)),

        // String Literal
        RegexTokenizationRule::boxed("^\".*\"", |capture| {
            TokenKind::Literal(Literal::String(capture.as_str().to_string()))
        })?,

        // Float Literal 
        // Notice that we only tokenize positive floats - is this good?
        RegexTokenizationRule::boxed("^\\d+\\.\\d+|^\\d+\\.|\\.\\d+", |capture| {
            TokenKind::Literal(Literal::Float(capture.as_str().parse::<f64>()
                                              .expect("Couldn't parse f64 after regex match")))
        })?,

        // Integer Literal
        // Notice that we only tokenize positive integers - is this good?
        RegexTokenizationRule::boxed("^\\d+", |capture| {
            TokenKind::Literal(Literal::Integer(capture.as_str().parse::<i128>()
                                                .expect("Couldn't parse u64 after regex match")))
        })?,

        // Bool Literal
        ExactTokenizationRule::boxed("true", TokenKind::Literal(Literal::Bool(true))),
        ExactTokenizationRule::boxed("false", TokenKind::Literal(Literal::Bool(false))),

        // Identifier
        RegexTokenizationRule::boxed("^[a-zA-Z]\\w*", |capture| {
            TokenKind::Identifier(capture.as_str().to_string())
        })?,
    ])
}

// SourceReader

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

    pub fn make_span(&self, size: usize) -> Span {
        Span::new(self.position, self.position + size)
    }

    pub fn advance(&mut self, size: usize) {
        self.position += size
    }
}

impl<'a> AsRef<str> for SourceReader<'a> {
    fn as_ref(&self) -> &str {
        &self.source[self.position..]
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
    rules: Vec<Box<dyn TokenizationRule>>,
    whitespace_regex: Regex,
}

impl<'a> Tokenizer<'a> {
    pub fn new(source: &'a str) -> Self {
        Self {
            source: source.into(),

            rules: create_tokenization_rules()
                .expect("failed creating rules - happens only cause of code issues, fix it"),

            // This won't panic, we know it compiles.
            whitespace_regex: Regex::new(r"^[\t\n\r ]+").unwrap(),
        }
    }

    pub fn next_token(&mut self) -> Option<Token> {
        if self.source.did_pass_eof() {
            return None;
        }

        self.advance_whitespace();

        for rule in &self.rules {
            if let Some(token) = rule.tokenize(&self.source) {
                self.source.advance(token.span.get_size());
                return Some(token);
            }
        }

        None
    }

    fn advance_whitespace(&mut self) {
        if let Some(c) = self.whitespace_regex.captures(self.source.as_ref()) {
            self.source.advance(c.get(0).unwrap().len());
        }
    }
}
