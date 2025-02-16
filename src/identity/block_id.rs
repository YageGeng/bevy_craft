use std::borrow::Cow;

use super::Identity;

/// block id
#[derive(Hash, PartialEq, Eq)]
pub struct BlockId {
    ns: String,
    name: String,
}

impl BlockId {
    pub fn new<T: ToString, U: ToString>(ns: T, name: U) -> Self {
        Self {
            ns: ns.to_string(),
            name: name.to_string(),
        }
    }
}

impl Identity for BlockId {
    fn id(&self) -> Cow<str> {
        Cow::Owned(format!("{}:{}", self.ns, self.name))
    }

    fn ns(&self) -> Cow<str> {
        Cow::Borrowed(&self.ns)
    }
}
