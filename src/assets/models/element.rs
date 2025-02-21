use bevy::utils::HashMap;
use serde::{Deserialize, Serialize};

use crate::assets::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Element {
    /// 立方体左下角坐标
    pub from: [u8; 3],
    /// 立方体右上角坐标
    pub to: [u8; 3],
    /// 立方体每个面的定义
    pub faces: HashMap<BlockFace, ElementFace>,
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
}
