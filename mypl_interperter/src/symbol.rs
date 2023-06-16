use anyhow::Result;
use std::collections::HashMap;

use crate::{expr_eval::Value, prelude::InterperterError};
use InterperterError::{SymbolNotFound, SymbolAlreadyExists};

#[derive(Debug, PartialEq, Clone)]
pub enum Mutability {
    Mutable,
    Immutable,
}

pub enum SymbolKind {
    Variable(Value),
}

pub struct Symbol {
    pub mutability: Mutability,
    pub kind: SymbolKind,

    // We can also add
    // - access modifiers
    // ...
}

pub struct SymbolTable {
    symbols: HashMap<String, Symbol>,
}

impl Default for SymbolTable {
    fn default() -> Self {
        Self::new()
    }
}

impl SymbolTable {

    pub fn new() -> Self {
        Self {
            symbols: Default::default(),
        }
    }

    pub fn get(&self, name: &str) -> Result<&Symbol, InterperterError> {
        self.symbols.get(name)
            .ok_or_else(|| InterperterError::SymbolNotFound(name.to_string()))
    }

    pub fn insert(&mut self, name: &str, symbol: Symbol) -> Result<(), InterperterError> {
        if self.symbols.contains_key(name) {
            Err(SymbolAlreadyExists(name.to_string()))
        } else {
            if self.symbols.insert(name.to_string(), symbol).is_some() {
                // SymbolTable is not Sync or Send
                unreachable!();
            }

            Ok(())
        }
    }

    pub fn update<F>(&mut self, name: &str, update: F) -> Result<(), InterperterError> where
        F: FnOnce(&mut Symbol) -> Result<(), InterperterError>,
    {
        if let Some(symbol) = self.symbols.get_mut(name) {
            Ok(update(symbol)?)
        } else {
            Err(SymbolNotFound(name.to_string()))
        }
    }
}
