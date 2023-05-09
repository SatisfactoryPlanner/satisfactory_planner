pub mod error;
pub mod recipe_graph;

pub use planner_registry;

use lazy_static::lazy_static;
use planner_registry::Registry;
#[cfg(feature = "bindgen")]
use wasm_bindgen::prelude::wasm_bindgen;

lazy_static! {
    pub static ref DEFAULT_REGISTRY: Registry = Registry::default();
}

#[cfg(feature = "bindgen")]
#[wasm_bindgen(js_name = get_craftable_items)]
pub fn get_craftable_items_js() -> js_sys::Array {
    let craftable_items = DEFAULT_REGISTRY.get_craftable_items();
    craftable_items
        .iter()
        .map(|e| serde_wasm_bindgen::to_value(&**e).unwrap())
        .collect()
}

#[cfg(feature = "bindgen")]
#[wasm_bindgen(js_name = get_item_recipes)]
pub fn get_item_recipes_js(item_name: String) -> js_sys::Array {
    let item = DEFAULT_REGISTRY
        .get_craftable_items()
        .iter()
        .find(|e| e.name == item_name)
        .cloned()
        .unwrap();

    DEFAULT_REGISTRY
        .get_recipes_for_item(item)
        .iter()
        .map(|e| serde_wasm_bindgen::to_value(e).unwrap())
        .collect()
}

// #[cfg(feature = "bindgen")]
// #[wasm_bindgen(js_name = calculate)]
// pub fn calculate_js(recipe_name: String) -> wasm_bindgen::JsValue {
//     let recipe = DEFAULT_REGISTRY.get_recipe(&recipe_name).unwrap();

//     recipe_graph::generate_graph(recipe, expected_output_per_minute, registry, recipe_overrides)

//     serde_wasm_bindgen::to_value(&123).unwrap()
// }
