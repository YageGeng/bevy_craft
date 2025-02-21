use bevy::prelude::*;
use bevy_asset_loader::prelude::*;

use super::prelude::*;

pub struct BCAssetPlugin;

#[derive(Clone, Eq, PartialEq, Debug, Hash, Default, States)]
pub enum BCLoadState {
    #[default]
    ModelLoading,
    ModelLoaded,
    TextureLoading,
    TextureLoaded,
    Next,
}

impl Plugin for BCAssetPlugin {
    fn build(&self, app: &mut App) {
        app.init_asset::<Model>()
            .init_state::<BCLoadState>()
            .register_asset_loader(ModelLoader)
            .add_loading_state(
                LoadingState::new(BCLoadState::ModelLoading)
                    .continue_to_state(BCLoadState::ModelLoaded)
                    .load_collection::<ModelAssets>(),
            )
            .add_systems(OnEnter(BCLoadState::Next), resolve_models)
            // .add_systems(OnEnter(BCLoadState::TextureLoading), ())
            ;
    }
}
