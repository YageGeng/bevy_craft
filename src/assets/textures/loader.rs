use std::ops::{Deref, DerefMut};

use bevy::prelude::*;
use bevy_asset_loader::prelude::*;

use crate::assets::prelude::*;

#[derive(AssetCollection, Resource)]
pub struct BlockTextures {
    #[asset(key = "all_block_textures", collection(typed))]
    textures: Vec<Handle<Image>>,
}

impl Deref for BlockTextures {
    type Target = Vec<Handle<Image>>;

    fn deref(&self) -> &Self::Target {
        &self.textures
    }
}

impl DerefMut for BlockTextures {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.textures
    }
}

pub fn pre_texture_load(models: Res<ModelManager>, mut dynamic_assets: ResMut<DynamicAssets>) {
    let all_block_texture = models.all_texture_path();
    dynamic_assets.register_asset(
        "all_block_textures",
        Box::new(StandardDynamicAsset::Files {
            paths: all_block_texture,
        }),
    );
}
