use bevy::{prelude::*, utils::HashMap};

use crate::{assets::prelude::*, identity::prelude::*};

pub const CHUNK_SIZE: i32 = 16;

#[derive(Component, Default, Debug)]
pub struct Chunk {
    pub data: HashMap<IVec3, BlockData>,
}

#[derive(Debug)]
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
                    .and_then(|model| model.vertex(*pos, atlas, self, models))
            })
            .reduce(|mut first, second| {
                first.merge(second);
                first
            })
            .unwrap_or_default();

        Mesh::from(vertex)
    }

    pub fn position(&self) -> Option<IVec2> {
        self.data
            .iter()
            .next()
            .map(|(block_pos, _)| block_pos.xz().div_euclid(IVec2::splat(CHUNK_SIZE)))
    }

    pub fn opposite(&self, pos: IVec3, face: BlockFace) -> Option<&BlockData> {
        self.data.get(&(pos + IVec3::from(face)))
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_chunk_pos() {
        let mut chunk = Chunk::default();
        chunk.data.insert(
            IVec3::new(0, 0, 0),
            BlockData {
                id: BlockId(String::new()),
            },
        );
        assert_eq!(chunk.position(), Some(IVec2::new(0, 0)));

        let mut chunk = Chunk::default();
        chunk.data.insert(
            IVec3::new(1, 0, 15),
            BlockData {
                id: BlockId(String::new()),
            },
        );
        assert_eq!(chunk.position(), Some(IVec2::new(0, 0)));

        let mut chunk = Chunk::default();
        chunk.data.insert(
            IVec3::new(16, 0, 16),
            BlockData {
                id: BlockId(String::new()),
            },
        );
        assert_eq!(chunk.position(), Some(IVec2::new(1, 1)));

        let mut chunk = Chunk::default();
        chunk.data.insert(
            IVec3::new(9, 0, 10),
            BlockData {
                id: BlockId(String::new()),
            },
        );
        assert_eq!(chunk.position(), Some(IVec2::new(0, 0)));

        let mut chunk = Chunk::default();
        chunk.data.insert(
            IVec3::new(-9, 0, 10),
            BlockData {
                id: BlockId(String::new()),
            },
        );
        assert_eq!(chunk.position(), Some(IVec2::new(-1, 0)));

        let mut chunk = Chunk::default();
        chunk.data.insert(
            IVec3::new(-16, 0, -10),
            BlockData {
                id: BlockId(String::new()),
            },
        );
        assert_eq!(chunk.position(), Some(IVec2::new(-1, -1)));

        let mut chunk = Chunk::default();
        chunk.data.insert(
            IVec3::new(-17, 0, -17),
            BlockData {
                id: BlockId(String::new()),
            },
        );
        assert_eq!(chunk.position(), Some(IVec2::new(-2, -2)));
    }
}
