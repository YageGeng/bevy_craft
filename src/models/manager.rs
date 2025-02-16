use bevy::{prelude::*, utils::hashbrown::HashMap};

use crate::identity::Identity;

use super::Model;

#[derive(Resource)]
pub struct ModelManager<T: Identity> {
    pub models: HashMap<T, Model>,
}

impl<T: Identity> ModelManager<T> {
    pub fn load(asset_server: Res<AssetServer>) -> Self {
        todo!()
    }

    pub fn resolve(&mut self) {
        todo!()
    }
}
