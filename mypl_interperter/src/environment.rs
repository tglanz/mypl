use anyhow::Result;
use std::collections::HashMap;

use crate::{expr_eval::Value, prelude::InterperterError};
use InterperterError::{EnvironmentValueNotFound, EnvironmentValueAlreadyExists};

pub(crate) struct Environment {
    data: HashMap<String, Value>, 
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

    pub fn get_value(&self, name: &str) -> Result<&Value, InterperterError> {
        self.data.get(name)
            .ok_or_else(|| EnvironmentValueNotFound(name.to_string()))
    }

    pub fn insert_value(&mut self, name: &str, value: Value) -> Result<(), InterperterError> {
        if self.data.contains_key(name) {
            Err(EnvironmentValueAlreadyExists(name.to_string()))
        } else {
            let res = self.data.insert(name.to_string(), value);
            if res.is_some() {
                // We already checked that the environment doesn't contain that value.
                // If we reached here, it is possible that we introduced concurrency and perhaps have a race condition.
                // For now, we are single threaded and we shouldn't reach
                // here.
                unreachable!();
            }

            Ok(())
        }
    }
}
