use std::{collections::HashMap, rc::Rc, sync::Arc};

use petgraph::{
    stable_graph::NodeIndex,
    visit::{Bfs, Dfs, EdgeRef},
    Graph,
};
use planner_registry::{
    items::{item::Item, recipe::Recipe},
    Registry,
};

fn get_or_create_node(
    nodes: &mut HashMap<&str, NodeIndex>,
    graph: &mut Graph<Arc<Item>, f32>,
    item: Arc<Item>,
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
    graph: &mut Graph<Arc<Item>, f32>,
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
    recipe: &Recipe,
    expected_output_per_minute: f32,
    registry: &Registry,
    recipe_overrides: &HashMap<&'static str, &Recipe>,
    existing_nodes: &mut HashMap<&str, NodeIndex>,
    graph: &mut Graph<Arc<Item>, f32>,
    root_node: NodeIndex,
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
            if let Some(recipe) = recipe_overrides
                .get(name)
                .copied()
                .or_else(|| registry.get_default_recipe(name))
            {
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
    recipe_overrides: &HashMap<&'static str, &Recipe>,
) -> (Graph<Arc<Item>, f32>, NodeIndex) {
    let mut graph: Graph<Arc<Item>, f32> = Graph::new();

    let root_node = graph.add_node(recipe.output.item.clone());

    let mut existing_nodes = HashMap::new();

    add_recipe_tree_to_graph(
        recipe,
        expected_output_per_minute,
        registry,
        recipe_overrides,
        &mut existing_nodes,
        &mut graph,
        root_node,
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
            .sum();

        let name = graph.node_weight(node).unwrap().name;

        let outgoing_edges = graph
            .edges_directed(node, petgraph::Direction::Outgoing)
            .map(|e| e.id())
            .collect::<Vec<_>>();

        for outgoing_edge in outgoing_edges {
            graph.remove_edge(outgoing_edge);
        }

        if let Some(recipe) = registry.get_default_recipe(name) {
            add_recipe_tree_to_graph(
                recipe,
                sum,
                registry,
                recipe_overrides,
                &mut existing_nodes,
                &mut graph,
                node,
            );
        }
    }

    // flatten edge
    let mut bfs = Bfs::new(&graph, root_node);
    while let Some(node) = bfs.next(&graph) {
        let neighbors = graph
            .neighbors_directed(node, petgraph::Direction::Outgoing)
            .collect::<Vec<_>>();

        for neighbor in neighbors {
            let edges = graph.edges_connecting(node, neighbor).collect::<Vec<_>>();

            let edge_ids = edges.iter().map(|e| e.id()).collect::<Vec<_>>();
            let sum = edges.iter().map(|e| *e.weight()).sum();

            for edge in edge_ids {
                graph.remove_edge(edge);
            }

            graph.add_edge(node, neighbor, sum);
        }
    }

    (graph, root_node)
}
