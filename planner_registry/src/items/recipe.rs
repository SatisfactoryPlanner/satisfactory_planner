use std::rc::Rc;

#[cfg(feature = "bindgen")]
use wasm_bindgen::prelude::wasm_bindgen;
use wasm_bindgen::JsValue;

use crate::buildings::building::Machine;

use super::{item::Item, ItemAmount, ItemAmountBind};

#[derive(Debug, Clone)]
pub struct ItemRate {
    pub item: Rc<Item>,
    pub rate: f32,
}

#[wasm_bindgen]
#[derive(Debug, Clone, PartialEq)]
pub struct Recipe {
    pub name: &'static str,
    pub machine: Machine,
    #[wasm_bindgen(skip)]
    pub input: Vec<ItemAmount>,
    #[wasm_bindgen(skip)]
    pub output: ItemAmount,
    #[wasm_bindgen(skip)]
    pub byproduct: Option<ItemAmount>,
    pub manufacturing_duration: f32,
    pub alternate: bool,
}

#[cfg(feature = "bindgen")]
#[wasm_bindgen]
impl Recipe {
    pub fn get_input(&self) -> JsValue {
        serde_wasm_bindgen::to_value(
            &self
                .input
                .iter()
                .map(|e| Into::<ItemAmountBind>::into(e.clone()))
                .collect::<Vec<ItemAmountBind>>(),
        )
        .unwrap()
    }

    pub fn get_output(&self) -> JsValue {
        serde_wasm_bindgen::to_value(&Into::<ItemAmountBind>::into(self.output.clone())).unwrap()
    }

    pub fn has_byproduct(&self) -> bool {
        self.byproduct.is_some()
    }

    pub fn get_byproduct(&self) -> JsValue {
        if let Some(ref byproduct) = self.byproduct {
            serde_wasm_bindgen::to_value(&Into::<ItemAmountBind>::into(byproduct.clone())).unwrap()
        } else {
            serde_wasm_bindgen::to_value(&[0]).unwrap()
        }
    }
}
