pub mod item;
pub mod recipe;

#[derive(Debug, Clone)]
pub struct ItemWithRate(pub item::Item, pub f32);
