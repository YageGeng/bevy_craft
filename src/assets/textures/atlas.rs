use std::marker::PhantomData;
use std::ops::Index;

use bevy::{image::ImageSampler, prelude::*, sprite::TextureAtlasBuilderError, utils::HashMap};

use crate::assets::prelude::*;
use crate::identity::prelude::*;

pub const DEFAULT_ATLAS_MAX_SIZE: UVec2 = UVec2::splat(2048);

#[derive(Resource)]
pub struct AppTextureAtlas<T: Identity> {
    atlas: Handle<Image>,
    source: TextureAtlasSources,
    layout: TextureAtlasLayout,
    texture_map: HashMap<TextureId, AssetId<Image>>,
    _marker: PhantomData<T>,
}

impl<T: Identity> AppTextureAtlas<T> {
    pub fn uv<U: Into<TextureId>>(&self, texture_id: U) -> Option<URect> {
        self.texture_map
            .get(&texture_id.into())
            .and_then(|asset_id| self.source.texture_ids.get(asset_id))
            .map(|idx| *self.layout.textures.index(*idx))
    }

    pub fn size(&self) -> UVec2 {
        self.layout.size
    }

    pub fn atlas(&self) -> Handle<Image> {
        self.atlas.clone_weak()
    }
}

/// run OnEnter AppLoadeStaet::TextureLoaded
pub fn build_atlas(
    mut commands: Commands,
    mut textures: ResMut<Assets<Image>>,
    block_texturues: Res<BlockTextures>,
    mut app_state: ResMut<NextState<AppLoadState>>,
) {
    let mut builder = TextureAtlasBuilder::default();
    builder.max_size(DEFAULT_ATLAS_MAX_SIZE);
    builder.padding(UVec2::splat(2));

    let mut texture_map: HashMap<TextureId, AssetId<Image>> =
        HashMap::with_capacity(block_texturues.len());

    for handler in block_texturues.iter() {
        let Some(texture) = textures.get(handler) else {
            bevy::log::warn!("{:?} did not loaded yet", handler.path());
            continue;
        };

        if let Ok(texture_id) = TextureId::try_from(handler) {
            let id = handler.id();
            builder.add_texture(Some(id), texture);
            texture_map.insert(texture_id, id);
        } else {
            bevy::log::warn!("{:?} can not be convert to TextureId, amz", handler.path());
        }
    }

    loop {
        match builder.build() {
            Ok((layout, source, mut atlas_image)) => {
                atlas_image.sampler = ImageSampler::nearest();

                commands.insert_resource(AppTextureAtlas {
                    atlas: textures.add(atlas_image),
                    layout,
                    source,
                    texture_map,
                    _marker: PhantomData::<TextureId>,
                });

                app_state.set(AppLoadState::Next);
                break;
            }

            Err(err) => match err {
                TextureAtlasBuilderError::NotEnoughSpace => {
                    bevy::log::warn!("texture atlas is too small, try again");
                    builder.max_size(DEFAULT_ATLAS_MAX_SIZE * 2);
                    continue;
                }
                TextureAtlasBuilderError::WrongFormat => {
                    // NOTE: maybe only log and break?
                    panic!("{}", err);
                }
            },
        };
    }
}
