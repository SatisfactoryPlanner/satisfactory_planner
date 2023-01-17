use serde::{Deserialize, Serialize};

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Item {
    pub name: &'static str,
    pub raw: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct ItemBind {
    pub name: String,
    pub raw: bool,
}

impl Into<ItemBind> for Item {
    fn into(self) -> ItemBind {
        ItemBind {
            name: self.name.to_string(),
            raw: self.raw,
        }
    }
}
