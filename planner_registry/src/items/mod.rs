use std::rc::Rc;

use serde::{Deserialize, Serialize};
#[cfg(feature = "bindgen")]
use wasm_bindgen::prelude::wasm_bindgen;

use self::item::{Item, ItemBind};

pub mod item;
pub mod recipe;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ItemAmount {
    pub item: Rc<Item>,
    pub amount: u32,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct ItemAmountBind {
    pub item: ItemBind,
    pub amount: u32,
}

impl Into<ItemAmountBind> for ItemAmount {
    fn into(self) -> ItemAmountBind {
        ItemAmountBind {
            item: (*self.item).clone().into(),
            amount: self.amount,
        }
    }
}
