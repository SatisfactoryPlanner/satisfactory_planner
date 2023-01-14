pub mod error;
pub mod recipe_graph;

#[cfg(test)]
mod tests {
    use petgraph::dot::Dot;
    use planner_registry::Registry;

    use super::*;

    #[test]
    fn it_works() {
        let registry = Registry::new();

        let recipe = registry
            .get_default_recipe("Thermal Propulsion Rocket")
            .unwrap();
        let graph = recipe_graph::generate_graph(recipe, 60f32, &registry);
        println!("{:?}", Dot::new(&graph));
    }
}
