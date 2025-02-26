use bevy::{math::I8Vec3, prelude::*, utils::HashMap};
use serde::{Deserialize, Serialize};

use crate::assets::prelude::*;

pub const DEFAULT_ELEMENT_SIZE: f32 = 16.0;
pub const FULL_OFFSET: i8 = 8;
pub const BLOCK_CENTER: [i8; 3] = [8, 8, 8];
const DEFAULT_TEXTURE_SIZE: i8 = 16;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Element {
    /// 立方体左下角坐标
    pub from: [i8; 3],
    /// 立方体右上角坐标
    pub to: [i8; 3],
    /// 立方体每个面的定义
    pub faces: HashMap<BlockFace, ElementFace>,
}

impl Element {
    pub const fn block_center() -> I8Vec3 {
        I8Vec3::from_array(BLOCK_CENTER)
    }

    pub fn min(&self) -> Vec3 {
        Vec3::new(
            self.from[0] as f32 / DEFAULT_ELEMENT_SIZE,
            self.from[1] as f32 / DEFAULT_ELEMENT_SIZE,
            self.from[2] as f32 / DEFAULT_ELEMENT_SIZE,
        )
    }

    pub fn max(&self) -> Vec3 {
        Vec3::new(
            self.to[0] as f32 / DEFAULT_ELEMENT_SIZE,
            self.to[1] as f32 / DEFAULT_ELEMENT_SIZE,
            self.to[2] as f32 / DEFAULT_ELEMENT_SIZE,
        )
    }

