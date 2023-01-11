use crate::buildings::building::Machine;

use super::ItemWithRate;

#[derive(Debug, Clone)]
pub struct Recipe {
    pub machine: Machine,
    pub input: Vec<ItemWithRate>,
    pub output_rate: f32,
    pub byproduct: Option<ItemWithRate>,
}
