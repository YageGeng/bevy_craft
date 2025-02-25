use bevy::prelude::*;

use bevy_craft::{assets::prelude::*, identity::prelude::*};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(AppAssetPlugin)
        .add_systems(OnEnter(AppLoadState::Next), render_dirt)
        .run();
}

fn render_dirt(
    mut commands: Commands,
    models: Res<ModelManager>,
    atlas: Res<AppTextureAtlas<TextureId>>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let model = models
        .get(&BlockId("bevy_craft:block/dirt".to_string()))
        .unwrap();

    let vertex = model.vertex(Vec3::splat(0.0), &atlas).unwrap();

    let mesh = vertex.build();

    commands.spawn((
        Mesh3d(meshes.add(mesh)),
        MeshMaterial3d(materials.add(StandardMaterial {
            base_color_texture: Some(atlas.atlas()),
            ..default()
        })),
    ));
    // Transform for the camera and lighting, looking at (0,0,0) (the position of the mesh).
    let camera_and_light_transform =
        Transform::from_xyz(1.8, 1.8, 1.8).looking_at(Vec3::ZERO, Vec3::Y);

    // Camera in 3D space.
    commands.spawn((Camera3d::default(), camera_and_light_transform));

    // Light up the scene.
    commands.spawn((PointLight::default(), camera_and_light_transform));
}
