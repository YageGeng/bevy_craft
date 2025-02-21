use bevy::prelude::*;
use bevy_craft::assets::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(AppAssetPlugin)
        .add_systems(OnEnter(AppLoadState::Next), test_atlas)
        .run();
}

fn test_atlas(
    mut commands: Commands,
    atlas: Res<BlockTextureAtlas>,
    asset_server: Res<AssetServer>,
) {
    bevy::log::info!("RENDER");

    let dirt = asset_server.load::<Image>("bevy_craft/textures/block/dirt.png");

    commands.spawn((
        Transform {
            translation: Vec3::new(100.0, 0.0, 0.0),
            scale: Vec3::splat(3.0),
            ..default()
        },
        Sprite::from_atlas_image(
            atlas.atlas.clone(),
            atlas.source.handle(atlas.layout.clone(), &dirt).unwrap(),
        ),
    ));
    commands.spawn(Camera2d);
}
