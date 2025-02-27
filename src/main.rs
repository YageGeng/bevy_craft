use bevy::{
    pbr::wireframe::{WireframeConfig, WireframePlugin},
    prelude::*,
    render::{
        settings::{Backends, PowerPreference, RenderCreation, WgpuSettings},
        RenderPlugin,
    },
};

use bevy_craft::{assets::prelude::*, chunk::Chunk, identity::prelude::*};

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
        .add_plugins(WireframePlugin)
        .add_plugins(AppAssetPlugin)
        .add_systems(OnEnter(AppLoadState::Next), render_dirt)
        .add_systems(Update, input_handler)
        .add_systems(Update, toggle_wireframe)
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
    // let mut chunk = Chunk::default();
    // chunk.data.insert(
    //     IVec3::new(0, 0, 0),
    //     BlockData {
    //         id: BlockId("bevy_craft:block/cherry_stairs".to_string()),
    //     },
    // );
    // chunk.data.insert(
    //     IVec3::new(0, 1, 0),
    //     BlockData {
    //         id: BlockId("bevy_craft:block/cherry_stairs".to_string()),
    //     },
    // );
    // chunk.data.insert(
    //     IVec3::new(1, 0, 0),
    //     BlockData {
    //         id: BlockId("bevy_craft:block/cherry_stairs".to_string()),
    //     },
    // );
    // chunk.data.insert(
    //     IVec3::new(0, 0, 1),
    //     BlockData {
    //         id: BlockId("bevy_craft:block/cherry_stairs".to_string()),
    //     },
    // );
    // chunk.data.insert(
    //     IVec3::new(2, 0, 1),
    //     BlockData {
    //         id: BlockId("bevy_craft:block/cherry_stairs".to_string()),
    //     },
    // );
    //
    let chunk = Chunk::generate_with_noise(1234, 0.1, 0.1);
    let mesh = chunk.mesh(&atlas, &models);

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
        Transform::from_xyz(1.8, 4.0, 1.8).looking_at(Vec3::ZERO, Vec3::Y);

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

fn toggle_wireframe(mut config: ResMut<WireframeConfig>, keys: Res<ButtonInput<KeyCode>>) {
    if keys.just_pressed(KeyCode::F3) {
        config.global = !config.global; // 按 F3 切换
    }
}
