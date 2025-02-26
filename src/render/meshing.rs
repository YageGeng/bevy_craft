use bevy::prelude::*;

use crate::{
    assets::prelude::*,
    chunk::{BlockData, Chunk},
    identity::prelude::*,
    render::prelude::*,
};

impl Model {
    pub fn vertex(
        &self,
        pos: IVec3,
        atlas: &AppTextureAtlas<TextureId>,
        chunk: &Chunk,
        models: &ModelManager,
    ) -> Option<Vertex> {
        let element_size = self
            .elements
            .as_ref()
            .map(|elements| elements.len())
            .unwrap_or(0);
        if element_size == 0 {
            return None;
        }

        let mut positions = Vec::new();
        let mut normals = Vec::new();
        let mut uvs = Vec::new();
        let mut indices = Vec::new();

        if let Some(ref elements) = self.elements {
            for element in elements {
                let min = element.min();
                let max = element.max();
                for (face, face_data) in &element.faces {
                    // cullface
                    if let Some(cull_face) = face_data.cullface {
                        let should_cull_face = chunk
                            .data
                            .get(&(pos + IVec3::from(cull_face)))
                            .map(|BlockData { id: block_id, .. }| block_id)
                            .and_then(|block_id| models.get(block_id))
                            .is_some_and(|model| model.should_cull_face(cull_face.opposite()));

                        // TODO: Alpha Face?
                        if should_cull_face {
                            bevy::log::debug!("pos: {:?}, face: {:?}, cullface", pos, face);
                            continue;
                        }
                    };

                    // positions
                    positions.extend(
                        face.vertex(min, max).iter().map(|&[x, y, z]| {
                            [x + pos.x as f32, y + pos.y as f32, z + pos.z as f32]
                        }),
                    );

                    // normals
                    normals.extend([face.normal(); 4]);

                    // indices
                    indices.extend(face.indice(positions.len() as u32 - 4));

                    // uvs
                    let Rect {
                        min: Vec2 { x: u_min, y: v_min },
                        max: Vec2 { x: u_max, y: v_max },
                    } = face_data
                        .texture
                        .reference()
                        .and_then(|key| {
                            self.textures
                                .as_ref()
                                .and_then(|textures| textures.texture_path(key))
                        })
                        .and_then(|texture| TextureId::try_from(texture).ok())
                        .and_then(|texture_id| atlas.uv(texture_id))
                        .unwrap_or(Rect::EMPTY); // NOTE: maybe need an default texture?

                    let [u1, v1, u2, v2] =
                        face_data.uv.unwrap_or(face.uv(element.from, element.to));

                    let u1 = u_min + (u1 as f32 / DEFAULT_ELEMENT_SIZE) * (u_max - u_min);
                    let v1 = v_min + (v1 as f32 / DEFAULT_ELEMENT_SIZE) * (v_max - v_min);
                    let u2 = u_min + (u2 as f32 / DEFAULT_ELEMENT_SIZE) * (u_max - u_min);
                    let v2 = v_min + (v2 as f32 / DEFAULT_ELEMENT_SIZE) * (v_max - v_min);

                    uvs.extend_from_slice(&[[u1, v1], [u2, v1], [u2, v2], [u1, v2]]);
                }
            }
        }

        Some(Vertex {
            positions,
            normals,
            uvs,
            indices,
        })
    }

    pub fn should_cull_face(&self, face: BlockFace) -> bool {
        self.elements
            .as_ref()
            .is_some_and(|element| element.iter().any(|element| element.is_full_face(face)))
    }
}
