use mypl_ast::prelude::*;
use crate::expr_eval::ValueType;

#[derive(Debug, thiserror::Error)]
pub enum InterperterError {
    #[error("GenericInterperterError({0})")]
    Generic(String),

    #[error("Binary expression type mismatch: {0:?} != {1:?}")]
    BinaryExprTypeMismatch(ValueType, ValueType),

    #[error("Cannot apply binary operator \"{0}\" on types \"{1:?}\" and \"{2:?}\"")]
    InvalidBinaryApplication(BinOp, ValueType, ValueType),

    #[error("Cannot apply unary operator \"{0}\" on type \"{1:?}\"")]
    InvalidUnaryApplication(UnOp, ValueType),

    #[error("Symbol \"{0}\" ({1}) not found")]
    SymbolNotFound(String, String),


    #[error("Symbol \"{0}\" is already defined")]
    SymbolAlreadyDefined(String),
}


