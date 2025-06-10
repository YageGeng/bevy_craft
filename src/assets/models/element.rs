use bevy::{platform::collections::HashMap, prelude::*};
use serde::{Deserialize, Serialize};

use crate::assets::prelude::*;

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
    pub fn faces(&self, face: BlockFace) -> Option<Face<'_>> {
        self.faces.get(&face).map(|data| Face {
            from: self.from,
            to: self.to,
            face,
            data,
        })
    }
}

impl FaceAble for Element {
    fn from(&self) -> [i8; 3] {
        self.from
    }

    fn to(&self) -> [i8; 3] {
        self.to
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

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Axis {
    X,
    Y,
    Z,
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

    pub fn uv(&self, min: [i8; 3], max: [i8; 3]) -> [i8; 4] {
        match self {
            BlockFace::Down => [
                min[0],
                DEFAULT_ELEMENT_SIZE_I8 - max[2],
                max[0],
                DEFAULT_ELEMENT_SIZE_I8 - min[2],
            ],
            BlockFace::Up => [min[0], min[2], max[0], max[2]],
            BlockFace::North => [
                DEFAULT_ELEMENT_SIZE_I8 - max[0],
                DEFAULT_ELEMENT_SIZE_I8 - max[1],
                DEFAULT_ELEMENT_SIZE_I8 - min[0],
                DEFAULT_ELEMENT_SIZE_I8 - min[1],
            ],
            BlockFace::South => [
                min[0],
                DEFAULT_ELEMENT_SIZE_I8 - max[1],
                max[0],
                DEFAULT_ELEMENT_SIZE_I8 - min[1],
            ],
            BlockFace::West => [
                min[2],
                DEFAULT_ELEMENT_SIZE_I8 - max[1],
                max[2],
                DEFAULT_ELEMENT_SIZE_I8 - min[1],
            ],
            BlockFace::East => [
                DEFAULT_ELEMENT_SIZE_I8 - max[2],
                DEFAULT_ELEMENT_SIZE_I8 - max[1],
                DEFAULT_ELEMENT_SIZE_I8 - min[2],
                DEFAULT_ELEMENT_SIZE_I8 - min[1],
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

    pub const fn opposite(&self) -> BlockFace {
        match self {
            BlockFace::Down => BlockFace::Up,
            BlockFace::Up => BlockFace::Down,
            BlockFace::North => BlockFace::South,
            BlockFace::South => BlockFace::North,
            BlockFace::West => BlockFace::East,
            BlockFace::East => BlockFace::West,
        }
    }

    pub const fn is_neg_axis(&self) -> bool {
        match self {
            BlockFace::Down | BlockFace::North | BlockFace::West => true,
            BlockFace::Up | BlockFace::South | BlockFace::East => false,
        }
    }

    pub const fn default_size(&self) -> i8 {
        if self.is_neg_axis() {
            0
        } else {
            DEFAULT_ELEMENT_SIZE_I8
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

impl From<BlockFace> for Axis {
    fn from(value: BlockFace) -> Self {
        match value {
            BlockFace::Down | BlockFace::Up => Axis::Y,
            BlockFace::North | BlockFace::South => Axis::Z,
            BlockFace::West | BlockFace::East => Axis::X,
        }
    }
}

impl From<&BlockFace> for Axis {
    fn from(value: &BlockFace) -> Self {
        (*value).into()
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
