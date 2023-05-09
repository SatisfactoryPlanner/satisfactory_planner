use std::sync::Arc;

use self::item::Item;

pub mod item;
pub mod recipe;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "bindgen", derive(serde::Serialize))]
pub struct ItemAmount {
    pub item: Arc<Item>,
    pub amount: u32,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "bindgen", derive(serde::Serialize))]
pub struct ItemAmountBind {
    pub item: Item,
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
