use std::collections::HashMap;

use mypl_lex::prelude::*;
use mypl_ast::prelude::*;

use anyhow::Result;

use crate::{
    error::InterperterError,
    expr_eval::{
        ExprEval, Value, ValueType
    }
};

pub struct Interperter {
    variables: HashMap<String, Value>,
}

impl Interperter {
    pub fn new() -> Self {
        Self {
            variables: Default::default(),
        }
    }

    pub fn evaluate_expr(&mut self, expr: &Expr) -> Result<Value, InterperterError> {
        expr.accept_expr_visitor(self)
    }

    pub fn interpret_stmt(&mut self, stmt: &Stmt) -> Result<(), InterperterError> {
        stmt.accept_stmt_visitor(self)?;
        Ok(())
    }
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

impl ExprVisitor for Interperter {
    type Result = Result<Value, InterperterError>;

    fn visit_binary_expr(&mut self, op: &BinOp, lhs: &Expr, rhs: &Expr) -> Self::Result {
        use InterperterError::*;

        let lhs_val = self.evaluate_expr(&lhs)?;
        let rhs_val = self.evaluate_expr(&rhs)?;

        let lhs_type = lhs_val.get_type();
        let rhs_type = rhs_val.get_type();

        // TODO: improve. Language features such as coercion, inference.
        if lhs_type != rhs_type {
            return Err(BinaryExprTypeMismatch(lhs_type, rhs_type));
        }

        // Remember that for now, both types are the same
        match lhs_type {
            ValueType::String => ExprEval::bin_string_string(op, &lhs_val, &rhs_val),
            ValueType::Float => ExprEval::bin_float_float(op, &lhs_val, &rhs_val),
            ValueType::Integer=> ExprEval::bin_integer_integer(op, &lhs_val, &rhs_val),
            ValueType::Bool =>  ExprEval::bin_bool_bool(op, &lhs_val, &rhs_val),
        }
    }

    fn visit_unary_expr(&mut self, op: &UnOp, expr: &Expr) -> Self::Result {
        use InterperterError::*;
        let expr_val = self.evaluate_expr(&expr)?;
        match expr_val {
            Value::String(_) => Err(InvalidUnaryApplication(*op, ValueType::String)),
            Value::Bool(val) => match op {
                UnOp::Not => Ok(Value::Bool(!val)),
                UnOp::Neg => Err(InvalidUnaryApplication(*op, ValueType::Bool)),
            }
            Value::Integer(val) => match op {
                UnOp::Not => Err(InvalidUnaryApplication(*op, ValueType::Integer)),
                UnOp::Neg => Ok(Value::Integer(-1 * val)),
            }
            Value::Float(val) => match op {
                UnOp::Not => Err(InvalidUnaryApplication(*op, ValueType::Float)),
                UnOp::Neg => Ok(Value::Float(-1.0 * val)),
            }
        }
    }

    fn visit_literal_expr(&mut self, literal: &Literal) -> Self::Result {
        Ok(match literal {
            Literal::String(val) => Value::String(val.clone()), 
            Literal::Bool(val) => Value::Bool(val.clone()),
            Literal::Integer(val) => Value::Integer(val.clone()),
            Literal::Float(val) => Value::Float(val.clone()),
        })
    }

    fn visit_variable_expr(&mut self, identifier: &String) -> Self::Result {
        if let Some(val) = self.variables.get(identifier) {
            Ok(val.clone())
        } else {
            Err(InterperterError::SymbolNotFound(identifier.to_string(), "Variable".to_string()))
        }
    }
}

impl StmtVisitor for Interperter {
    type Result = Result<(), InterperterError>;

    fn visit_expr_stmt(&mut self, expr: &Expr) -> Self::Result {
        let val = self.evaluate_expr(&expr)?;
        println!("{:?}", val);

        Ok(())
    }

    fn visit_print_stmt(&mut self, expr: &Expr) -> Self::Result {
        let expr_val = self.evaluate_expr(&expr)?; 
        println!("{:?}", expr_val);
        Ok(())
    }

    fn visit_decl_stmt(&mut self, decl: &Decl) -> Self::Result {
        match &decl.kind {
            DeclKind::Const(ident, init_expr) | DeclKind::Var(ident, init_expr) => {
                let expr_val = self.evaluate_expr(init_expr.as_ref())?;
                
                if self.variables.contains_key(ident) {
                    return Err(InterperterError::SymbolAlreadyDefined(ident.to_string()))
                }

                self.variables.insert(ident.to_string(), expr_val);
                Ok(())
            },
        }
    }
}
