use bevy::prelude::*;
use bevy_asset_loader::prelude::*;
use derive_more::derive::IntoIterator;

use crate::assets::prelude::*;

#[derive(AssetCollection, Resource, IntoIterator)]
pub struct BlockTextures {
    #[into_iterator(owned, ref, ref_mut)]
    #[asset(key = "all_block_textures", collection(typed))]
    textures: Vec<Handle<Image>>,
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
