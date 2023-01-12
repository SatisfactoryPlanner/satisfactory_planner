use std::{fmt::Display, rc::Rc};

use crate::buildings::building::Machine;

use super::{item::Item, ItemAmount};

#[derive(Debug, Clone)]
pub struct ItemRate {
    pub item: Rc<Item>,
    pub rate: f32,
}

impl Display for ItemRate {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} ({}/min)", self.item.name, self.rate)
    }
}

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

impl Display for Recipe {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let items_in = self
            .input_rate()
            .iter()
            .map(|ir| ir.to_string())
            .collect::<Vec<_>>()
            .join(", ");
        let items_out = self
            .output_rate()
            .iter()
            .map(|ir| ir.to_string())
            .collect::<Vec<_>>()
            .join(", ");
        write!(f, "You need: {items_in} to get {items_out}")
    }
}

pub fn calculate_rate(duration: f32, amount: f32) -> f32 {
    60f32 / duration * amount
}

impl Recipe {
    pub fn input_rate(&self) -> Vec<ItemRate> {
        self.input
            .iter()
            .map(|c| ItemRate {
                item: c.item.clone(),
                rate: calculate_rate(self.manufacturing_duration, c.amount as f32),
            })
            .collect()
    }

    /// `rates[0]` is an actual item; `rates[1]` is byproduct
    pub fn output_rate(&self) -> Vec<ItemRate> {
        let mut rates = Vec::with_capacity(2);
        rates.push(ItemRate {
            item: self.output.item.clone(),
            rate: calculate_rate(self.manufacturing_duration, self.output.amount as f32),
        });

        if let Some(byproduct) = &self.byproduct {
            rates.push(ItemRate {
                item: byproduct.item.clone(),
                rate: calculate_rate(self.manufacturing_duration, byproduct.amount as f32),
            })
        }

        rates
    }
}
