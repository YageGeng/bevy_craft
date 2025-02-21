use bevy::prelude::*;
use bevy_asset_loader::prelude::*;

use super::prelude::*;

pub struct AppAssetPlugin;

#[derive(Clone, Eq, PartialEq, Debug, Hash, Default, States)]
pub enum AppLoadState {
    #[default]
    ModelLoading,
    ModelLoaded,
    TextureLoading,
    TextureLoaded,
    Next,
}

impl Plugin for AppAssetPlugin {
    fn build(&self, app: &mut App) {
        app.init_asset::<Model>()
            .init_state::<AppLoadState>()
            .register_asset_loader(ModelLoader)
            .add_loading_state(
                LoadingState::new(AppLoadState::ModelLoading)
                    .continue_to_state(AppLoadState::ModelLoaded)
                    .load_collection::<ModelAssets>(),
            )
            .add_loading_state(
                LoadingState::new(AppLoadState::TextureLoading)
                    .continue_to_state(AppLoadState::TextureLoaded)
                    .load_collection::<BlockTextures>(),
            )
            .add_systems(OnEnter(AppLoadState::ModelLoaded), resolve_models)
            .add_systems(OnEnter(AppLoadState::TextureLoading), pre_texture_load)
            .add_systems(OnEnter(AppLoadState::TextureLoaded), build_atlas);
    }
}