    pub fn is_full_face(&self, face: BlockFace) -> bool {
        let center = Self::block_center();
        let min_offset = center - I8Vec3::from_array(self.from);
        let max_offset = I8Vec3::from_array(self.to) - center;

        match face {
            BlockFace::Down => {
                min_offset.y == FULL_OFFSET
                    && min_offset.x == FULL_OFFSET
                    && min_offset.z == FULL_OFFSET
                    && max_offset.x == FULL_OFFSET
                    && max_offset.z == FULL_OFFSET
            }
            BlockFace::Up => {
                min_offset.x == FULL_OFFSET
                    && min_offset.z == FULL_OFFSET
                    && max_offset.x == FULL_OFFSET
                    && max_offset.y == FULL_OFFSET
                    && max_offset.z == FULL_OFFSET
            }
            BlockFace::North => {
                min_offset.x == FULL_OFFSET
                    && min_offset.y == FULL_OFFSET
                    && min_offset.z == FULL_OFFSET
                    && max_offset.x == FULL_OFFSET
                    && max_offset.y == FULL_OFFSET
            }
            BlockFace::South => {
                min_offset.x == FULL_OFFSET
                    && min_offset.y == FULL_OFFSET
                    && max_offset.x == FULL_OFFSET
                    && max_offset.y == FULL_OFFSET
                    && max_offset.z == FULL_OFFSET
            }
            BlockFace::West => {
                min_offset.x == FULL_OFFSET
                    && min_offset.y == FULL_OFFSET
                    && min_offset.z == FULL_OFFSET
                    && max_offset.y == FULL_OFFSET
                    && max_offset.z == FULL_OFFSET
            }
            BlockFace::East => {
                min_offset.y == FULL_OFFSET
                    && min_offset.z == FULL_OFFSET
                    && max_offset.x == FULL_OFFSET
                    && max_offset.y == FULL_OFFSET
                    && max_offset.z == FULL_OFFSET
            }
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ElementFace {
    /// uv坐标
    pub uv: Option<[i8; 4]>,
    /// 表面剔除方向
    pub cullface: Option<BlockFace>,
    /// 材质标签
    pub texture: Texture,
}

#[derive(Deserialize, Serialize, Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[serde(rename_all = "lowercase")]
#[repr(u8)]
pub enum BlockFace {
    #[serde(alias = "bottom")]
    Down, // -Y
    Up,    // +Y
    North, // -Z
    South, // +Z
    West,  // -X
    East,  // +X
}

impl BlockFace {
    #[rustfmt::skip]
    pub const fn normal(&self) -> [f32; 3] {
        match self {
            BlockFace::Down  => [0.0, -1.0, 0.0],   // -Y
            BlockFace::Up    => [0.0, 1.0, 0.0],    // +Y
            BlockFace::North => [0.0, 0.0, -1.0],   // -Z
            BlockFace::South => [0.0, 0.0, 1.0],    // +Z
            BlockFace::West  => [-1.0, 0.0, 0.0],   // -X
            BlockFace::East  => [1.0, 0.0, 0.0],    // +X
        }
    }

    pub fn vertex(&self, min: Vec3, max: Vec3) -> [[f32; 3]; 4] {
        crate::block_vertex!(self, min, max)
    }

    pub fn vertex_i8(&self, min: [i8; 3], max: [i8; 3]) -> [[i8; 3]; 4] {
        crate::block_vertex!(self, min, max)
    }

    pub fn uv(&self, min: [i8; 3], max: [i8; 3]) -> [i8; 4] {
        match self {
            BlockFace::Down => [
                min[0],
                DEFAULT_TEXTURE_SIZE - max[2],
                max[0],
                DEFAULT_TEXTURE_SIZE - min[2],
            ],
            BlockFace::Up => [min[0], min[2], max[0], max[2]],
            BlockFace::North => [
                DEFAULT_TEXTURE_SIZE - max[0],
                DEFAULT_TEXTURE_SIZE - max[1],
                DEFAULT_TEXTURE_SIZE - min[0],
                DEFAULT_TEXTURE_SIZE - min[1],
            ],
            BlockFace::South => [
                min[0],
                DEFAULT_TEXTURE_SIZE - max[1],
                max[0],
                DEFAULT_TEXTURE_SIZE - min[1],
            ],
            BlockFace::West => [
                min[2],
                DEFAULT_TEXTURE_SIZE - max[1],
                max[2],
                DEFAULT_TEXTURE_SIZE - min[1],
            ],
            BlockFace::East => [
                DEFAULT_TEXTURE_SIZE - max[2],
                DEFAULT_TEXTURE_SIZE - max[1],
                DEFAULT_TEXTURE_SIZE - min[2],
                DEFAULT_TEXTURE_SIZE - min[1],
            ],
        }
    }

    pub fn indice(&self, offset: u32) -> [u32; 6] {
        match self {
            BlockFace::Down => [
                offset,
                offset + 1,
                offset + 3,
                offset + 1,
                offset + 2,
                offset + 3,
            ],
            _ => [
                offset,
                offset + 3,
                offset + 1,
                offset + 1,
                offset + 3,
                offset + 2,
            ],
        }
    }

    pub fn opposite(&self) -> BlockFace {
        match self {
            BlockFace::Down => BlockFace::Up,
            BlockFace::Up => BlockFace::Down,
            BlockFace::North => BlockFace::South,
            BlockFace::South => BlockFace::North,
            BlockFace::West => BlockFace::East,
            BlockFace::East => BlockFace::West,
        }
    }
}

impl From<BlockFace> for IVec3 {
    fn from(value: BlockFace) -> Self {
        match value {
            BlockFace::Down => IVec3::NEG_Y,
            BlockFace::Up => IVec3::Y,
            BlockFace::North => IVec3::NEG_Z,
            BlockFace::South => IVec3::Z,
            BlockFace::West => IVec3::NEG_X,
            BlockFace::East => IVec3::X,
        }
    }
}

#[macro_export]
macro_rules! block_vertex {
    ($face:expr, $min:expr, $max:expr) => {
        match $face {
            BlockFace::Down => [
                [$min[0], $min[1], $min[2]],
                [$max[0], $min[1], $min[2]],
                [$max[0], $min[1], $max[2]],
                [$min[0], $min[1], $max[2]],
            ],
            BlockFace::Up => [
                [$min[0], $max[1], $min[2]],
                [$max[0], $max[1], $min[2]],
                [$max[0], $max[1], $max[2]],
                [$min[0], $max[1], $max[2]],
            ],
            BlockFace::North => [
                [$max[0], $max[1], $min[2]],
                [$min[0], $max[1], $min[2]],
                [$min[0], $min[1], $min[2]],
                [$max[0], $min[1], $min[2]],
            ],
            BlockFace::South => [
                [$min[0], $max[1], $max[2]],
                [$max[0], $max[1], $max[2]],
                [$max[0], $min[1], $max[2]],
                [$min[0], $min[1], $max[2]],
            ],
            BlockFace::West => [
                [$min[0], $max[1], $min[2]],
                [$min[0], $max[1], $max[2]],
                [$min[0], $min[1], $max[2]],
                [$min[0], $min[1], $min[2]],
            ],
            BlockFace::East => [
                [$max[0], $max[1], $max[2]],
                [$max[0], $max[1], $min[2]],
                [$max[0], $min[1], $min[2]],
                [$max[0], $min[1], $max[2]],
            ],
        }
    };
}
