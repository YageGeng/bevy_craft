use bevy::prelude::*;

use bevy_craft::assets::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(AppAssetPlugin)
        .run();
}
