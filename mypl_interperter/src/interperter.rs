use mypl_lex::prelude::*;
use mypl_ast::prelude::*;

use anyhow::Result;

#[derive(Debug, thiserror::Error)]
pub enum InterperterError {
    #[error("GenericInterperterError({0})")]
    Generic(String),

    #[error("Binary expression type mismatch: {0:?} != {1:?}")]
    BinaryExprTypeMismatch(ExprType, ExprType),

    #[error("Cannot apply binary operator \"{0}\" on types \"{1:?}\" and \"{2:?}\"")]
    InvalidBinaryApplication(BinOp, ExprType, ExprType),

    #[error("Cannot apply unary operator \"{0}\" on type \"{1:?}\"")]
    InvalidUnaryApplication(UnOp, ExprType),
}

pub struct Interperter {}

impl Interperter {
    pub fn new() -> Self {
        Self {}
    }

    pub fn interpret_expr(&mut self, expr: &Expr) -> Result<ExprValue, InterperterError> {
        expr.accept_visitor(self)
    }
}

#[derive(Debug, PartialEq, Clone)]
pub enum ExprValue {
    String(String),
    Float(f64),
    Integer(i128),
    Bool(bool),
}

impl ExprValue {
    fn get_type(&self) -> ExprType {
        match self {
            ExprValue::String(_) => ExprType::String, 
            ExprValue::Float(_) => ExprType::Float,
            ExprValue::Integer(_) => ExprType::Integer,
            ExprValue::Bool(_) => ExprType::Bool,
        }
    }

    fn unwrap_string(&self) -> &String {
        match self {
            ExprValue::String(val) => val,
            _ => panic!("attempted to unwrap_string, but is: {:?}", self),
        }
    }
    
    fn unwrap_float(&self) -> &f64 {
        match self {
            ExprValue::Float(val) => val,
            _ => panic!("attempted to unwrap_float, but is: {:?}", self),
        }
    }

    fn unwrap_integer(&self) -> &i128 {
        match self {
            ExprValue::Integer(val) => val,
            _ => panic!("attempted to unwrap_integer, but is: {:?}", self),
        }
    }
    
    fn unwrap_bool(&self) -> &bool {
        match self {
            ExprValue::Bool(val) => val,
            _ => panic!("attempted to unwrap_bool, but is: {:?}", self),
        }
    }
}

#[derive(Debug, PartialEq)]
pub enum ExprType {
    String,
    Float,
    Integer,
    Bool,
}

// fn deduce_expr_type(expr: &Expr) -> Result<ExprType, InterperterError> {
//     let expr_type = match &expr.kind {
//         ExprKind::Literal(lit) => match lit {
//             Literal::String(_) => ExprType::String,
//             Literal::Number(_) => ExprType::Integer, // TODO: this is simplification. float != int
//                                                      // but both are numbers
//             Literal::Bool(_) => ExprType::Bool,
//         },
//         ExprKind::Unary(_, inner) => deduce_expr_type(&inner)?,
//         ExprKind::Binary(_, lhs, rhs) => {
//             let lhs_type = deduce_expr_type(&lhs)?;
//             let rhs_type = deduce_expr_type(&rhs)?;
//             if lhs_type != rhs_type {
//                 return Err(InterperterError::BinaryExprTypeMismatch(lhs_type, rhs_type));
//             }
// 
//             lhs_type
//         }
//     };
// 
//     Ok(expr_type)
// }

struct Eval;

impl Eval {
    fn bin_string_string(op: &BinOp, lhs: &ExprValue, rhs: &ExprValue) -> Result<ExprValue, InterperterError> {
        let lhs = lhs.unwrap_string();
        let rhs = rhs.unwrap_string();
        match op {
            BinOp::Eq => Ok(ExprValue::Bool(lhs == rhs)),
            BinOp::Ne => Ok(ExprValue::Bool(lhs != rhs)),
            _ => Err(InterperterError::InvalidBinaryApplication(*op, ExprType::String, ExprType::String)),
        }
    }

    fn bin_integer_integer(op: &BinOp, lhs: &ExprValue, rhs: &ExprValue) -> Result<ExprValue, InterperterError> {
        let lhs = lhs.unwrap_integer();
        let rhs = rhs.unwrap_integer();
        match op {
            BinOp::Add => Ok(ExprValue::Integer(lhs + rhs)),
            BinOp::Sub => Ok(ExprValue::Integer(lhs - rhs)),
            BinOp::Mul => Ok(ExprValue::Integer(lhs * rhs)),
            BinOp::Div => Ok(ExprValue::Float(*lhs as f64 / *rhs as f64)),
            BinOp::Eq => Ok(ExprValue::Bool(lhs == rhs)),
            BinOp::Lt => Ok(ExprValue::Bool(lhs < rhs)),
            BinOp::Le => Ok(ExprValue::Bool(lhs <= rhs)),
            BinOp::Ne => Ok(ExprValue::Bool(lhs != rhs)),
            BinOp::Ge => Ok(ExprValue::Bool(lhs > rhs)),
            BinOp::Gt => Ok(ExprValue::Bool(lhs >= rhs)),
            _ => Err(InterperterError::InvalidBinaryApplication(*op, ExprType::Integer, ExprType::Integer)),
        }
    }

