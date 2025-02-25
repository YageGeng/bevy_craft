use bevy::{prelude::*, utils::HashMap};

use crate::identity::prelude::*;

pub const CHUNK_SIZE: i32 = 16;

#[derive(Component)]
pub struct Chunk {
    /// y is always 0
    pub position: IVec3,
    pub data: HashMap<IVec3, BlockData>,
}

pub struct BlockData {
    pub id: BlockId,
}
