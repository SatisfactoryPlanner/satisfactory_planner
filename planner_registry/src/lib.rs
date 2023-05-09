use std::collections::HashMap;
use std::sync::Arc;

pub mod buildings;
pub mod items;

mod registry_data;

use crate::items::item::Item;
use crate::items::recipe::Recipe;

pub struct Registry {
    pub item_registry: Vec<Arc<Item>>,
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

    pub fn get_craftable_items(&self) -> Vec<Arc<Item>> {
        self.item_registry
            .iter()
            .cloned()
            .filter(|item| !item.raw)
            .collect()
    }

    pub fn get_recipes_for_item(&self, item: Arc<Item>) -> Vec<&Recipe> {
        self.recipe_registry
            .get(item.name)
            .map(|e| e.iter().collect::<Vec<_>>())
            .unwrap_or_default()
    }

    pub fn get_recipe(&self, recipe_name: &str) -> Option<&Recipe> {
        self.recipe_registry.iter().flat_map(|(_, recipes)| recipes)
            .find(|e| e.name == recipe_name)
    }

    pub fn get_default_recipe(&self, recipe_name: &str) -> Option<&Recipe> {
        self.recipe_registry
            .get(recipe_name)
            .and_then(|r| r.iter().find(|r| !r.alternate))
    }
}

impl Default for Registry {
    fn default() -> Self {
        Self::new()
    }
}
