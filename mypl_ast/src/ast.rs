use mypl_lex::prelude::*;

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
