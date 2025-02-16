use element::Element;
use serde::{Deserialize, Serialize};
use texture::Textures;

mod element;
mod manager;
mod texture;

#[derive(Debug, Serialize, Deserialize)]
pub struct Model {
    pub parent: Option<String>,
    pub textures: Textures,
    pub elements: Vec<Element>,
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
        assert_eq!(model.textures.len(), 4);
        assert_eq!(model.elements.len(), 1);
        assert_eq!(model.elements[0].faces.len(), 6);

        Ok(())
    }
}
