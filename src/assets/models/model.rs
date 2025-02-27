use bevy::prelude::*;
use serde::{Deserialize, Serialize};

use crate::assets::prelude::*;

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

    pub fn faces(&self, face: BlockFace) -> Option<Vec<Face<'_>>> {
        self.elements.as_ref().map(|elemtnes| {
            elemtnes
                .iter()
                .flat_map(|element| element.faces(face))
                .collect::<Vec<_>>()
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
