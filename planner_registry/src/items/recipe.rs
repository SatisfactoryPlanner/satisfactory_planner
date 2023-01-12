use crate::buildings::building::Machine;

use super::ItemAmount;

#[derive(Debug, Clone, PartialEq)]
pub struct Recipe {
    pub name: &'static str,
    pub machine: Machine,
    pub input: Vec<ItemAmount>,
    pub output: ItemAmount,
    pub byproduct: Option<ItemAmount>,
    pub manufacturing_duration: f32,
    pub alternate: bool,
}
