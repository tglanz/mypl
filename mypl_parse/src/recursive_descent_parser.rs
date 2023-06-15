use mypl_ast::prelude::*;
use mypl_lex::prelude::*;

use anyhow::Result;

use crate::{
    parse::{ParseError, Parser},
    token_kind_predicates::TokenKindExtensions,
};

use ParseError::*;

trait OptionExtensions {
    fn when_some<R, F: FnOnce() -> R>(self, f: F) -> Self;
}

impl<T> OptionExtensions for Option<T> {
    fn when_some<R, F: FnOnce() -> R>(self, f: F) -> Self {
        if self.is_some() {
            f();
        }

        self
    }
}

pub struct RecursiveDescentParser<'a> {
    position: usize,

    // TODO: replace this with an iterator
    tokens: &'a Vec<Token>,
}

impl<'a> RecursiveDescentParser<'a> {
    pub fn new(tokens: &'a Vec<Token>) -> Self {
        Self {
            position: 0,
            tokens,
        }
    }

    fn advance(&mut self) -> Option<&Token> {
        self.position += 1;
        return self.previous_token();
    }

    fn match_predicate<P: (Fn(&TokenKind) -> bool)>(&mut self, predicate: P) -> Option<Token> {
        self
            .token()
            .filter(|t| predicate(&t.kind))
            .cloned()
            .when_some(|| self.advance())
    }

    fn match_variant(&mut self, kind: &TokenKind) -> Option<Token> {
        self.match_predicate(|k| std::mem::discriminant(kind) == std::mem::discriminant(k))
    }

    fn match_keyword(&mut self, keyword: &Keyword) -> Option<Token> {
        self.match_predicate(|k| match k {
            TokenKind::Keyword(kw) => kw == keyword,
            _ => false,
        })
    }

    fn match_identifier(&mut self) -> Option<String> {
        self.token()
            .and_then(|t| match &t.kind {
                TokenKind::Identifier(identifier) => Some(identifier),
                _ => None,
            })
            .cloned()
            .when_some(|| self.advance())
    }

    fn match_literal(&mut self) -> Option<Literal> {
        self.token()
            .and_then(|t| match &t.kind {
                TokenKind::Literal(literal) => Some(literal),
                _ => None
            })
            .cloned()
            .when_some(|| self.advance())
    }

    fn match_binary_op(&mut self, ops: &[BinOp]) -> Option<BinOp> {
        self
            .token()
            .and_then(|t| t.kind.to_binary_op())
            .filter(|op| ops.contains(op))
            .when_some(|| self.advance())
    }

    fn match_unary_op(&mut self, ops: &[UnOp]) -> Option<UnOp> {
        self
            .token()
            .and_then(|t| t.kind.to_unary_op())
            .filter(|op| ops.contains(op))
            .when_some(|| self.advance())
    }

    fn token(&self) -> Option<&Token> {
        self.tokens.get(self.position)
    }

    fn previous_token(&mut self) -> Option<&Token> {
        self.tokens.get(self.position - 1)
    }

    // Grammar
    
    fn program(&mut self) -> Result<Vec<Stmt>, ParseError> {
        let mut statements = Vec::new();
        while let None = self.match_variant(&TokenKind::Eof) {
            statements.push(self.decl()?);
        }

        Ok(statements)
    }

    fn decl(&mut self) -> Result<Stmt, ParseError> {
        if self.match_keyword(&Keyword::Const).is_some() {
            Ok(self.const_decl()?)
        } else if self.match_keyword(&Keyword::Var).is_some() {
            Ok(self.var_decl()?)
        } else {
            Ok(self.statement()?)
        }
    }

    fn const_decl(&mut self) -> Result<Stmt, ParseError> {
        let identifier = self.match_identifier()
            .ok_or_else(|| ExpectedToken("identifier".to_string(), "const_decl".to_string()))?;
        
        let _ = self.match_variant(&TokenKind::Eq)
            .ok_or_else(|| ExpectedToken("=".to_string(), "const_decl".to_string()));

        let expr = self.expression()
            .map_err(|_| ParseError::Default("expected expression in const_decl".to_string()))?;

        let _ = self.match_variant(&TokenKind::SemiColon)
            .ok_or_else(|| ExpectedToken(";".to_string(), "const_decl".to_string()));

        Ok(Stmt {
            kind: StmtKind::Decl(Box::new(Decl {
                kind: DeclKind::Const(identifier, Box::new(expr))
            }))
        })
    }

