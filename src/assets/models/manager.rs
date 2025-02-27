use std::ops::{Deref, DerefMut};

use crate::identity::prelude::*;
use bevy::{log, prelude::*, utils::HashMap};
use topo_sort::TopoSort;

use crate::assets::prelude::*;

#[derive(Resource, Default)]
pub struct ModelManager {
    models: HashMap<BlockId, Model>,
}

impl Deref for ModelManager {
    type Target = HashMap<BlockId, Model>;

    fn deref(&self) -> &Self::Target {
        &self.models
    }
}

impl DerefMut for ModelManager {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.models
    }
}

impl ModelManager {
    pub fn merge(&mut self) {
        // topo sort
        let mut topo = TopoSort::with_capacity(self.models.len());

        for (block_id, model) in &self.models {
            topo.insert(
                block_id.clone(),
                model
                    .parent
                    .as_ref()
                    .and_then(|id| BlockId::try_from(id.as_str()).ok()),
            );
        }

        match topo.into_vec_nodes() {
            topo_sort::SortResults::Full(block_ids) => {
                for block_id in block_ids {
                    let parent_model = self
                        .models
                        .get(&block_id)
                        .and_then(|model| model.parent.as_ref())
                        .and_then(|parent| {
                            BlockId::try_from(parent.as_str())
                                .map_err(|e| log::error!("{}", e))
                                .ok()
                                .and_then(|key| self.models.get(&key))
                        })
                        .cloned();

                    // unwrap is safe
                    if let Some(parent_model) = parent_model {
                        let model = unsafe { self.models.get_mut(&block_id).unwrap_unchecked() };
                        model.merge(parent_model);
                    }
                }
            }
            topo_sort::SortResults::Partial(_) => panic!("unexpected cycle!"),
        }
    }

    pub fn all_texture_path(&self) -> Vec<String> {
        self.models
            .values()
            .flat_map(|model| &model.textures)
            .flat_map(|texture| texture.values())
            .filter_map(|texture| {
                TextureId::try_from(texture)
                    .map_err(|err| log::debug!("{}", err))
                    .ok()
                    .map(|texture_id| texture_id.path())
            })
            .collect()
    }
}
