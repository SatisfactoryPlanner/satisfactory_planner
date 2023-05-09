#[cfg(feature = "bindgen")]
use wasm_bindgen::prelude::wasm_bindgen;

use crate::items::recipe::Recipe;

#[cfg_attr(feature = "bindgen", derive(serde::Serialize), wasm_bindgen)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum Machine {
    None,
    Assembler,
    Blender,
    Constructor,
    Foundry,
    Manufacturer,
    Packager,
    ParticleAccelerator,
    Refinery,
    Smelter,
}

#[wasm_bindgen]
#[derive(Debug, Clone)]
pub struct Building {
    #[wasm_bindgen(skip)]
    pub recipe: Recipe,
}

#[cfg_attr(feature = "bindgen", wasm_bindgen)]
impl Building {
    pub fn get_recipe(&self) -> Recipe {
        self.recipe.clone()
    }
}
