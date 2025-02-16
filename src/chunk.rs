use bevy::{math::Vec3, prelude::*, utils::HashMap};

use crate::identity::block_id::BlockId;

pub struct Chunk {
    /// 区块位置
    pub position: Vec3,
    pub data: HashMap<Vec3, BlockData>,
}

pub struct BlockData {
    pub id: BlockId,
}
