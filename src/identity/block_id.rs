use super::{Identity, DEFAULT_NAMESPACE};
use crate::models::model::Model;
use bevy::asset::{AssetPath, Handle};
use derive_more::derive::Display;
use std::{borrow::Cow, path::Path};

/// block id
#[derive(Display, Debug, Hash, PartialEq, Eq, Clone)]
#[display("{}:{}", ns, name)]
pub struct BlockId {
    ns: String,
    name: String,
}

impl BlockId {
    pub fn new<T: ToString, U: ToString>(ns: T, name: U) -> Self {
        Self {
            ns: ns.to_string(),
            name: name.to_string(),
        }
    }
}

impl Identity for BlockId {
    fn id(&self) -> Cow<str> {
        Cow::Owned(format!("{}:{}", self.ns, self.name))
    }

    fn ns(&self) -> Cow<str> {
        Cow::Borrowed(&self.ns)
    }
}

impl From<&str> for BlockId {
    // bevy_craft/models/block/grass_block.json
    fn from(path: &str) -> Self {
        // parse for Model.parent
        if path.contains(":") {
            let (ns, name) = path.split_once(":").unwrap();
            BlockId {
                ns: ns.to_string(),
                name: name.to_string(),
            }
        } else {
            match path.split_once("/") {
                Some((ns, remainder)) => {
                    let remainder = remainder.strip_prefix("models/").unwrap_or(remainder);
                    let id = remainder.strip_suffix(".json").unwrap_or(remainder);

                    BlockId {
                        ns: ns.to_string(),
                        name: id.to_string(),
                    }
                }
                None => BlockId {
                    ns: DEFAULT_NAMESPACE.to_string(),
                    name: path.to_string(),
                },
            }
        }
    }
}

impl From<&Path> for BlockId {
    fn from(path: &Path) -> Self {
        let path = path.as_os_str().to_str().unwrap();
        BlockId::from(path)
    }
}

impl From<&AssetPath<'_>> for BlockId {
    fn from(value: &AssetPath<'_>) -> Self {
        BlockId::from(value.path())
    }
}

impl TryFrom<&Handle<Model>> for BlockId {
    type Error = &'static str;

    fn try_from(value: &Handle<Model>) -> Result<Self, Self::Error> {
        let path = value.path().ok_or("model not loaded yet.")?;
        Ok(BlockId::from(path))
    }
}