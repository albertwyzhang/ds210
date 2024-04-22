use std::collections::{HashMap, HashSet};
use std::error::Error;
use std::fs::File;
use std::io::{BufRead, BufReader, Write};

use petgraph::algo::dijkstra;
use petgraph::dot::{Config, Dot};
use petgraph::graph::{NodeIndex};
use petgraph::Directed;

// Type alias for the graph representation
pub type Graph = petgraph::Graph<(), f64, Directed>;

// Read the graph from the file
pub fn read_graph(file_path: &str) -> Result<Graph, Box<dyn std::error::Error>> {
    let mut graph: Graph = Graph::new();

    let file = File::open(file_path)?;
    let reader = BufReader::new(file);
    for line in reader.lines() {
        let line = line?;
        let mut parts = line.split_whitespace();
        if let (Some(src), Some(dest)) = (parts.next(), parts.next()) {
            let src: usize = src.parse()?;
            let dest: usize = dest.parse()?;
            // Ensure that src and dest are within the bounds of the graph's nodes
            if src < graph.node_count() && dest < graph.node_count() {
                // For undirected graph, add both directions
                graph.add_edge(NodeIndex::new(src), NodeIndex::new(dest), 1.0);
                graph.add_edge(NodeIndex::new(dest), NodeIndex::new(src), 1.0);
            } else {
                // Handle invalid node indices here
                println!("Invalid node indices: src={}, dest={}", src, dest);
            }
        }
    }
    

// Compute the number of neighbors at distance 2 for each vertex
pub fn neighbors_at_distance_2(graph: &Graph) -> HashMap<usize, usize> {
    let mut neighbors_at_distance_2: HashMap<usize, usize> = HashMap::new();

    for node in graph.node_indices() {
        let mut visited: HashSet<NodeIndex> = HashSet::new();
        let mut neighbors_at_distance_2_count = 0;
        for neighbor in graph.neighbors(node) {
            for second_neighbor in graph.neighbors(neighbor) {
                if !visited.contains(&second_neighbor) {
                    visited.insert(second_neighbor);
                    neighbors_at_distance_2_count += 1;
                }
            }
        }
        neighbors_at_distance_2.insert(node.index(), neighbors_at_distance_2_count);
    }

    neighbors_at_distance_2
}

// Analyze degree distribution
pub fn analyze_degree_distribution(graph: &Graph) -> HashMap<usize, usize> {
    let mut degree_counts: HashMap<usize, usize> = HashMap::new();

    for node in graph.node_indices() {
        let degree = graph.neighbors(node).count();
        *degree_counts.entry(degree).or_insert(0) += 1;
    }

    degree_counts
}

// Analyze centrality
pub fn analyze_centrality(graph: &Graph) {
    let mut degree_centrality: HashMap<NodeIndex, f64> = HashMap::new();

    for node in graph.node_indices() {
        let scores = dijkstra(graph, node, None, |e| *e.weight());
        let sum_of_shortest_paths: f64 = scores.values().sum(); 
        let node_centrality = 1.0 / sum_of_shortest_paths;
        degree_centrality.insert(node, node_centrality);
    }

    println!("Degree Centrality: {:?}", degree_centrality);
}


// Function to print top n elements
pub fn print_top_n<K: std::fmt::Debug, V: Ord + std::fmt::Display>(
    map: &HashMap<K, V>,
    n: usize,
) {
    let mut sorted_vec: Vec<_> = map.iter().collect();
    sorted_vec.sort_by_key(|&(_, v)| std::cmp::Reverse(v));
    for (key, value) in sorted_vec.iter().take(n) {
        println!("{:?}: {}", key, value);
    }
}

// Visualization function
pub fn visualize_graph(
    graph: &Graph,
    file_path: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    let mut file = File::create(file_path)?;
    let dot = Dot::with_config(graph, &[Config::EdgeNoLabel]);
    write!(file, "{:?}", dot)?;
    Ok(())
}

fn main() -> Result<(), Box<dyn Error>> {
    // Read the graph from the file
    let file_path = "facebook_combined.txt";
    let graph = match read_graph(file_path) {
        Ok(graph) => graph,
        Err(err) => {
            eprintln!("Error reading graph from file: {}", err);
            return Err(err.into());
        }
    };
    // Analyze degree distribution
    let degree_counts = analyze_degree_distribution(&graph);

    println!("Degree Distribution:");
    for (degree, count) in &degree_counts {
        println!("Degree {}: Count {}", degree, count);
    }

    // Compute number of neighbors at distance 2
    let neighbors_at_distance_2 = neighbors_at_distance_2(&graph);

    println!("Neighbors at Distance 2:");
    for (vertex, count) in &neighbors_at_distance_2 {
        println!("Vertex {}: Count {}", vertex, count);
    }

    // Analyze centrality
    analyze_centrality(&graph);

    // Visualize the graph
    visualize_graph(&graph, "graph_visualization.dot")?;

    Ok(())
}
}
