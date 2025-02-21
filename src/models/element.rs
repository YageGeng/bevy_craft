use bevy::utils::HashMap;
use serde::{Deserialize, Serialize};

use super::texture::Texture;

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
    pub fn face_posittion(&self, _face: BlockFace) {
        
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
    Down,
    Up,
    North,
    South,
    West,
    East,
}
