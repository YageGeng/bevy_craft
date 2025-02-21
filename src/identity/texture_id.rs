use derive_more::derive::Display;

use super::Identity;

#[derive(Display, Debug, Hash, PartialEq, Eq, Clone)]
#[display("{}:{}", ns, name)]
pub struct TextureId {
    ns: String,
    name: String,
}

impl Identity for TextureId {
    fn id(&self) -> std::borrow::Cow<str> {
        std::borrow::Cow::Owned(format!("{}:{}", self.ns, self.name))
    }

    fn ns(&self) -> std::borrow::Cow<str> {
        std::borrow::Cow::Borrowed(&self.ns)
    }
}
