use std::collections::HashMap;
use std::rc::Rc;

pub mod buildings;
pub mod items;

mod registry_data;

use crate::items::item::Item;
use crate::items::recipe::Recipe;

pub struct Registry {
    pub item_registry: Vec<Rc<Item>>,
    pub recipe_registry: HashMap<&'static str, Vec<Recipe>>,
}

impl Registry {
    pub fn new() -> Self {
        let (item_registry, recipe_registry) = registry_data::get_registry();

        let mut keys = recipe_registry.keys().copied().collect::<Vec<_>>();
        keys.sort();

        Registry {
            item_registry,
            recipe_registry,
        }
    }

    pub fn get_craftable_items(&self) -> Vec<Rc<Item>> {
        self.item_registry
            .iter()
            .cloned()
            .filter(|item| !item.raw)
            .collect()
    }

    pub fn get_recipes(&self, query: &str) -> Option<&Vec<Recipe>> {
        self.recipe_registry.get(query)
    }

    pub fn get_default_recipe(&self, query: &str) -> Option<&Recipe> {
        self.recipe_registry
            .get(query)
            .and_then(|r| r.iter().find(|r| !r.alternate))
    }

    pub fn search(&self, query: &str) -> Vec<(&str, &Vec<Recipe>)> {
        self.recipe_registry
            .iter()
            .filter(|(name, _)| name.to_lowercase().contains(query))
            .map(|(name, recipe)| (*name, recipe))
            .collect::<Vec<_>>()
    }
}

impl Default for Registry {
    fn default() -> Self {
        Self::new()
    }
}
