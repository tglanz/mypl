use std::result;
use std::error;

pub type Result<T> = result::Result<T, Box<dyn error::Error>>;