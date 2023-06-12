use mypl_ast::prelude::{BinOp, UnOp};
use mypl_lex::prelude::TokenKind;

pub trait TokenKindExtensions {
    fn is_eof(&self) -> bool;
    fn to_binary_op(&self) -> Option<BinOp>;
    fn to_unary_op(&self) -> Option<UnOp>;
}

impl TokenKindExtensions for TokenKind {
    fn is_eof(&self) -> bool {
        match self {
            TokenKind::Eof => true,
            _ => false,
        }
    }

    fn to_unary_op(&self) -> Option<UnOp> {
        use TokenKind::*;
        match self {
            Not => Some(UnOp::Not),
            Minus => Some(UnOp::Neg),
            _ => None,
        }
    }

    fn to_binary_op(&self) -> Option<BinOp> {
        use TokenKind::*;
        match self {
            EqEq => Some(BinOp::Eq),
            Lt => Some(BinOp::Lt),
            Le => Some(BinOp::Le),
            Ne => Some(BinOp::Ne),
            Ge => Some(BinOp::Ge),
            Gt => Some(BinOp::Gt),
            AndAnd => Some(BinOp::And),
            OrOr => Some(BinOp::Or),
            Plus => Some(BinOp::Add),
            Minus => Some(BinOp::Sub),
            Star => Some(BinOp::Mul),
            Slash => Some(BinOp::Div),
            Percent => Some(BinOp::Rem),
            _ => None,
        }
    }
}