    fn var_decl(&mut self) -> Result<Stmt, ParseError> {
        let identifier = self.match_identifier()
            .ok_or_else(|| ExpectedToken("identifier".to_string(), "decl".to_string()))?;
        
        self.match_variant(&TokenKind::Eq)
            .ok_or_else(|| ExpectedToken("=".to_string(), "decl".to_string()))?;

        let expr = self.expression()
            .map_err(|_| ParseError::Default("expected expression at var_decl".to_string()))?;

        self.match_variant(&TokenKind::SemiColon)
            .ok_or_else(|| ExpectedToken(";".to_string(), "decl".to_string()))?;

        Ok(Stmt {
            kind: StmtKind::Decl(Box::new(Decl {
                kind: DeclKind::Var(identifier, Box::new(expr))
            }))
        })
    }

    fn statement(&mut self) -> Result<Stmt, ParseError> {
        if self.match_keyword(&Keyword::Print).is_some() {
            self.print_statement()
        } else {
            self.expression_statement()
        }
    }

    fn print_statement(&mut self) -> Result<Stmt, ParseError> {
        let expr = self.expression()?;
        self.match_variant(&TokenKind::SemiColon)
            .ok_or_else(|| ExpectedToken(";".to_string(), "print_statement".to_string()))?;
        Ok(Stmt {
            kind: StmtKind::Print(Box::new(expr)),
        })
    }

    fn expression_statement(&mut self) -> Result<Stmt, ParseError> {
        let expr = self.expression()?;

        // TODO: type the error 
        let _ = self.match_variant(&TokenKind::SemiColon)
            .ok_or_else(|| ParseError::Default("exprStmt: Expected token \";\"".to_string()));

        Ok(Stmt {
            kind: StmtKind::Expr(Box::new(expr))
        })
    }

    fn expression(&mut self) -> Result<Expr, ParseError> {
        return self.equality();
    }

    fn equality(&mut self) -> Result<Expr, ParseError> {
        let mut lhs = self.comparison()?;

        while let Some(op) = self.match_binary_op(&[BinOp::Eq, BinOp::Ne]) {
            let rhs = self.expression()?;
            lhs = Expr {
                kind: ExprKind::Binary(op, Box::new(lhs), Box::new(rhs))
            }
        }

        Ok(lhs)
    }

    fn comparison(&mut self) -> Result<Expr, ParseError> {
        let mut lhs = self.term()?;

        while let Some(op) = self.match_binary_op(&[BinOp::Gt, BinOp::Ge, BinOp::Lt, BinOp::Le]) {
            let rhs = self.term()?;
            lhs = Expr {
                kind: ExprKind::Binary(op, Box::new(lhs), Box::new(rhs))
            }
        }

        Ok(lhs)
    }

    fn term(&mut self) -> Result<Expr, ParseError> {
        let mut lhs = self.factor()?;

        while let Some(op) = self.match_binary_op(&[BinOp::Add, BinOp::Sub]) {
            let rhs = self.factor()?;
            lhs = Expr {
                kind: ExprKind::Binary(op, Box::new(lhs), Box::new(rhs))
            }
        }

        Ok(lhs)
    }

    fn factor(&mut self) -> Result<Expr, ParseError> {
        // TODO: figure unwraps
        let mut lhs = self.unary()?;

        while let Some(op) = self.match_binary_op(&[BinOp::Mul, BinOp::Div]) {
            let rhs = self.unary()?;
            lhs = Expr {
                kind: ExprKind::Binary(op, Box::new(lhs), Box::new(rhs))
            }
        }

        Ok(lhs)
    }

    fn unary(&mut self) -> Result<Expr, ParseError> {
        if let Some(op) = self.match_unary_op(&[UnOp::Not, UnOp::Neg]) {
            let expr = self.unary()?;
            return Ok(Expr {
                kind: ExprKind::Unary(op, Box::new(expr)),
            });
        }

        self.primary()
    }

