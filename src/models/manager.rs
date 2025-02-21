use crate::identity::block_id::BlockId;
use bevy::{log, prelude::*, utils::HashMap};
use topo_sort::TopoSort;

use super::model::Model;

#[derive(Resource, Default)]
pub struct ModelManager {
    pub models: HashMap<BlockId, Model>,
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
}