    fn bin_float_float(op: &BinOp, lhs: &ExprValue, rhs: &ExprValue) -> Result<ExprValue, InterperterError> {
        let lhs = lhs.unwrap_float();
        let rhs = rhs.unwrap_float();
        match op {
            BinOp::Add => Ok(ExprValue::Float(lhs + rhs)),
            BinOp::Sub => Ok(ExprValue::Float(lhs - rhs)),
            BinOp::Mul => Ok(ExprValue::Float(lhs * rhs)),
            BinOp::Div => Ok(ExprValue::Float(lhs / rhs)),
            BinOp::Eq => Ok(ExprValue::Bool(lhs == rhs)),
            BinOp::Lt => Ok(ExprValue::Bool(lhs < rhs)),
            BinOp::Le => Ok(ExprValue::Bool(lhs <= rhs)),
            BinOp::Ne => Ok(ExprValue::Bool(lhs != rhs)),
            BinOp::Ge => Ok(ExprValue::Bool(lhs > rhs)),
            BinOp::Gt => Ok(ExprValue::Bool(lhs >= rhs)),
            _ => Err(InterperterError::InvalidBinaryApplication(*op, ExprType::Float, ExprType::Float)),
        }
    }

    fn bin_bool_bool(op: &BinOp, lhs: &ExprValue, rhs: &ExprValue) -> Result<ExprValue, InterperterError> {
        let lhs = *lhs.unwrap_bool();
        let rhs = *rhs.unwrap_bool();
        match op {
            BinOp::And => Ok(ExprValue::Bool(lhs && rhs)),
            BinOp::Or  => Ok(ExprValue::Bool(lhs || rhs)),
            BinOp::Eq  => Ok(ExprValue::Bool(lhs == rhs)),
            BinOp::Ne  => Ok(ExprValue::Bool(lhs != rhs)),
            _ => Err(InterperterError::InvalidBinaryApplication(*op, ExprType::Bool, ExprType::Bool)),
        }
    }
}

impl Visitor for Interperter {
    type Result = Result<ExprValue, InterperterError>;

    fn visit_binary_expr(&mut self, op: &BinOp, lhs: &Expr, rhs: &Expr) -> Self::Result {
        use InterperterError::*;

        let lhs_val = self.interpret_expr(lhs)?;
        let rhs_val = self.interpret_expr(rhs)?;

        let lhs_type = lhs_val.get_type();
        let rhs_type = rhs_val.get_type();

        // TODO: improve. Language features such as coercion, inference.
        if lhs_type != rhs_type {
            return Err(BinaryExprTypeMismatch(lhs_type, rhs_type));
        }

        // Remember that for now, both types are the same
        match lhs_type {
            ExprType::String => Eval::bin_string_string(op, &lhs_val, &rhs_val),
            ExprType::Float => Eval::bin_float_float(op, &lhs_val, &rhs_val),
            ExprType::Integer=> Eval::bin_integer_integer(op, &lhs_val, &rhs_val),
            ExprType::Bool =>  Eval::bin_bool_bool(op, &lhs_val, &rhs_val),
        }
    }

    fn visit_unary_expr(&mut self, op: &UnOp, expr: &Expr) -> Self::Result {
        use InterperterError::*;
        let expr_val = self.interpret_expr(expr)?;
        match expr_val {
            ExprValue::String(_) => Err(InvalidUnaryApplication(*op, ExprType::String)),
            ExprValue::Bool(val) => match op {
                UnOp::Not => Ok(ExprValue::Bool(!val)),
                UnOp::Neg => Err(InvalidUnaryApplication(*op, ExprType::Bool)),
            }
            ExprValue::Integer(val) => match op {
                UnOp::Not => Err(InvalidUnaryApplication(*op, ExprType::Integer)),
                UnOp::Neg => Ok(ExprValue::Integer(-1 * val)),
            }
            ExprValue::Float(val) => match op {
                UnOp::Not => Err(InvalidUnaryApplication(*op, ExprType::Float)),
                UnOp::Neg => Ok(ExprValue::Float(-1.0 * val)),
            }
        }
    }

    fn visit_literal_expr(&mut self, literal: &Literal) -> Self::Result {
        Ok(match literal {
            Literal::String(val) => ExprValue::String(val.clone()), 
            Literal::Bool(val) => ExprValue::Bool(val.clone()),
            Literal::Integer(val) => ExprValue::Integer(val.clone()),
            Literal::Float(val) => ExprValue::Float(val.clone()),
        })
    }
}
