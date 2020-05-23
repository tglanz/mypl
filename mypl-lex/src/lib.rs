extern crate log;
extern crate regex;

mod patterns;
mod source_peeker;

pub mod lexer;
pub mod span;
pub mod tokens;

pub mod prelude {
    pub use super::lexer::*;
    pub use super::span::*;
    pub use super::tokens::*;

    pub(crate) use super::patterns::*;
    pub(crate) use super::source_peeker::*;
}
