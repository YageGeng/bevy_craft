use std::{path::Path, str::FromStr};

use bevy::prelude::*;
use derive_more::derive::{Display, From};

use super::{Identity, IdentityError};
use crate::{identity::*, models::model::Model};

/// block id
/// a block_id must be namespace:block/name
/// a block_id can be Into a path namespace/models/block/name.json
#[derive(Display, Debug, Hash, PartialEq, Eq, Clone, From)]
#[display("{}", _0)]
pub struct BlockId(pub String);

impl Identity for BlockId {
    fn id(&self) -> &str {
        &self.0
    }
}

impl FromStr for BlockId {
    type Err = IdentityError;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        let (ns, remainder) = if let Some((ns, value)) = value.split_once(IDENTITY_DELIMITER) {
            (ns, value)
        } else if let Some((ns, value)) = value.split_once("/models/") {
            (ns, value)
        } else {
            (DEFAULT_NAMESPACE, value)
        };

        if !remainder.starts_with("block") {
            return Err(IdentityError::BlockIdError(value.to_string()));
        }

        let value = remainder.strip_suffix(".json").unwrap_or(remainder);

        Ok(Self(format!("{}:{}", ns, value)))
    }
}

impl TryFrom<&Handle<Model>> for BlockId {
    type Error = IdentityError;

    fn try_from(handle: &Handle<Model>) -> Result<Self, Self::Error> {
        let path = handle
            .path()
            .ok_or(IdentityError::BlockIdError(handle.id().to_string()))?
            .path();

        BlockId::try_from(path)
    }
}

impl TryFrom<&Path> for BlockId {
    type Error = IdentityError;

    fn try_from(path: &Path) -> Result<Self, Self::Error> {
        let path = path.to_str().ok_or(IdentityError::BlockIdError(
            path.to_string_lossy().to_string(),
        ))?;

        path.parse()
    }
}

impl TryFrom<&str> for BlockId {
    type Error = IdentityError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        value.parse()
    }
}

#[cfg(test)]
mod test {

    use super::*;

    #[test]
    fn test_parse() {
        let block_id = "bevy_craft/models/block/cube.json".parse::<BlockId>();
        assert!(block_id.is_ok());
        assert_eq!(
            block_id.unwrap(),
            BlockId(String::from("bevy_craft:block/cube"))
        );

        let block_id = "bevy_craft:block/cube".parse::<BlockId>();
        assert!(block_id.is_ok());
        assert_eq!(
            block_id.unwrap(),
            BlockId(String::from("bevy_craft:block/cube"))
        );

        let block_id = "block/cube".parse::<BlockId>();
        assert!(block_id.is_ok());
        assert_eq!(
            block_id.unwrap(),
            BlockId(String::from("bevy_craft:block/cube"))
        );

        let block_id = "bevy_craft:models/block/cube.json".parse::<BlockId>();
        assert!(block_id.is_err());
        assert_eq!(
            block_id.unwrap_err(),
            IdentityError::BlockIdError(String::from("bevy_craft:models/block/cube.json"))
        );
    }
}