    fn primary(&mut self) -> Result<Expr, ParseError> {
        // TODO: This is a very shitty code. Refactor.
        if let Some(literal) = self.match_literal() {
            Ok(Expr {
                kind: ExprKind::Literal(literal)
            })
        } else if self.match_predicate(|k| match k {
            TokenKind::Delim(DelimDir::Open, DelimType::Paren) => true,
            _ => false,
        }).is_some() {
            let expr = self
                .expression()
                .expect("SyntaxError: expected expression - TODO: better error handling");

            let _ = self
                .match_predicate(|k| match k {
                    TokenKind::Delim(DelimDir::Close, DelimType::Paren) => true,
                    _ => false,
                })
                .expect("SyntaxError: expected right paren - TODO: better error handling");

            Ok(expr)
        } else if let Some(ident) = self.match_identifier() {
            Ok(Expr {
                kind: ExprKind::Variable(ident.to_string())
            })
        } else {
            Err(ParseError::Default("primary expression exhausted".to_string()))
        }
    }
}

impl<'a> Parser for RecursiveDescentParser<'a> {
    fn parse(&mut self) -> Result<Vec<Stmt>, ParseError> {
        self.program()
    }
}

#[cfg(test)]
mod tests {
    // use super::*;
    // use mypl_lex::prelude::*;

    // #[test]
    // fn parse_literal_string() {
    //     let expected = Expr {
    //         kind: ExprKind::Literal(Literal::String("some-string".to_string())),
    //     };

    //     let tokens = vec![Token {
    //         span: Span::new(0, 0),
    //         kind: TokenKind::Literal(Literal::String("some-string".to_string())),
    //     }];

    //     let mut parser = RecursiveDescentParser::new(&tokens);

    //     let actual = parser.parse().expect("oops");
    //     assert_eq!(expected, actual);
    // }

    // #[test]
    // fn parse_unary() {
    //     let expected = Expr {
    //         kind: ExprKind::Unary(
    //             UnOp::Not,
    //             Box::new(Expr {
    //                 kind: ExprKind::Literal(Literal::Bool(true)),
    //             }),
    //         ),
    //     };

    //     let tokens = vec![
    //         Token {
    //             span: Span::new(0, 0),
    //             kind: TokenKind::Not,
    //         },
    //         Token {
    //             span: Span::new(0, 0),
    //             kind: TokenKind::Literal(Literal::Bool(true)),
    //         },
    //     ];

    //     let mut parser = RecursiveDescentParser::new(&tokens);

    //     let actual = parser.parse().expect("oops");
    //     println!("{:#?}", actual);
    //     assert_eq!(expected, actual);
    // }

    // #[test]
    // fn parse_factor() {
    //     let expected = Expr {
    //         kind: ExprKind::Binary(
    //             BinOp::Mul,
    //             Box::new(Expr {
    //                 kind: ExprKind::Literal(Literal::Number("2".to_string())),
    //             }),
    //             Box::new(Expr {
    //                 kind: ExprKind::Unary(
    //                     UnOp::Neg,
    //                     Box::new(Expr {
    //                         kind: ExprKind::Literal(Literal::Number("3".to_string())),
    //                     }),
    //                 ),
    //             }),
    //         ),
    //     };

    //     let tokens = vec![
    //         Token {
    //             span: Span::new(0, 0),
    //             kind: TokenKind::Literal(Literal::Number("2".to_string())),
    //         },
    //         Token {
    //             span: Span::new(0, 0),
    //             kind: TokenKind::Star,
    //         },
    //         Token {
    //             span: Span::new(0, 0),
    //             kind: TokenKind::Minus,
    //         },
    //         Token {
    //             span: Span::new(0, 0),
    //             kind: TokenKind::Literal(Literal::Number("3".to_string())),
    //         },
    //     ];

    //     let mut parser = RecursiveDescentParser::new(&tokens);

    //     let actual = parser.parse().expect("oops");
    //     println!("{:#?}", actual);
    //     assert_eq!(expected, actual);
    // }

