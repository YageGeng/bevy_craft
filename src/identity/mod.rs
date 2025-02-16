use std::{borrow::Cow, hash::Hash};

pub mod block_id;

pub const IDENTITY_DELIMITER: char = ':';
pub const DEFAULT_NAMESPACE: &str = "bevy_craft";

pub trait Identity: Eq + Hash {
    /// get the id such namespace:name
    fn id(&self) -> Cow<str>;
    /// get the namespace
    fn ns(&self) -> Cow<str>;
}
