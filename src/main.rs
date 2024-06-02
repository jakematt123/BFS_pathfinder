mod shared;
mod bfs;
mod dijkstra;
mod dfs;

use std::fs::File;
use std::io::Write;
use std::ptr::null;
use std::{collections::HashMap, env};
use shared::Territory;

fn visualize(world: &HashMap<String, Territory>, filename: &str) {
    // Create a directed graph
    let mut graph = petgraph::graph::DiGraph::<String, &str>::new();
    let mut node_indices: HashMap<String, petgraph::graph::NodeIndex> = HashMap::new();

    // Add nodes to the graph and update the mapping
    for (name, _) in world.iter() {
        let node_index = graph.add_node(name.clone());
        node_indices.insert(name.clone(), node_index);
    }

    // Add edges to the graph using NodeIndex
    for (name, territory) in world.iter() {
        let source_index = *node_indices.get(name).unwrap();
        for neighbor in &territory.neighbors {
            let target_index = *node_indices.get(neighbor).unwrap();
            graph.add_edge(source_index, target_index, "");
        }
    }

    // Generate DOT format
    let dot = petgraph::dot::Dot::with_config(&graph, &[petgraph::dot::Config::EdgeNoLabel]);
    let dot_content = format!("{}", dot);

    // Write the DOT content to the specified file
    if let Ok(mut file) = File::create(filename) {
        if let Err(err) = file.write_all(dot_content.as_bytes()) {
            eprintln!("Error writing to file: {}", err);
        }
    } else {
        eprintln!("Error creating file: {}", filename);
    }
}

macro_rules! insert_territory {
    ($territories:expr, $name:expr, [$($neighbor:expr),*], $resources:expr) => {
      {
        let neighbors: Vec<String> = vec![$($neighbor.to_owned()),*];
        let territory = Territory {
            neighbors,
            resources: $resources.to_owned(),
        };
        $territories.insert($name.to_owned(), territory);
      }
    };
}

fn create_world() -> HashMap<String, Territory> {
    let mut territories: HashMap<String, Territory> = HashMap::new();
    insert_territory!(territories, "A", ["B", "C"], "Resource A");
    insert_territory!(territories, "B", ["C", "D", "B"], "Resource B");
    insert_territory!(territories, "C", ["D", "E", "B"], "Resource C");
    insert_territory!(territories, "D", ["E", "F"], "Resource D");
    insert_territory!(territories, "E", ["F", "G"], "Resource E");
    insert_territory!(territories, "F", ["G", "H"], "Resource F");
    insert_territory!(territories, "G", ["H"], "Resource G");
    insert_territory!(territories, "H", [], "Resource H");
    insert_territory!(territories, "I", ["A", "B", "C", "D", "E", "F", "H"], "Resource I");

    return territories;
}


fn main() {
    let world: HashMap<String, Territory> = create_world();
    visualize(&world, "graph.dot");

    let args: Vec<String> = env::args().collect();

    let start: Option<&str> = Some(&args[1]);
    let end: Option<&str> = Some(&args[2]);

    let path: Option<Vec<String>> = bfs::fastest_routing(start.unwrap_or("A"), end.unwrap_or("H"), &world);
  
    match path {
      Some(p) => println!("Shortest path: {:?}", p),
      None => println!("No valid path found."),
    }
  
  }