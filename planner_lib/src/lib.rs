use std::collections::HashMap;

use petgraph::{dot::Dot, stable_graph::NodeIndex, Graph};
use planner_registry::{items::recipe::Recipe, Registry};

pub mod error;

pub fn traverse_recipe(recipe: &Recipe, registry: &Registry) {
    let mut g: Graph<&str, &str> = Graph::new();
    let mut index_map = HashMap::new();

    let root_node = g.add_node(recipe.name);
    index_map.insert(recipe.name, root_node);
    add_nodes(&mut g, &mut index_map, recipe, registry);
    println!("{:?}", Dot::new(&g))
}

fn add_nodes(
    g: &mut Graph<&str, &str>,
    index_map: &mut HashMap<&str, NodeIndex>,
    recipe: &Recipe,
    registry: &Registry,
) {
    for node in &recipe.input {
        // Add the current node
        let current_node = g.add_node(node.item.name);
        index_map.insert(node.item.name, current_node);

        // Add an edge from the parent node to the current node
        g.add_edge(index_map[&recipe.name], current_node, "depends on");

        add_nodes(
            g,
            index_map,
            // Fix raw ingedients
            registry.get_default_recipe(node.item.name).unwrap(),
            registry,
        );
    }
}

#[cfg(test)]
mod tests {
    use planner_registry::Registry;

    use super::*;

    #[test]
    fn it_works() {
        let registry = Registry::new();

        let rec = registry.get_default_recipe("Modular Frame").unwrap();
        traverse_recipe(rec, &registry);
    }
}
