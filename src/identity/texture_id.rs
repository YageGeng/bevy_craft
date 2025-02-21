use std::{path::Path, str::FromStr};

use bevy::prelude::*;
use derive_more::derive::{Display, From};

use crate::assets::prelude::*;
use crate::identity::*;

#[derive(Display, Debug, Hash, PartialEq, Eq, Clone, From)]
#[display("{}", _0)]
pub struct TextureId(pub String);

impl Identity for TextureId {
    const DIR: &str = "textures";

    const EXTENSION: &str = "png";

    fn id(&self) -> &str {
        &self.0
    }
}

impl IdentityExtra for TextureId {
    const _DIR: &str = "/textures/";

    const _EXTENSION: &str = ".png";
}

impl FromStr for TextureId {
    type Err = IdentityError;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        let (ns, remainder) = if let Some((ns, value)) = value.split_once(IDENTITY_DELIMITER) {
            (ns, value)
        } else if let Some((ns, value)) = value.split_once(Self::_DIR) {
            (ns, value)
        } else {
            (DEFAULT_NAMESPACE, value)
        };

        if !remainder.starts_with("block") {
            return Err(IdentityError::TextureIdError(value.to_string()));
        }

        let value = remainder
            .strip_suffix(Self::_EXTENSION)
            .unwrap_or(remainder);

        Ok(Self(format!("{}:{}", ns, value)))
    }
}

impl TryFrom<&Handle<Model>> for TextureId {
    type Error = IdentityError;

    fn try_from(handle: &Handle<Model>) -> Result<Self, Self::Error> {
        let path = handle
            .path()
            .ok_or(IdentityError::TextureIdError(handle.id().to_string()))?
            .path();

        TextureId::try_from(path)
    }
}

impl TryFrom<&Path> for TextureId {
    type Error = IdentityError;

    fn try_from(path: &Path) -> Result<Self, Self::Error> {
        let path = path.to_str().ok_or(IdentityError::TextureIdError(
            path.to_string_lossy().to_string(),
        ))?;

        path.parse()
    }
}

impl TryFrom<&str> for TextureId {
    type Error = IdentityError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        value.parse()
    }
}

impl TryFrom<&Texture> for TextureId {
    type Error = IdentityError;

    fn try_from(texture: &Texture) -> Result<Self, Self::Error> {
        match texture.location() {
            Some(location) => TextureId::try_from(location),
            None => Err(IdentityError::TextureIdError(texture.0.clone())),
        }
    }
}

#[cfg(test)]
mod test {

    use super::*;

    #[test]
    fn test_parse() {
        let block_id = "bevy_craft/textures/block/cube.png".parse::<TextureId>();
        assert!(block_id.is_ok());
        assert_eq!(
            block_id.unwrap(),
            TextureId(String::from("bevy_craft:block/cube"))
        );

        let block_id = "bevy_craft:block/cube".parse::<TextureId>();
        assert!(block_id.is_ok());
        assert_eq!(
            block_id.unwrap(),
            TextureId(String::from("bevy_craft:block/cube"))
        );

        let block_id = "block/cube".parse::<TextureId>();
        assert!(block_id.is_ok());
        assert_eq!(
            block_id.unwrap(),
            TextureId(String::from("bevy_craft:block/cube"))
        );

        let block_id = "bevy_craft:textures/block/cube.png".parse::<TextureId>();
        assert!(block_id.is_err());
        assert_eq!(
            block_id.unwrap_err(),
            IdentityError::TextureIdError(String::from("bevy_craft:textures/block/cube.png"))
        );

        let block_id = "bevy_craft:block/dirt".parse::<TextureId>();
        assert!(block_id.is_ok());
        assert_eq!(
            block_id.unwrap(),
            TextureId(String::from("bevy_craft:block/dirt"))
        );
    }

    #[test]
    fn test_from_texture() {
        let texture = Texture("bevy_craft:block/dirt".to_string());
        let texture_id = TextureId::try_from(&texture);
        assert!(texture_id.is_ok());
        assert_eq!(
            texture_id.unwrap(),
            TextureId("bevy_craft:block/dirt".to_string())
        );

        let texture = Texture("#all".to_string());
        let texture_id = TextureId::try_from(&texture);
        assert!(texture_id.is_err());
        assert_eq!(
            texture_id.unwrap_err(),
            IdentityError::TextureIdError("#all".to_string())
        );

        let texture = Texture("bevy_craft:textures/block/cube.png".to_string());
        let texture_id = TextureId::try_from(&texture);
        assert!(texture_id.is_err());
        assert_eq!(
            texture_id.unwrap_err(),
            IdentityError::TextureIdError("bevy_craft:textures/block/cube.png".to_string())
        );
    }

    #[test]
    fn test_path() {
        let block_id = TextureId("bevy_craft:block/cube".to_string());
        assert_eq!(block_id.path(), "bevy_craft/textures/block/cube.png")
    }
}
