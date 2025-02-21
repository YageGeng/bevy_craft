use std::hash::Hash;

use derive_more::derive::{Display, Error};

pub mod block_id;

pub const IDENTITY_DELIMITER: char = ':';
pub const DEFAULT_NAMESPACE: &str = "bevy_craft";

pub trait Identity: Eq + Hash {
    /// get the id such namespace:name
    fn id(&self) -> &str;

    /// get the namespace
    fn ns(&self) -> &str {
        self.id()
            .find(IDENTITY_DELIMITER)
            .map(|pos| &self.id()[..pos])
            .unwrap_or(DEFAULT_NAMESPACE)
    }

    /// get name
    fn name(&self) -> &str {
        self.id()
            .find(IDENTITY_DELIMITER)
            .map(|pos| &self.id()[pos..])
            .unwrap_or(self.id())
    }
}

#[derive(Debug, Error, Display, PartialEq, Eq)]
pub enum IdentityError {
    #[display("parse BlockId error, {}", _0)]
    BlockIdError(#[error(not(source))] String),
    #[display("parse TextureId error, {}", _0)]
    TextureIdError(#[error(not(source))] String),
}
