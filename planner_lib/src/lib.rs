pub mod error;
pub mod recipe_graph;

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use petgraph::dot::Dot;
    use planner_registry::Registry;

    use super::*;

    #[test]
    fn it_works() {
        let registry = Registry::new();

        let recipe = registry.get_default_recipe("Computer").unwrap();
        let mut recipe_overrides = HashMap::new();

        let pure_ci = registry
            .get_recipes("Copper Ingot")
            .unwrap()
            .iter()
            .find(|r| r.name == "Alternate: Pure Copper Ingot")
            .unwrap();
        recipe_overrides.insert("Copper Ingot", pure_ci);

        let (graph, _root_node) =
            recipe_graph::generate_graph(recipe, 30f32, &registry, &recipe_overrides);
        println!("{:?}", Dot::new(&graph));
    }
}
