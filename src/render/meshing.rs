use bevy::prelude::*;

use crate::{assets::prelude::*, chunk::Chunk, identity::prelude::*};

pub fn _chunk_render(
    mut _chunk: Query<(Entity, &Chunk), Changed<Chunk>>,
    _model_manager: Res<ModelManager>,
    _texture_atlas: Res<AppTextureAtlas<BlockId>>,
) {
}