    // #[test]
    // fn parse_term() {
    //     let expected = Expr {
    //         kind: ExprKind::Binary(
    //             BinOp::Add,
    //             Box::new(Expr {
    //                 kind: ExprKind::Unary(
    //                     UnOp::Neg,
    //                     Box::new(Expr {
    //                         kind: ExprKind::Literal(Literal::Number("9".to_string())),
    //                     }),
    //                 ),
    //             }),
    //             Box::new(Expr {
    //                 kind: ExprKind::Binary(
    //                     BinOp::Mul,
    //                     Box::new(Expr {
    //                         kind: ExprKind::Literal(Literal::Number("2".to_string())),
    //                     }),
    //                     Box::new(Expr {
    //                         kind: ExprKind::Unary(
    //                             UnOp::Neg,
    //                             Box::new(Expr {
    //                                 kind: ExprKind::Literal(Literal::Number("3".to_string())),
    //                             }),
    //                         ),
    //                     }),
    //                 ),
    //             }),
    //         ),
    //     };

    //     let tokens = vec![
    //         Token {
    //             span: Span::new(0, 0),
    //             kind: TokenKind::Minus,
    //         },
    //         Token {
    //             span: Span::new(0, 0),
    //             kind: TokenKind::Literal(Literal::Number("9".to_string())),
    //         },
    //         Token {
    //             span: Span::new(0, 0),
    //             kind: TokenKind::Plus,
    //         },
    //         Token {
    //             span: Span::new(0, 0),
    //             kind: TokenKind::Literal(Literal::Number("2".to_string())),
    //         },
    //         Token {
    //             span: Span::new(0, 0),
    //             kind: TokenKind::Star,
    //         },
    //         Token {
    //             span: Span::new(0, 0),
    //             kind: TokenKind::Minus,
    //         },
    //         Token {
    //             span: Span::new(0, 0),
    //             kind: TokenKind::Literal(Literal::Number("3".to_string())),
    //         },
    //     ];

    //     let mut parser = RecursiveDescentParser::new(&tokens);

    //     let actual = parser.parse().expect("oops");
    //     println!("{:#?}", actual);
    //     assert_eq!(expected, actual);
    // }

    // #[test]
    // fn parse_comparison() {
    //     let expected = Expr {
    //         kind: ExprKind::Binary(
    //             BinOp::Lt,
    //             Box::new(Expr {
    //                 kind: ExprKind::Literal(Literal::Number("1".to_string())),
    //             }),
    //             Box::new(Expr {
    //                 kind: ExprKind::Binary(
    //                     BinOp::Add,
    //                     Box::new(Expr {
    //                         kind: ExprKind::Unary(
    //                             UnOp::Neg,
    //                             Box::new(Expr {
    //                                 kind: ExprKind::Literal(Literal::Number("9".to_string())),
    //                             }),
    //                         ),
    //                     }),
    //                     Box::new(Expr {
    //                         kind: ExprKind::Binary(
    //                             BinOp::Mul,
    //                             Box::new(Expr {
    //                                 kind: ExprKind::Literal(Literal::Number("2".to_string())),
    //                             }),
    //                             Box::new(Expr {
    //                                 kind: ExprKind::Unary(
    //                                     UnOp::Neg,
    //                                     Box::new(Expr {
    //                                         kind: ExprKind::Literal(Literal::Number(
    //                                             "3".to_string(),
    //                                         )),
    //                                     }),
    //                                 ),
    //                             }),
    //                         ),
    //                     }),
    //                 ),
    //             }),
    //         ),
    //     };

    //     let tokens = vec![
    //         Token {
    //             span: Span::new(0, 0),
    //             kind: TokenKind::Literal(Literal::Number("1".to_string())),
    //         },
    //         Token {
    //             span: Span::new(0, 0),
    //             kind: TokenKind::Lt,
    //         },
    //         Token {
    //             span: Span::new(0, 0),
    //             kind: TokenKind::Minus,
    //         },
    //         Token {
    //             span: Span::new(0, 0),
    //             kind: TokenKind::Literal(Literal::Number("9".to_string())),
    //         },
    //         Token {
    //             span: Span::new(0, 0),
    //             kind: TokenKind::Plus,
    //         },
    //         Token {
    //             span: Span::new(0, 0),
    //             kind: TokenKind::Literal(Literal::Number("2".to_string())),
    //         },
    //         Token {
    //             span: Span::new(0, 0),
    //             kind: TokenKind::Star,
    //         },
    //         Token {
    //             span: Span::new(0, 0),
    //             kind: TokenKind::Minus,
    //         },
    //         Token {
    //             span: Span::new(0, 0),
    //             kind: TokenKind::Literal(Literal::Number("3".to_string())),
    //         },
    //     ];

