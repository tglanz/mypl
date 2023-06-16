use mypl_lex::prelude::*;
use mypl_ast::prelude::*;

use anyhow::Result;

use crate::{
    error::InterperterError,
    expr_eval::{
        ExprEval, Value, ValueType
    },
    symbol::Mutability,
    environment::Environment,
};

pub struct Interperter {
    global_env: Environment,
}

impl Interperter {
    pub fn new() -> Self {
        Self {
            global_env: Default::default(),
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
        Ok(self.global_env
               .get_variable_value(identifier)?
               .clone())
    }
}

impl StmtVisitor for Interperter {
    type Result = Result<(), InterperterError>;

    fn visit_expr_stmt(&mut self, expr: &Expr) -> Self::Result {
        let val = self.evaluate_expr(&expr)?;
        println!("{:?}", val);

        Ok(())
    }
    
    fn visit_println_stmt(&mut self, expr: &Expr) -> Self::Result {
        match self.evaluate_expr(&expr)? {
            Value::String(val) => println!("{}", val), 
            Value::Float(val) => println!("{}", val), 
            Value::Integer(val) => println!("{}", val), 
            Value::Bool(val) => println!("{}", val), 
        }
        Ok(())
    }

    fn visit_print_stmt(&mut self, expr: &Expr) -> Self::Result {
        match self.evaluate_expr(&expr)? {
            Value::String(val) => print!("{}", val), 
            Value::Float(val) => print!("{}", val), 
            Value::Integer(val) => print!("{}", val), 
            Value::Bool(val) => print!("{}", val), 
        }
        Ok(())
    }

    fn visit_decl_stmt(&mut self, decl: &Decl) -> Self::Result {
        match &decl.kind {
            DeclKind::Const(identifier, expr) => {
                let val = self.evaluate_expr(expr)?;
                self.global_env.define_variable(identifier, Mutability::Immutable, val)?;
                Ok(())
            },
            DeclKind::Var(identifier, expr) => {
                let val = self.evaluate_expr(expr)?;
                self.global_env.define_variable(identifier, Mutability::Mutable, val)?;
                Ok(())
           },
        }
    }

    fn visit_assign_stmt(&mut self, identifier: &String, expr: &Expr) -> Self::Result {
        let value = self.evaluate_expr(expr)?;
        self.global_env.assign_to_variable(identifier, value)?;
        Ok(())
    }
}
