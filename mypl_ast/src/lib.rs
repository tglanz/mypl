extern crate mypl_lex;
extern crate anyhow;

use mypl_lex::prelude::Literal;

#[derive(Clone, Debug, PartialEq, Copy)]
pub enum BinOp {
    Add,
    Sub,
    Mul,
    Div,
    Rem,
    And,
    Or,
    Eq,
    Lt,
    Le,
    Ne,
    Ge,
    Gt,
}

#[derive(Clone, Debug, PartialEq, Copy)]
pub enum UnOp {
    /// The `!` operator for logical inversion
    Not,
    /// The `-` operator for negation
    Neg,
}

#[derive(Clone, PartialEq, Debug)]
pub enum ExprKind {
    Binary(BinOp, Box<Expr>, Box<Expr>),
    Unary(UnOp, Box<Expr>),
    Literal(Literal),
}

#[derive(Clone, PartialEq, Debug)]
pub struct Expr {
    pub kind: ExprKind
}