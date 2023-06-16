use anyhow::Result;

use crate::{expr_eval::Value, prelude::InterperterError};

use crate::symbol::*;

pub(crate) struct Environment {
    symbols: SymbolTable,
}

impl Default for Environment {
    fn default() -> Self {
        Self::new()
    }
}

impl Environment {
    pub fn new() -> Self {
        Self {
            symbols: Default::default(),
        }
    }

    pub fn define_variable(
        &mut self, name: &str, mutability: Mutability, value: Value
    ) -> Result<(), InterperterError> {
        self.symbols.insert(name, Symbol {
            mutability,
            kind: SymbolKind::Variable(value),
        })?;
        Ok(())
    }

    pub fn assign_to_variable(
        &mut self, name: &str, new_value: Value
    ) -> Result<(), InterperterError> {
        self.symbols.update(name, |symbol| match &symbol.mutability {
            Mutability::Immutable => Err(InterperterError::ImmutableAssignment(name.to_string())),
            Mutability::Mutable => match &symbol.kind {
                SymbolKind::Variable(..) => {
                    symbol.kind = SymbolKind::Variable(new_value);
                    Ok(())
                },
            },
        })
    }

    pub fn get_variable_value(&self, name: &str) -> Result<&Value, InterperterError> {
        match &self.symbols.get(name)?.kind {
            SymbolKind::Variable(value) => Ok(&value),
            //_ => Err(InterperterError::Generic(format!("\"{}\" is not a variable", name))),
        }
    }
}
