use anyhow::Result;
use thiserror::Error;

use mypl_ast::prelude::*;

#[derive(Error, Debug)]
pub enum ParseError {
    #[error("ParseError: {0}")]
    Default(String),

    #[error("Expected token \"{0}\" at \"{1}\"")]
    ExpectedToken(String, String),
}

pub trait Parser {
    fn parse(&mut self) -> Result<Vec<Stmt>, ParseError>;
}
