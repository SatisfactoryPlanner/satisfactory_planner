use std::{collections::HashMap, rc::Rc};

use petgraph::{
    stable_graph::NodeIndex,
    visit::{Dfs, EdgeRef},
    Graph,
};
use planner_registry::{
    items::{item::Item, recipe::Recipe},
    Registry,
};

fn get_or_create_node(
    nodes: &mut HashMap<&str, NodeIndex>,
    graph: &mut Graph<Rc<Item>, f32>,
    item: Rc<Item>,
) -> NodeIndex {
    match nodes.get(item.name) {
        Some(e) => *e,
        None => {
            let node = graph.add_node(item.clone());
            nodes.insert(item.name, node);
            node
        }
    }
}

fn add_recipe_to_graph(
    existing_nodes: &mut HashMap<&str, NodeIndex>,
    root_node: NodeIndex,
    expected_output_per_minute: f32,
    recipe: &Recipe,
    graph: &mut Graph<Rc<Item>, f32>,
) -> Vec<(&'static str, f32, NodeIndex)> {
    let mut next_round = Vec::new();

    if recipe.output.item.raw {
        return Vec::new();
    }

    let per_minute_multiplier = 60f32 / recipe.manufacturing_duration;
    let crafted_per_minute = recipe.output.amount as f32 * per_minute_multiplier;
    let multiplier = expected_output_per_minute / crafted_per_minute;

    for input in &recipe.input {
        let input_node = get_or_create_node(existing_nodes, graph, input.item.clone());
        let per_minute = input.amount as f32 * per_minute_multiplier * multiplier;

        // graph loop guard
        if graph
            .edges_connecting(input_node, root_node)
            .next()
            .is_none()
        {
            if !input.item.raw {
                next_round.push((input.item.name, per_minute, input_node));
            }

            graph.add_edge(root_node, input_node, -per_minute);
        }
    }

    if let Some(ref byproduct) = recipe.byproduct {
        let byproduct_node = get_or_create_node(existing_nodes, graph, byproduct.item.clone());
        let per_minute = byproduct.amount as f32 * per_minute_multiplier * multiplier;

        graph.add_edge(root_node, byproduct_node, per_minute);
    }

    next_round
}

fn add_recipe_tree_to_graph(
    existing_nodes: &mut HashMap<&str, NodeIndex>,
    graph: &mut Graph<Rc<Item>, f32>,
    root_node: NodeIndex,
    expected_output_per_minute: f32,
    registry: &Registry,
    recipe: &Recipe,
) {
    let mut current_round = add_recipe_to_graph(
        existing_nodes,
        root_node,
        expected_output_per_minute,
        recipe,
        graph,
    );
    let mut next_round = Vec::new();

    loop {
        for (name, expected_output_per_minute, node) in current_round {
            if let Some(recipe) = registry.get_default_recipe(name) {
                next_round.extend(add_recipe_to_graph(
                    existing_nodes,
                    node,
                    expected_output_per_minute,
                    recipe,
                    graph,
                ));
            }
        }

        if next_round.is_empty() {
            break;
        }

        current_round = next_round;
        next_round = Vec::new();
    }
}

pub fn generate_graph(
    recipe: &Recipe,
    expected_output_per_minute: f32,
    registry: &Registry,
) -> Graph<Rc<Item>, f32> {
    let mut graph: Graph<Rc<Item>, f32> = Graph::new();

    // todo: recipe settings

    let root_node = graph.add_node(recipe.output.item.clone());

    let mut existing_nodes = HashMap::new();

    add_recipe_tree_to_graph(
        &mut existing_nodes,
        &mut graph,
        root_node,
        expected_output_per_minute,
        registry,
        recipe,
    );

    // recalculating with account for byproducts
    let mut dfs = Dfs::new(&graph, root_node);
    while let Some(node) = dfs.next(&graph) {
        let needs_recalculating = graph
            .edges_directed(node, petgraph::Direction::Incoming)
            .any(|e| *e.weight() > 0f32);

        if !needs_recalculating {
            continue;
        }

        let byproduct_only = graph
            .edges_directed(node, petgraph::Direction::Incoming)
            .all(|e| *e.weight() > 0f32);

        if byproduct_only {
            continue;
        }

        let sum = graph
            .edges_directed(node, petgraph::Direction::Incoming)
            .map(|e| *e.weight())
            .sum::<f32>();

        let name = graph.node_weight(node).unwrap().name;

        let outgoing_edges = graph
            .edges_directed(node, petgraph::Direction::Outgoing)
            .map(|e| e.id())
            .collect::<Vec<_>>();

        for outgoing_edge in outgoing_edges {
            graph.remove_edge(outgoing_edge);
        }

        // todo: recipe override
        if let Some(recipe) = registry.get_default_recipe(name) {
            add_recipe_tree_to_graph(&mut existing_nodes, &mut graph, node, sum, registry, recipe);
        }
    }

    graph
}
