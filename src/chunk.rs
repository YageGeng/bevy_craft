use bevy::{prelude::*, utils::HashMap};

use crate::{assets::prelude::*, identity::prelude::*};

pub const CHUNK_SIZE: i32 = 16;

#[derive(Component, Default)]
pub struct Chunk {
    /// y is always 0
    pub position: IVec3,
    pub data: HashMap<IVec3, BlockData>,
}

pub struct BlockData {
    pub id: BlockId,
}

impl Chunk {
    pub fn mesh(&self, atlas: &AppTextureAtlas<TextureId>, models: &ModelManager) -> Mesh {
        let vertex = self
            .data
            .iter()
            .flat_map(|(pos, block_data)| {
                models
                    .get(&block_data.id)
                    .and_then(|model| model.vertex(pos + self.position, atlas, self, models))
            })
            .reduce(|mut first, second| {
                first.merge(second);
                first
            })
            .unwrap_or_default();

        Mesh::from(vertex)
    }
}
