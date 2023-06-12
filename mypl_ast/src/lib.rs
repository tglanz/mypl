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

impl BinOp {
    pub fn as_code(&self) -> &str {
        match self {
            BinOp::Add => "+", 
            BinOp::Sub => "-", 
            BinOp::Mul => "*", 
            BinOp::Div => "/", 
            BinOp::Rem => "%", 
            BinOp::And => "&&", 
            BinOp::Or => "||", 
            BinOp::Eq => "==", 
            BinOp::Lt => "<", 
            BinOp::Le => "<=", 
            BinOp::Ne => "!=", 
            BinOp::Ge => ">=", 
            BinOp::Gt => ">", 
        }
    }
}

#[derive(Clone, Debug, PartialEq, Copy)]
pub enum UnOp {
    /// The `!` operator for logical inversion
    Not,
    /// The `-` operator for negation
    Neg,
}

impl UnOp {
    pub fn as_code(&self) -> &str {
        match self {
            UnOp::Not => "!", 
            UnOp::Neg => "-",
        }
    }
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

pub trait AcceptVisitor {
    fn accept_visitor<V: Visitor>(&self, visitor: &mut V) -> V::Result;
}

pub trait Visitor {
    type Result;

    fn visit_binary_expr(&mut self, op: &BinOp, lhs: &Expr, rhs: &Expr) -> Self::Result;
    fn visit_unary_expr(&mut self, op: &UnOp, expr: &Expr) -> Self::Result;
    fn visit_literal_expr(&mut self, literal: &Literal) -> Self::Result; 
}

impl AcceptVisitor for Expr {
    fn accept_visitor<V: Visitor>(&self, visitor: &mut V) -> V::Result {
        use ExprKind::*;
        match &self.kind {
            Binary(op, lhs, rhs) => visitor.visit_binary_expr(op, lhs, rhs),
            Unary(op, expr) => visitor.visit_unary_expr(op, expr),
            Literal(literal) => visitor.visit_literal_expr(literal),
            
        }
    }
}

pub struct AstFormatter;

impl AstFormatter {
    pub fn format_ast(ast: &Expr) -> String {
        let repr = format!("{:#?}", ast);
        repr.replace("    ", "  ")
    }
}
