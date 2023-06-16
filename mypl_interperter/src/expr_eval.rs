use mypl_ast::prelude::BinOp;

use crate::prelude::InterperterError;

#[derive(Debug, PartialEq, Clone)]
pub enum Value {
    String(String),
    Float(f64),
    Integer(i128),
    Bool(bool),
}

impl Value {
    pub fn get_type(&self) -> ValueType {
        match self {
            Value::String(_) => ValueType::String, 
            Value::Float(_) => ValueType::Float,
            Value::Integer(_) => ValueType::Integer,
            Value::Bool(_) => ValueType::Bool,
        }
    }

    pub fn unwrap_string(&self) -> &String {
        match self {
            Value::String(val) => val,
            _ => panic!("attempted to unwrap_string, but is: {:?}", self),
        }
    }
    
    pub fn unwrap_float(&self) -> &f64 {
        match self {
            Value::Float(val) => val,
            _ => panic!("attempted to unwrap_float, but is: {:?}", self),
        }
    }

    pub fn unwrap_integer(&self) -> &i128 {
        match self {
            Value::Integer(val) => val,
            _ => panic!("attempted to unwrap_integer, but is: {:?}", self),
        }
    }
    
    pub fn unwrap_bool(&self) -> &bool {
        match self {
            Value::Bool(val) => val,
            _ => panic!("attempted to unwrap_bool, but is: {:?}", self),
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub enum ValueType {
    String,
    Float,
    Integer,
    Bool,
}

pub(crate) struct ExprEval;

impl ExprEval {
    pub fn bin_string_string(op: &BinOp, lhs: &Value, rhs: &Value) -> Result<Value, InterperterError> {
        let lhs = lhs.unwrap_string();
        let rhs = rhs.unwrap_string();
        match op {
            BinOp::Eq => Ok(Value::Bool(lhs == rhs)),
            BinOp::Ne => Ok(Value::Bool(lhs != rhs)),
            _ => Err(InterperterError::InvalidBinaryApplication(*op, ValueType::String, ValueType::String)),
        }
    }

    pub fn bin_integer_integer(op: &BinOp, lhs: &Value, rhs: &Value) -> Result<Value, InterperterError> {
        let lhs = lhs.unwrap_integer();
        let rhs = rhs.unwrap_integer();
        match op {
            BinOp::Add => Ok(Value::Integer(lhs + rhs)),
            BinOp::Sub => Ok(Value::Integer(lhs - rhs)),
            BinOp::Mul => Ok(Value::Integer(lhs * rhs)),
            BinOp::Div => Ok(Value::Float(*lhs as f64 / *rhs as f64)),
            BinOp::Eq => Ok(Value::Bool(lhs == rhs)),
            BinOp::Lt => Ok(Value::Bool(lhs < rhs)),
            BinOp::Le => Ok(Value::Bool(lhs <= rhs)),
            BinOp::Ne => Ok(Value::Bool(lhs != rhs)),
            BinOp::Ge => Ok(Value::Bool(lhs > rhs)),
            BinOp::Gt => Ok(Value::Bool(lhs >= rhs)),
            _ => Err(InterperterError::InvalidBinaryApplication(*op, ValueType::Integer, ValueType::Integer)),
        }
    }

    pub fn bin_float_float(op: &BinOp, lhs: &Value, rhs: &Value) -> Result<Value, InterperterError> {
        let lhs = lhs.unwrap_float();
        let rhs = rhs.unwrap_float();
        match op {
            BinOp::Add => Ok(Value::Float(lhs + rhs)),
            BinOp::Sub => Ok(Value::Float(lhs - rhs)),
            BinOp::Mul => Ok(Value::Float(lhs * rhs)),
            BinOp::Div => Ok(Value::Float(lhs / rhs)),
            BinOp::Eq => Ok(Value::Bool(lhs == rhs)),
            BinOp::Lt => Ok(Value::Bool(lhs < rhs)),
            BinOp::Le => Ok(Value::Bool(lhs <= rhs)),
            BinOp::Ne => Ok(Value::Bool(lhs != rhs)),
            BinOp::Ge => Ok(Value::Bool(lhs > rhs)),
            BinOp::Gt => Ok(Value::Bool(lhs >= rhs)),
            _ => Err(InterperterError::InvalidBinaryApplication(*op, ValueType::Float, ValueType::Float)),
        }
    }

    pub fn bin_bool_bool(op: &BinOp, lhs: &Value, rhs: &Value) -> Result<Value, InterperterError> {
        let lhs = *lhs.unwrap_bool();
        let rhs = *rhs.unwrap_bool();
        match op {
            BinOp::And => Ok(Value::Bool(lhs && rhs)),
            BinOp::Or  => Ok(Value::Bool(lhs || rhs)),
            BinOp::Eq  => Ok(Value::Bool(lhs == rhs)),
            BinOp::Ne  => Ok(Value::Bool(lhs != rhs)),
            _ => Err(InterperterError::InvalidBinaryApplication(*op, ValueType::Bool, ValueType::Bool)),
        }
    }
}


