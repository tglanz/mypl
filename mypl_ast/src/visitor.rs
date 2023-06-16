use mypl_lex::prelude::Literal;
use crate::prelude::*;

pub trait AcceptExprVisitor {
    fn accept_expr_visitor<V: ExprVisitor>(&self, visitor: &mut V) -> V::Result;
}

pub trait ExprVisitor {
    type Result;

    fn visit_binary_expr(&mut self, op: &BinOp, lhs: &Expr, rhs: &Expr) -> Self::Result;
    fn visit_unary_expr(&mut self, op: &UnOp, expr: &Expr) -> Self::Result;
    fn visit_literal_expr(&mut self, literal: &Literal) -> Self::Result; 
    fn visit_variable_expr(&mut self, identifier: &String) -> Self::Result;
}

impl AcceptExprVisitor for Expr {
    fn accept_expr_visitor<V: ExprVisitor>(&self, visitor: &mut V) -> V::Result {
        use ExprKind::*;
        match &self.kind {
            Binary(op, lhs, rhs) => visitor.visit_binary_expr(op, lhs, rhs),
            Unary(op, expr) => visitor.visit_unary_expr(op, expr),
            Literal(literal) => visitor.visit_literal_expr(literal),
            Variable(identifier) => visitor.visit_variable_expr(identifier),
            
        }
    }
}

pub trait AcceptStmtVisitor {
    fn accept_stmt_visitor<V: StmtVisitor>(&self, visitor: &mut V) -> V::Result;
}

pub trait StmtVisitor {
    type Result;

    fn visit_expr_stmt(&mut self, expr: &Expr) -> Self::Result;
    fn visit_print_stmt(&mut self, expr: &Expr) -> Self::Result;
    fn visit_decl_stmt(&mut self, decl: &Decl) -> Self::Result;
    fn visit_assign_stmt(&mut self, identifier: &String, expr: &Expr) -> Self::Result;
}

impl AcceptStmtVisitor for Stmt {
    fn accept_stmt_visitor<V: StmtVisitor>(&self, visitor: &mut V) -> V::Result {
        use StmtKind::*;
        match &self.kind {
            Expr(expr) => visitor.visit_expr_stmt(expr),
            Print(expr) => visitor.visit_print_stmt(expr),
            Decl(decl) => visitor.visit_decl_stmt(decl),
            Assign(ident, expr) => visitor.visit_assign_stmt(ident, expr),
        }
    }
}

