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

impl std::fmt::Display for BinOp {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(match self {
            BinOp::Add => "+",
            BinOp::Sub => "-",
            BinOp::Mul => "*",
            BinOp::Div => "/",
            BinOp::Rem => "%",
            BinOp::And => "&",
            BinOp::Or => "||",
            BinOp::Eq => "==",
            BinOp::Lt => "<",
            BinOp::Le => "<=",
            BinOp::Ne => "!=",
            BinOp::Ge => ">=",
            BinOp::Gt => ">",
        })
    }
}

#[derive(Clone, Debug, PartialEq, Copy)]
pub enum UnOp {
    /// The `!` operator for logical inversion
    Not,
    /// The `-` operator for negation
    Neg,
}

impl std::fmt::Display for UnOp {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(match self {
            UnOp::Not => "!",
            UnOp::Neg => "-",
        })
    }
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
    Variable(String),
}

#[derive(Clone, PartialEq, Debug)]
pub struct Expr {
    pub kind: ExprKind,
}

#[derive(Clone, PartialEq, Debug)]
pub enum DeclKind {
    Var(String, Box::<Expr>),
    Const(String, Box::<Expr>),
}

#[derive(Clone, PartialEq, Debug)]
pub struct Decl {
    pub kind: DeclKind,
}

#[derive(Clone, PartialEq, Debug)]
pub enum StmtKind {
    Expr(Box<Expr>),
    Print(Box<Expr>),
    Println(Box<Expr>),
    Decl(Box<Decl>),
    Assign(String, Box<Expr>),
}

#[derive(Clone, PartialEq, Debug)]
pub struct Stmt {
    pub kind: StmtKind,
}

