pub mod item;
pub mod recipe;

#[derive(Debug, Clone)]
pub struct ItemAmount {
    pub item: item::Item,
    pub amount: u32,
    pub manufacturing_duration: f32,
}
