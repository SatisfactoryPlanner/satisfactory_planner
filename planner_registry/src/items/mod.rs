use std::rc::Rc;

use self::item::Item;

pub mod item;
pub mod recipe;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ItemAmount {
    pub item: Rc<Item>,
    pub amount: u32,
}
