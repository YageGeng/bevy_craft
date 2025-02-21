use bevy::{log, prelude::*};
use bevy_asset_loader::loading_state::config::ConfigureLoadingState;
use bevy_asset_loader::loading_state::{LoadingState, LoadingStateAppExt};
use bevy_craft::models::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .init_asset::<Model>()
        .register_asset_loader(ModelLoader)
        .init_state::<ModelLoadState>()
        .add_loading_state(
            LoadingState::new(ModelLoadState::Loading)
                .continue_to_state(ModelLoadState::Next)
                .load_collection::<ModelAssets>(),
        )
        .add_systems(
            OnEnter(ModelLoadState::Next),
            (pre_resolve_models, resolve_models).chain(),
        )
        .run();
}

fn resolve_models(mut model_manager: ResMut<ModelManager>) {
    log::info!("start merge");
    model_manager.merge();

    log::info!(
        "{}",
        serde_json::to_string_pretty(
            &model_manager
                .models
                .get(&bevy_craft::identity::block_id::BlockId::new(
                    "bevy_craft",
                    "block/dirt"
                ))
                .unwrap()
        )
        .unwrap()
    );
}
