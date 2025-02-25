use crate::identity::block_id::BlockId;
use bevy::asset::{io::Reader, AssetLoader, LoadContext};
use bevy::log;
use bevy::prelude::*;
use bevy_asset_loader::asset_collection::AssetCollection;
use derive_more::derive::{Display, Error, From, IntoIterator};

use crate::assets::prelude::*;

pub const BLOCK_MODELS_PATH: &str = "bevy_craft/models/block";

pub struct ModelLoader;

#[derive(Debug, Error, From, Display)]
pub enum ModelLoadError {
    #[display("Failed to load model: {}", _0)]
    Io(std::io::Error),
    #[display("Model Syntax Error: {}", _0)]
    JsonError(serde_json::Error),
}

impl AssetLoader for ModelLoader {
    type Asset = Model;
    type Settings = ();
    type Error = ModelLoadError;

    async fn load(
        &self,
        reader: &mut dyn Reader,
        _settings: &Self::Settings,
        _load_context: &mut LoadContext<'_>,
    ) -> Result<Self::Asset, Self::Error> {
        let mut bytes = Vec::<u8>::new();
        reader.read_to_end(&mut bytes).await?;
        Ok(serde_json::from_slice::<Model>(&bytes)?)
    }

    fn extensions(&self) -> &[&str] {
        &["json"]
    }
}

#[derive(Debug, AssetCollection, Resource, IntoIterator)]
pub struct ModelAssets {
    #[into_iterator(owned, ref, ref_mut)]
    #[asset(path = "bevy_craft/models/block", collection(typed))]
    models: Vec<Handle<Model>>,
}

pub fn resolve_models(
    mut commands: Commands,
    model_handles: ResMut<ModelAssets>,
    mut models_assets: ResMut<Assets<Model>>,
    mut next_state: ResMut<NextState<AppLoadState>>,
) {
    let mut result = ModelManager::default();
    model_handles.into_iter().for_each(|handle| {
        if let Some(model) = models_assets.remove(handle) {
            // unwrap is safe
            let block_id = BlockId::try_from(handle).unwrap();
            result.insert(block_id, model);
        } else {
            log::error!("{:?} not loaded yet", handle);
        }
    });

    result.merge();
    commands.remove_resource::<ModelAssets>();
    commands.insert_resource(result);
    next_state.set(AppLoadState::TextureLoading);
}
