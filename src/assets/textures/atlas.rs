use bevy::{image::ImageSampler, prelude::*};

use crate::assets::prelude::*;

#[derive(Resource)]
pub struct BlockTextureAtlas {
    pub atlas: Handle<Image>,
    pub source: TextureAtlasSources,
    pub layout: Handle<TextureAtlasLayout>,
    // hashmap textureid -> Asset<Image>
}

/// run OnEnter(AppLoadeStaet::TextureLoaded)
pub fn build_atlas(
    mut commands: Commands,
    mut textures: ResMut<Assets<Image>>,
    block_texturues: Res<BlockTextures>,
    mut app_state: ResMut<NextState<AppLoadState>>,
    mut texture_atlases: ResMut<Assets<TextureAtlasLayout>>,
) {
    let mut builder = TextureAtlasBuilder::default();

    for handler in block_texturues.into_iter() {
        let Some(texture) = textures.get(handler) else {
            bevy::log::warn!("{:?} did not loaded yet", handler.path());
            continue;
        };
        builder.add_texture(Some(handler.id()), texture);
    }

    let (layout, source, mut atlas_image) = builder.build().unwrap();

    // 最临近采样
    atlas_image.sampler = ImageSampler::nearest();

    let atlas = textures.add(atlas_image);
    let layout = texture_atlases.add(layout);

    commands.insert_resource(BlockTextureAtlas {
        atlas,
        source,
        layout,
    });
    // 加载完成,且图集构建完成
    app_state.set(AppLoadState::Next);
}
