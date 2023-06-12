use mypl_lex::prelude::Literal;
use crate::prelude::*;

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


