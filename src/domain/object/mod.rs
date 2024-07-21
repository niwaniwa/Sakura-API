use std::marker::PhantomData;

use serde::{Deserialize, Deserializer, Serialize};

pub mod account;
pub mod auth;
pub mod card;
pub mod door;
pub mod mqtt;

#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq, Serialize)]
pub struct Id<T> {
    id: i64,
    _phantom: PhantomData<T>,
}

impl<T> Id<T> {
    pub fn new(id: i64) -> Self {
        Self {
            id,
            _phantom: PhantomData,
        }
    }
    pub fn get(&self) -> i64 {
        self.id
    }
}

impl<T> Default for Id<T> {
    fn default() -> Self {
        Id::new(0)
    }
}

impl<'de, T> Deserialize<'de> for Id<T> {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let id = i64::deserialize(deserializer)?;
        Ok(Id::new(id))
    }
}
