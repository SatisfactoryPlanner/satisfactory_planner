use std::rc::Rc;

use super::recipe::Recipe;

#[derive(Debug, Clone)]
pub struct Item {
    // None if raw/from node
    pub recipe: Option<Rc<Recipe>>,
}
