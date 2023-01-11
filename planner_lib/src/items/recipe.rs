use crate::buildings::building::Machine;

use super::ItemAmount;

#[derive(Debug, Clone)]
pub struct Recipe {
    pub name: String,
    pub machine: Machine,
    pub input: Vec<ItemAmount>,
    pub output: ItemAmount,
    pub byproduct: Option<ItemAmount>,
}
