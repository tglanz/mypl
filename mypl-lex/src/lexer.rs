use crate::prelude::*;
use crate::keywords;

use log;

pub struct Lexer<'a> {
    //source: &'a str,
    source: &'a str,
    peeker: SourcePeeker<'a>,
    patterns: Patterns,
    pos: usize,
    all_keywords: Vec<Keyword>,
}

impl<'a> Lexer<'a> {
    pub fn new(source: &'a str) -> Self {
        Self {
            source,
            peeker: SourcePeeker::new(source),
            patterns: Patterns::default(),
            pos: 0,
            all_keywords: keywords::create_all()
        }
    }

    pub fn tokenize(&mut self) -> Vec<Token> {
        let mut ans = Vec::new();

        log::debug!("staring to tokenize source: \n{}", self.source);

        loop {
            log::debug!("");
            log::debug!("==== next token ====");
            let x = self.peeker.until(self.pos, '\n');
            log::debug!("rest of line: {:?}", x);

            let mut token = self.next_token();

            log::debug!("found: {}", token.kind.short_name());
            log::debug!(
                "span: {:?}, actual source: {:?}",
                token.span,
                token.span.slice_string(self.source)
            );
            token = self.bump_token(token);

            ans.push(token);

            match ans.last().unwrap().kind {
                TokenKind::Invalid | TokenKind::Sentinel(SentinelType::EndOfFile) => break,
                _ => {}
            }
        }

        ans
    }

    fn remaining_source(&self) -> &'a str {
        &self.source[self.pos..]
    }

    fn next_token(&mut self) -> Token {
        self.tokenize_eof()
            .or_else(|| self.tokenize_whitespace())
            .or_else(|| self.tokenize_comment())
            .or_else(|| self.tokenize_keyword())
            .unwrap_or_else(|| Token::create_invalid(self.pos))
    }

    fn tokenize_eof(&mut self) -> Option<Token> {
        if self.pos < self.source.len() {
            None
        } else {
            Some(Token::create(
                self.pos,
                self.pos,
                TokenKind::Sentinel(SentinelType::EndOfFile),
            ))
        }
    }

    fn tokenize_whitespace(&mut self) -> Option<Token> {
        log::debug!("tokenize_whitespace");
        self.patterns
            .whitespace
            .captures(self.remaining_source())
            .map(|c| {
                let start = self.pos;
                let end = start + c.get(0).unwrap().range().len();
                log::debug!("whitespace detected start={}, end={}", start, end);
                Token::create(start, end, TokenKind::Whitespace)
            })
    }

    fn tokenize_comment(&mut self) -> Option<Token> {
        self.patterns
            .comment
            .captures(self.remaining_source())
            .map(|c| {
                let start = self.pos;
                let end = start + c.get(0).unwrap().range().len();
                log::debug!("comment detected start={}, end={}", start, end);
                Token::create(
                    start,
                    end,
                    TokenKind::Comment(c.get(1).unwrap().as_str().trim().to_owned()),
                )
            })
    }

    fn tokenize_keyword(&mut self) -> Option<Token> {
        for keyword in self.all_keywords.iter() {
            let code = keyword.to_code();
            log::debug!("checking if keyword: {}", code);
            if self.peeker.starts_with(self.pos, code) {
                return Some(Token::create_keyword(self.pos, *keyword));
            }
        }

        None
    }

    fn bump_token(&mut self, token: Token) -> Token {
        self.pos += token.span.length();
        token
    }
}
