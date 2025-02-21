use bevy::prelude::*;
use bevy_asset_loader::prelude::*;

use super::loader::{pre_resolve_models, ModelAssets, ModelLoadState};

pub struct ModelPlugin;

impl Plugin for ModelPlugin {
    fn build(&self, app: &mut bevy::app::App) {
        app.add_loading_state(
            LoadingState::new(ModelLoadState::Loading)
                .continue_to_state(ModelLoadState::Next)
                .load_collection::<ModelAssets>(),
        )
        .add_systems(OnEnter(ModelLoadState::Next), pre_resolve_models);
    }
}
