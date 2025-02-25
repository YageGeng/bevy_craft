use bevy::{prelude::*, utils::HashMap};
use serde::{Deserialize, Serialize};

use crate::assets::prelude::*;

pub const DEFAULT_ELEMENT_SIZE: f32 = 16.0;
const DEFAULT_TEXTURE_SIZE: u8 = 16;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Element {
    /// 立方体左下角坐标
    pub from: [u8; 3],
    /// 立方体右上角坐标
    pub to: [u8; 3],
    /// 立方体每个面的定义
    pub faces: HashMap<BlockFace, ElementFace>,
}

impl Element {
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
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ElementFace {
    /// uv坐标
    pub uv: Option<[u8; 4]>,
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
        match self {
            BlockFace::Down => [
                [min.x, min.y, min.z],
                [max.x, min.y, min.z],
                [max.x, min.y, max.z],
                [min.x, min.y, max.z],
            ],
            BlockFace::Up => [
                [min.x, max.y, min.z],
                [max.x, max.y, min.z],
                [max.x, max.y, max.z],
                [min.x, max.y, max.z],
            ],
            BlockFace::North => [
                [max.x, max.y, min.z],
                [min.x, max.y, min.z],
                [min.x, min.y, min.z],
                [max.x, min.y, min.z],
            ],
            BlockFace::South => [
                [min.x, min.y, max.z],
                [min.x, max.y, max.z],
                [max.x, max.y, max.z],
                [max.x, min.y, max.z],
            ],
            BlockFace::West => [
                [min.x, max.y, min.z],
                [min.x, max.y, max.z],
                [min.x, min.y, max.z],
                [min.x, min.y, min.z],
            ],
            BlockFace::East => [
                [max.x, max.y, max.z],
                [max.x, max.y, min.z],
                [max.x, min.y, min.z],
                [max.x, min.y, max.z],
            ],
        }
    }

    pub fn uv(&self, min: [u8; 3], max: [u8; 3]) -> [u8; 4] {
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
        [
            offset,
            offset + 3,
            offset + 1,
            offset + 1,
            offset + 3,
            offset + 2,
        ]
    }
}
