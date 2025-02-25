use bevy::{
    prelude::*,
    render::{
        settings::{Backends, PowerPreference, RenderCreation, WgpuSettings},
        RenderPlugin,
    },
};

use bevy_craft::{assets::prelude::*, identity::prelude::*};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(RenderPlugin {
            render_creation: RenderCreation::Automatic(WgpuSettings {
                backends: Some(Backends::VULKAN),
                power_preference: PowerPreference::LowPower,
                ..Default::default()
            }),
            ..Default::default()
        }))
        .add_plugins(AppAssetPlugin)
        .add_systems(OnEnter(AppLoadState::Next), render_dirt)
        .add_systems(Update, input_handler)
        .run();
}
#[derive(Component)]
struct CustomUV;

fn render_dirt(
    mut commands: Commands,
    models: Res<ModelManager>,
    atlas: Res<AppTextureAtlas<TextureId>>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let model = models
        .get(&BlockId("bevy_craft:block/grass_block".to_string()))
        .unwrap();

    let vertex = model.vertex(Vec3::splat(-0.5), &atlas).unwrap();

    let mesh = vertex.build();

    commands.spawn((
        Mesh3d(meshes.add(mesh)),
        MeshMaterial3d(materials.add(StandardMaterial {
            base_color_texture: Some(atlas.atlas()),
            ..default()
        })),
        CustomUV,
    ));
    // Transform for the camera and lighting, looking at (0,0,0) (the position of the mesh).
    let camera_and_light_transform =
        Transform::from_xyz(1.8, 1.8, 1.8).looking_at(Vec3::ZERO, Vec3::Y);

    // Camera in 3D space.
    commands.spawn((Camera3d::default(), camera_and_light_transform));

    // Light up the scene.
    commands.spawn((PointLight::default(), camera_and_light_transform));
}

fn input_handler(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut query: Query<&mut Transform, With<CustomUV>>,
    time: Res<Time>,
) {
    if keyboard_input.pressed(KeyCode::KeyX) {
        for mut transform in &mut query {
            transform.rotate_x(time.delta_secs() / 1.2);
        }
    }
    if keyboard_input.pressed(KeyCode::KeyY) {
        for mut transform in &mut query {
            transform.rotate_y(time.delta_secs() / 1.2);
        }
    }
    if keyboard_input.pressed(KeyCode::KeyZ) {
        for mut transform in &mut query {
            transform.rotate_z(time.delta_secs() / 1.2);
        }
    }
    if keyboard_input.pressed(KeyCode::KeyR) {
        for mut transform in &mut query {
            transform.look_to(Vec3::NEG_Z, Vec3::Y);
        }
    }
}
