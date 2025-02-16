use std::ops::{Deref, DerefMut};

use bevy::utils::HashMap;
use serde::{Deserialize, Serialize};

const MODEL_TEXTURE_TAG: char = '#';

#[derive(Debug, Serialize, Deserialize)]
pub struct Textures {
    #[serde(flatten)]
    pub variables: HashMap<String, Texture>,
}

impl Textures {
    pub fn resolve(&mut self, other: &Self) {
        for texture in self.variables.values_mut() {
            if let Some(substitution) = texture.resolve(other) {
                *texture = Texture::from(substitution);
            }
        }
    }

    pub fn merge(&mut self, other: Self) {
        for (name, texture) in other.variables.into_iter() {
            //println!("inserting: {:?}", (&name, &texture));
            self.insert(name, texture);
        }
    }
}

impl Deref for Textures {
    type Target = HashMap<String, Texture>;

    fn deref(&self) -> &Self::Target {
        &self.variables
    }
}

impl DerefMut for Textures {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.variables
    }
}

#[derive(Deserialize, Serialize, Debug, Default, Clone, PartialEq, Eq)]
pub struct Texture(pub String);

impl Texture {
    pub fn location(&self) -> Option<&str> {
        if self.0.starts_with(MODEL_TEXTURE_TAG) {
            None
        } else {
            Some(&self.0)
        }
    }

    /// 获取引用值的key
    pub fn reference(&self) -> Option<&str> {
        if self.0.starts_with(MODEL_TEXTURE_TAG) {
            Some(&self.0[1..])
        } else {
            None
        }
    }

    pub fn resolve<'a>(&self, substitutions: &'a Textures) -> Option<&'a str> {
        self.reference()
            .and_then(|reference| substitutions.get(reference))
            .map(|texture| texture.0.as_str())
    }
}

impl<K, V> From<HashMap<K, V>> for Textures
where
    K: Into<String>,
    V: Into<Texture>,
{
    fn from(source: HashMap<K, V>) -> Self {
        let variables = source
            .into_iter()
            .map(|(k, v)| (k.into(), v.into()))
            .collect();
        Self { variables }
    }
}

impl From<&str> for Texture {
    fn from(value: &str) -> Self {
        Self(value.to_string())
    }
}

impl From<String> for Texture {
    fn from(source: String) -> Self {
        Self(source)
    }
}
