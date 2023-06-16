use anyhow::Result;

use crate::{expr_eval::Value, prelude::InterperterError};

use crate::symbol::*;
use InterperterError::{ImmutableAssignment, SymbolNotFound};

pub(crate) struct Environment<'a> {
    parent: Option<&'a mut Environment<'a>>,
    symbols: SymbolTable,
}

impl<'a> Default for Environment<'a> {
    fn default() -> Self {
        Self::new()
    }
}

impl<'a> Environment<'a> {
    pub fn new() -> Self {
        Self {
            parent: None,
            symbols: Default::default(),
        }
    }

    pub fn define_variable(
        &mut self, name: &str, mutability: Mutability, value: Value
    ) -> Result<(), InterperterError> {
        // We don't care if a parent has that symbol because we want to support shadowing
        self.symbols.insert(name, Symbol {
            mutability,
            kind: SymbolKind::Variable(value),
        })?;
        Ok(())
    }

    pub fn assign_to_variable(
        &mut self, name: &str, new_value: Value
    ) -> Result<(), InterperterError> {
        let result = self.symbols.update(name, |symbol| match &symbol.mutability {
            Mutability::Immutable => Err(ImmutableAssignment(name.to_string())),
            Mutability::Mutable => match &symbol.kind {
                SymbolKind::Variable(..) => {
                    symbol.kind = SymbolKind::Variable(new_value.clone());
                    return Ok(());
                },
            },
        });

        match result {
            Err(SymbolNotFound(..)) => {
                if let Some(parent) = self.parent.as_mut() {
                    return parent.assign_to_variable(name, new_value);
                }
            },
            _ => {},
        };

        Ok(())
    }

    pub fn get_variable_value(&self, name: &str) -> Result<&Value, InterperterError> {
        match self.symbols.get(name) {
            Ok(symbol) => match &symbol.kind {
                SymbolKind::Variable(value) => Ok(&value),
            }
            Err(SymbolNotFound(err)) => {
                // delegate to parent
                if let Some(parent) = &self.parent {
                    return parent.get_variable_value(name);
                } else {
                    // Return the original error
                    return Err(SymbolNotFound(err));
                }
                
            },
            Err(err) => Err(err),
        }
    }
}
