use std::rc::Rc;

pub mod item;
pub mod recipe;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ItemAmount {
    pub item: Rc<item::Item>,
    pub amount: u32,
}