    //     let mut parser = RecursiveDescentParser::new(&tokens);

    //     let actual = parser.parse().expect("oops");
    //     println!("{:#?}", actual);
    //     assert_eq!(expected, actual);
    // }

    // #[test]
    // fn parse_equality() {
    //     let expected = Expr {
    //         kind: ExprKind::Binary(
    //             BinOp::Eq,
    //             Box::new(Expr {
    //                 kind: ExprKind::Literal(Literal::Bool(true)),
    //             }),
    //             Box::new(Expr {
    //                 kind: ExprKind::Binary(
    //                     BinOp::Lt,
    //                     Box::new(Expr {
    //                         kind: ExprKind::Literal(Literal::Number("1".to_string())),
    //                     }),
    //                     Box::new(Expr {
    //                         kind: ExprKind::Binary(
    //                             BinOp::Add,
    //                             Box::new(Expr {
    //                                 kind: ExprKind::Unary(
    //                                     UnOp::Neg,
    //                                     Box::new(Expr {
    //                                         kind: ExprKind::Literal(Literal::Number(
    //                                             "9".to_string(),
    //                                         )),
    //                                     }),
    //                                 ),
    //                             }),
    //                             Box::new(Expr {
    //                                 kind: ExprKind::Binary(
    //                                     BinOp::Mul,
    //                                     Box::new(Expr {
    //                                         kind: ExprKind::Literal(Literal::Number(
    //                                             "2".to_string(),
    //                                         )),
    //                                     }),
    //                                     Box::new(Expr {
    //                                         kind: ExprKind::Unary(
    //                                             UnOp::Neg,
    //                                             Box::new(Expr {
    //                                                 kind: ExprKind::Literal(Literal::Number(
    //                                                     "3".to_string(),
    //                                                 )),
    //                                             }),
    //                                         ),
    //                                     }),
    //                                 ),
    //                             }),
    //                         ),
    //                     }),
    //                 ),
    //             }),
    //         ),
    //     };

    //     let tokens = vec![
    //         Token {
    //             span: Span::new(0, 0),
    //             kind: TokenKind::Literal(Literal::Bool(true)),
    //         },
    //         Token {
    //             span: Span::new(0, 0),
    //             kind: TokenKind::EqEq,
    //         },
    //         Token {
    //             span: Span::new(0, 0),
    //             kind: TokenKind::Literal(Literal::Number("1".to_string())),
    //         },
    //         Token {
    //             span: Span::new(0, 0),
    //             kind: TokenKind::Lt,
    //         },
    //         Token {
    //             span: Span::new(0, 0),
    //             kind: TokenKind::Minus,
    //         },
    //         Token {
    //             span: Span::new(0, 0),
    //             kind: TokenKind::Literal(Literal::Number("9".to_string())),
    //         },
    //         Token {
    //             span: Span::new(0, 0),
    //             kind: TokenKind::Plus,
    //         },
    //         Token {
    //             span: Span::new(0, 0),
    //             kind: TokenKind::Literal(Literal::Number("2".to_string())),
    //         },
    //         Token {
    //             span: Span::new(0, 0),
    //             kind: TokenKind::Star,
    //         },
    //         Token {
    //             span: Span::new(0, 0),
    //             kind: TokenKind::Minus,
    //         },
    //         Token {
    //             span: Span::new(0, 0),
    //             kind: TokenKind::Literal(Literal::Number("3".to_string())),
    //         },
    //     ];

    //     let mut parser = RecursiveDescentParser::new(&tokens);

    //     let actual = parser.parse().expect("oops");
    //     println!("{:#?}", actual);
    //     assert_eq!(expected, actual);
    // }
}
