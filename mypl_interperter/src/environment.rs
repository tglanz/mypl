use anyhow::Result;
use std::collections::HashMap;

use crate::{expr_eval::Value, prelude::InterperterError};
use InterperterError::{EnvironmentValueNotFound, EnvironmentValueAlreadyExists};

#[derive(Debug, PartialEq, Clone)]
pub enum Mutability {
    Mutable,
    Immutable,
}

pub enum Entity {
    Variable(Mutability, Value),
}

pub(crate) struct Environment {
    data: HashMap<String, Entity>, 
}

impl Default for Environment {
    fn default() -> Self {
        Self::new()
    }
}

impl Environment {
    pub fn new() -> Self {
        Self {
            data: Default::default(),
        }
    }

    pub fn get_entity(&self, name: &str) -> Result<&Entity, InterperterError> {
        self.data.get(name)
            .ok_or_else(|| EnvironmentValueNotFound(name.to_string()))
    }

    pub fn insert_entity(&mut self, name: &str, entity: Entity) -> Result<(), InterperterError> {
        if self.data.contains_key(name) {
            Err(EnvironmentValueAlreadyExists(name.to_string()))
        } else {
            if self.data.insert(name.to_string(), entity).is_some() {
                // We already checked that the environment doesn't contain that value.
                // If we reached here, it is possible that we introduced concurrency and perhaps have a race condition.
                // For now, we are single threaded and we shouldn't reach
                // here.
                unreachable!();
            }

            Ok(())
        }
    }

    pub fn update_entity<F>(&mut self, name: &str, update: F) -> Result<(), InterperterError> where
        F: FnOnce(&mut Entity) -> Result<(), InterperterError>,
    {
        if let Some(entity) = self.data.get_mut(name) {
            Ok(update(entity)?)
        } else {
            Err(EnvironmentValueNotFound(name.to_string()))
        }
    }

    pub fn define_variable(
        &mut self, name: &str, mutability: Mutability, value: Value
    ) -> Result<(), InterperterError> {
        self.insert_entity(name, Entity::Variable(mutability, value))?;
        Ok(())
    }

    pub fn assign_to_variable(
        &mut self, name: &str, new_value: Value
    ) -> Result<(), InterperterError> {
        self.update_entity(name, |entity| match entity {
            Entity::Variable(mutability, value) => match mutability {
                    Mutability::Mutable => {
                        *value = new_value;
                        Ok(())
                    },
                    Mutability::Immutable =>  {
                        Err(InterperterError::ImmutableAssignment(name.to_string()))
                    }
                }
            },
        )
    }

    pub fn get_variable_value(&self, name: &str) -> Result<&Value, InterperterError> {
        match self.get_entity(name)? {
            Entity::Variable(_, value) => Ok(value),
            //_ => Err(InterperterError::Generic(format!("\"{}\" is not a variable", name))),
        }
    }
}
