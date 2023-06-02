#![allow(dead_code)]

extern crate anyhow;
extern crate regex;

use anyhow::Result;
use regex::Regex;

trait FirstMatch {
    fn first_match<'a>(&self, content: &'a str) -> Option<regex::Match<'a>>;
}

impl FirstMatch for Regex {
    fn first_match<'a>(&self, content: &'a str) -> Option<regex::Match<'a>> {
        self.captures(content).and_then(|captures| captures.get(0))
    }
}

#[derive(Debug)]
pub enum TokenKind {
    Comment(String),
    Unknown(String),
}

impl TokenKind {
    fn get_size(&self) -> usize {
        use TokenKind::*;

        match self {
            Unknown(lexeme) | Comment(lexeme) => lexeme.len(),
        }
    }
}

struct Token {
    kind: TokenKind,
}

trait TokenizationRule {
    fn tokenize(&self, content: &str) -> Option<TokenKind>;
}

struct TokenizeComment;

impl TokenizationRule for TokenizeComment {
    fn tokenize(&self, content: &str) -> Option<TokenKind> {
        Regex::new("^//.*")
            .unwrap()
            .first_match(content)
            .map(|m| TokenKind::Comment(m.as_str().into()))
    }
}

struct RegexTokenizationSettings {
    regex: Regex,
    create: fn(regex::Match) -> Option<TokenKind>,
}

impl TokenizationRule for RegexTokenizationSettings {
    fn tokenize(&self, content: &str) -> Option<TokenKind> {
        self.regex
            .captures(content)
            .and_then(|captures| captures.get(0))
            .and_then(|capture| (self.create)(capture))
    }
}

fn main() -> Result<()> {
    let comment = "// some comment";
    // let comment_token = TokenizeComment::tokenize(comment);

    let rules: Vec<Box<dyn TokenizationRule>> = vec![Box::new(RegexTokenizationSettings {
        regex: Regex::new("^//.*").unwrap(),
        create: |capture: regex::Match| Some(TokenKind::Comment(capture.as_str().into())),
    })];

    for rule in rules {
        if let Some(token) = rule.tokenize(comment) {
            println!("token: {:#?}", token);
        }
    }

    Ok(())
}
