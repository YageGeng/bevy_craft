use std::hash::Hash;

use derive_more::derive::{Display, Error};

pub mod block_id;
pub mod texture_id;

pub const IDENTITY_DELIMITER: char = ':';
pub const DEFAULT_NAMESPACE: &str = "bevy_craft";

pub trait Identity: Eq + Hash {
    const DIR: &str;
    const EXTENSION: &str;

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
            .map(|pos| &self.id()[pos + 1..])
            .unwrap_or(self.id())
    }

    fn path(&self) -> String {
        format!(
            "{}/{}/{}.{}",
            self.ns(),
            Self::DIR,
            self.name(),
            Self::EXTENSION
        )
    }
}

pub(crate) trait IdentityExtra: Identity {
    const _DIR: &str;
    const _EXTENSION: &str;
}

#[derive(Debug, Error, Display, PartialEq, Eq)]
pub enum IdentityError {
    #[display("parse BlockId{{ {} }} error", _0)]
    BlockIdError(#[error(not(source))] String),
    #[display("parse TextureId{{ {} }} error", _0)]
    TextureIdError(#[error(not(source))] String),
}
