use bevy::prelude::*;
use serde::{Deserialize, Serialize};

use crate::{assets::prelude::*, identity::prelude::*, render::voxel::Vertex};

#[derive(Debug, Clone, Serialize, Deserialize, Asset, TypePath)]
pub struct Model {
    pub parent: Option<String>,
    pub textures: Option<Textures>,
    pub elements: Option<Vec<Element>>,
}

impl Model {
    pub fn merge(&mut self, other: Self) {
        self.parent = other.parent;

        if let Some(other_texture) = other.textures {
            match &mut self.textures {
                Some(texture) => {
                    texture.merge(other_texture);
                }
                None => self.textures = Some(other_texture),
            }
        }

        if let Some(other_element) = other.elements {
            match &mut self.elements {
                Some(element) => {
                    element.extend(other_element);
                }
                None => self.elements = Some(other_element),
            }
        }
    }

    pub fn vertex(&self, pos: Vec3, atlas: &AppTextureAtlas<TextureId>) -> Option<Vertex> {
        let element_size = self
            .elements
            .as_ref()
            .map(|elements| elements.len())
            .unwrap_or(0);
        if element_size == 0 {
            return None;
        }

        let (pos_x, pos_y, pos_z) = (pos.x, pos.y, pos.z);

        let mut positions = Vec::new();
        let mut normals = Vec::new();
        let mut uvs = Vec::new();
        let mut indices = Vec::new();

        if let Some(ref elements) = self.elements {
            for element in elements {
                let min = element.min();
                let max = element.max();
                for (face, face_data) in &element.faces {
                    let mut position = face.vertex(min, max);

                    position.iter_mut().for_each(|[x, y, z]| {
                        *x += pos_x;
                        *y += pos_y;
                        *z += pos_z
                    });
                    positions.extend(position);

                    normals.extend([face.normal(); 4]);

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

                    let offset = positions.len() as u32 - 4;
                    indices.extend(face.indice(offset));
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
}

#[cfg(test)]
mod test {
    use std::error::Error;

    use super::*;
    use serde_json::json;

    #[test]
    fn test_model_parse() -> Result<(), Box<dyn Error>> {
        let grass = json!(
        {
          "textures": {
              "particle": "block/dirt",
              "bottom": "block/dirt",
              "top": "block/grass_block_top",
              "side": "block/grass_block_side",
          },
          "elements": [
            {
              "from": [ 0, 0, 0 ],
              "to": [ 16, 16, 16 ],
              "faces": {
                  "down":  { "uv": [ 0, 0, 16, 16 ], "texture": "#bottom", "cullface": "down" },
                  "up":    { "uv": [ 0, 0, 16, 16 ], "texture": "#top",    "cullface": "up", "tintindex": 0 },
                  "north": { "uv": [ 0, 0, 16, 16 ], "texture": "#side",   "cullface": "north" },
                  "south": { "uv": [ 0, 0, 16, 16 ], "texture": "#side",   "cullface": "south" },
                  "west":  { "uv": [ 0, 0, 16, 16 ], "texture": "#side",   "cullface": "west" },
                  "east":  { "uv": [ 0, 0, 16, 16 ], "texture": "#side",   "cullface": "east" }
              }
            }
          ]
        });

        let model = serde_json::from_value::<Model>(grass)?;

        assert_eq!(model.parent, None);
        assert!(model.textures.is_some());
        assert_eq!(model.textures.unwrap().len(), 4);
        assert!(model.elements.is_some());
        assert_eq!(model.elements.unwrap().len(), 1);
        Ok(())
    }
}
