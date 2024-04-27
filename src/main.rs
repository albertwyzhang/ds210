use std::error::Error;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;


mod graph;
mod algorithms;

use graph::MyGraph;
use algorithms::dijkstra_shortest_paths;

fn read_graph(file_path: &str) -> Result<MyGraph, Box<dyn Error>> {
    let mut graph = MyGraph::new();

    let path = Path::new(file_path);
    let file = File::open(&path)?;

    let reader = std::io::BufReader::new(file);
    for line in reader.lines() {
        let line = line?;
        let parts: Vec<_> = line.split_whitespace().collect();
        if parts.len() == 2 {
            let src: usize = parts[0].parse()?;
            let dest: usize = parts[1].parse()?;
            graph.add_edge(src, dest);
        } else {
            return Err(From::from("Each line in the file must contain exactly two numbers"));
        }
    }

    Ok(graph)
}

fn main() -> Result<(), Box<dyn Error>> {
    let file_path = "facebook_combined.txt";
    let graph = match read_graph(file_path) {
        Ok(graph) => graph,
        Err(err) => {
            eprintln!("Error reading graph from file: {}", err);
            return Err(err.into());
        }
    };

    let degree_distribution = graph.degree_distribution();
    let neighbors_at_distance_2 = graph.neighbors_at_distance_2();
    let mut all_shortest_paths = Vec::new();

    for start_node in 0..graph.adjacency_list.len() {
        let shortest_paths = dijkstra_shortest_paths(&graph.adjacency_list, start_node);
        for (node, path) in shortest_paths.iter().enumerate() {
            if !path.is_empty() {
                all_shortest_paths.push((start_node, node, path.clone()));
            }
        }
    }

    let betweenness = graph::MyGraph::betweenness_centrality(&graph.adjacency_list, &all_shortest_paths);

    let mut sorted_degree_centrality: Vec<_> = betweenness.iter().collect();
    sorted_degree_centrality.sort_by(|(_, a), (_, b)| b.partial_cmp(a).unwrap());

    println!("Top 10 nodes with highest betweenness centrality:");
    for (i, (node, centrality)) in sorted_degree_centrality.iter().take(10).enumerate() {
        println!("{}. Node {}: {:.4}", i + 1, node, centrality);
    }

    let mut sorted_degree_distribution: Vec<_> = degree_distribution.iter().enumerate().collect();
    sorted_degree_distribution.sort_by(|(_, a), (_, b)| b.partial_cmp(a).unwrap());

    println!("Top 10 nodes with highest degree distribution:");
    for (i, (vertex, degree)) in sorted_degree_distribution.iter().take(10).enumerate() {
        println!("{}. Node {}: {}", i + 1, vertex, degree);
    }

    let mut sorted_neighbors_at_distance_2: Vec<_> = neighbors_at_distance_2.iter().enumerate().collect();
    sorted_neighbors_at_distance_2.sort_by(|(_, a), (_, b)| b.partial_cmp(a).unwrap());

    println!("Top 10 nodes with most neighbors at distance 2:");
    for (i, (vertex, neighbors)) in sorted_neighbors_at_distance_2.iter().take(10).enumerate() {
        println!("{}. Node {}: {}", i + 1, vertex, neighbors);
    }

    Ok(())
}
