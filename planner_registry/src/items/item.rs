#[cfg(feature = "bindgen")]
use wasm_bindgen::prelude::wasm_bindgen;

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "bindgen", derive(serde::Serialize), wasm_bindgen)]
pub struct Item {
    #[wasm_bindgen(skip)]
    pub name: &'static str,
    pub raw: bool,
}

#[wasm_bindgen]
impl Item {
    pub fn get_name(&self) -> String {
        self.name.to_string()
    }
}
