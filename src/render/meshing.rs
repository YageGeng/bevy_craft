use bevy::prelude::*;

use crate::{assets::prelude::*, chunk::Chunk, identity::prelude::*, render::prelude::*};

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

                    // TODO: Alpha Face?
                    if should_cull_face(pos, element, *face, face_data, chunk, models) {
                        bevy::log::info!("pos: {:?}, face: {:?}, cullface", pos, face);
                        continue;
                    }

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

                    let u1 = u_min + (u1 as f32 / DEFAULT_ELEMENT_SIZE_F32) * (u_max - u_min);
                    let v1 = v_min + (v1 as f32 / DEFAULT_ELEMENT_SIZE_F32) * (v_max - v_min);
                    let u2 = u_min + (u2 as f32 / DEFAULT_ELEMENT_SIZE_F32) * (u_max - u_min);
                    let v2 = v_min + (v2 as f32 / DEFAULT_ELEMENT_SIZE_F32) * (v_max - v_min);

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

    pub fn faces(&self, face: BlockFace) -> Option<Vec<Face<'_>>> {
        self.elements.as_ref().map(|elemtnes| {
            elemtnes
                .iter()
                .flat_map(|element| element.faces(face))
                .collect::<Vec<_>>()
        })
    }
}

fn should_cull_face(
    pos: IVec3,
    element: &Element,
    face: BlockFace,
    face_data: &ElementFace,
    chunk: &Chunk,
    models: &ModelManager,
) -> bool {
    let Some(cull_face) = face_data.cullface else {
        return false;
    };

    // the face is not normal, then must render
    if !element.is_normal_face(face) {
        return false;
    }

    let Some(opposite_faces) = chunk
        .opposite(pos, cull_face)
        .map(|block_data| &block_data.id)
        .and_then(|block_id| models.get(block_id))
        .and_then(|model| model.faces(face.opposite()))
    else {
        return false;
    };

    let rect = element.rect(face);
    opposite_faces
        .iter()
        .filter(|face| face.is_normal_face(face.face))
        .any(|face| {
            let opposite_rect = face.rect(face.face);
            opposite_rect.contains(rect.min) && opposite_rect.contains(rect.max)
        })
}
