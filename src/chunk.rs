use bevy::{prelude::*, utils::HashMap};
use noise::{NoiseFn, Perlin};

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
    /// 使用噪声生成区块
    /// seed: 随机种子，相同种子生成相同地形
    /// scale: 噪声缩放比例，控制地形起伏
    /// height_scale: 高度缩放比例
    pub fn generate_with_noise(seed: u32, scale: f64, height_scale: f64) -> Self {
        let perlin = Perlin::new(seed);
        let mut data = HashMap::new();

        for x in 0..CHUNK_SIZE {
            for z in 0..CHUNK_SIZE {
                // 获取噪声值
                let noise_value = perlin.get([x as f64 / scale, z as f64 / scale]);

                // 将噪声值映射到高度
                let height = ((noise_value + 1.0) * 0.5 * height_scale) as i32;

                // 填充方块
                for y in 0..=height {
                    data.insert(
                        IVec3::new(x, y, z),
                        BlockData {
                            id: BlockId("bevy_craft:block/cherry_stairs".to_string()),
                        },
                    );
                }
            }
        }

        Chunk { data }
    }

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
