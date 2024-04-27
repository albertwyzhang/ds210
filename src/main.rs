use std::error::Error;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;
use std::collections::{HashMap, BinaryHeap};
use std::cmp::Reverse;

// Define the struct for the graph
struct MyGraph {
    adjacency_list: Vec<Vec<usize>>,
}

impl MyGraph {
    fn new() -> MyGraph {
        MyGraph { adjacency_list: vec![] }
    }

    fn add_edge(&mut self, src: usize, dest: usize) {
        while self.adjacency_list.len() <= src.max(dest) {
            self.adjacency_list.push(vec![]);
        }
        if !self.adjacency_list[src].contains(&dest) {
            self.adjacency_list[src].push(dest);
        }
        if !self.adjacency_list[dest].contains(&src) {
            self.adjacency_list[dest].push(src);
        }
    }
    
    fn degree_distribution(&self) -> Vec<usize> {
        let mut degrees = vec![0; self.adjacency_list.len()];
        for (vertex, neighbors) in self.adjacency_list.iter().enumerate() {
            degrees[vertex] = neighbors.len();
        }
        degrees
    }

    fn neighbors_at_distance_2(&self) -> Vec<usize> {
        let mut neighbors_at_distance_2 = vec![0; self.adjacency_list.len()];

        for (vertex, neighbors) in self.adjacency_list.iter().enumerate() {
            let mut visited = vec![false; self.adjacency_list.len()];
            visited[vertex] = true;

            for &neighbor in neighbors {
                visited[neighbor] = true;

                for &second_hop_neighbor in &self.adjacency_list[neighbor] {
                    if !visited[second_hop_neighbor] && !neighbors.contains(&second_hop_neighbor) {
                        neighbors_at_distance_2[vertex] += 1;
                        visited[second_hop_neighbor] = true;
                    }
                }
            }
        }
        neighbors_at_distance_2
    }
        // Function to calculate betweenness centrality
    fn betweenness_centrality(adjacency_list: &[Vec<usize>], all_shortest_paths: &[(usize, usize, Vec<usize>)]) -> HashMap<usize, f64> {
        // Count the number of times each node has appeared in the middle of a vector(exclude the times it starts or ends at that node)
        let mut node_counts: HashMap<usize, usize> = HashMap::new();
        for (_, _, path) in all_shortest_paths {
            for &node in path.iter().skip(1).take(path.len() - 2) {
                *node_counts.entry(node).or_insert(0) += 1;
            }
        }

        // Calculate betweenness centrality for each node
        let num_nodes = adjacency_list.len();
        let mut betweenness_centralities = HashMap::new();
        for (node, count) in &node_counts {
            //divid the count by the total number of shortest paths
            let centrality = *count as f64 / (num_nodes * (num_nodes - 1)) as f64;
            betweenness_centralities.insert(*node, centrality);
        }

        betweenness_centralities
    }
}
// Function to read the graph from the file
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
    // Read the graph from the file
    let file_path = "test2.txt";
    let graph = match read_graph(file_path) {
        Ok(graph) => graph,
        Err(err) => {
            eprintln!("Error reading graph from file: {}", err);
            return Err(err.into());
        }
    };

    // Count degree distribution
    let degree_distribution = graph.degree_distribution();

    // Count number of neighbors at distance 2
    let neighbors_at_distance_2 = graph.neighbors_at_distance_2();

    // Calculate betweenness centrality
    let mut all_shortest_paths = Vec::new();

    for start_node in 0..graph.adjacency_list.len() {
        let shortest_paths = dijkstra_shortest_paths(&graph.adjacency_list, start_node);
        for (node, path) in shortest_paths.iter().enumerate() {
            if !path.is_empty() {
                all_shortest_paths.push((start_node, node, path.clone()));
            }
        }
    }
    let betweenness = MyGraph::betweenness_centrality(&graph.adjacency_list, &all_shortest_paths);

    // Sort the nodes by degree centrality in descending order
    let mut sorted_degree_centrality: Vec<_> = betweenness.iter().collect();
    sorted_degree_centrality.sort_by(|(_, a), (_, b)| b.partial_cmp(a).unwrap());

    // Print betweenness centrality
    println!("Betweenness Centrality:");
    for (node, centrality) in &betweenness {
        println!("Node {}: {:.4}", node, centrality);
    }

    // Print degree distribution
    println!("Degree Distribution:");
    for (vertex, degree) in degree_distribution.iter().enumerate() {
        println!("Node {}: {}", vertex, degree);
    }

    // Print number of neighbors at distance 2
    println!("Neighbors at Distance 2:");
    for (vertex, neighbors) in neighbors_at_distance_2.iter().enumerate() {
        println!("Node {}: {}", vertex, neighbors);
    }
    Ok(())
}




fn dijkstra_shortest_paths(adj_list: &[Vec<usize>], start_node: usize) -> Vec<Vec<usize>> {
    let num_nodes = adj_list.len();
    let mut distances = vec![usize::MAX; num_nodes];
    let mut shortest_paths = vec![vec![]; num_nodes];
    distances[start_node] = 0;
    let mut heap = BinaryHeap::new();
    heap.push(Reverse((0, start_node)));

    while let Some(Reverse((dist, node))) = heap.pop() {
        if dist > distances[node] {
            continue;
        }
        for &next_node in &adj_list[node] {
            let next_dist = dist + 1;
            if next_dist < distances[next_node] {
                distances[next_node] = next_dist;
                heap.push(Reverse((next_dist, next_node)));
                shortest_paths[next_node] = vec![node];
            } else if next_dist == distances[next_node] {
                shortest_paths[next_node].push(node);
            }
        }

        // Update the shortest paths in reverse direction (for undirected graph)
        for &prev_node in adj_list[node].iter() {
            let prev_dist = dist + 1;
            if prev_dist < distances[prev_node] {
                distances[prev_node] = prev_dist;
                heap.push(Reverse((prev_dist, prev_node)));
                shortest_paths[prev_node] = vec![node];
            } else if prev_dist == distances[prev_node] {
                shortest_paths[prev_node].push(node);
            }
        }
    }

    // Backtrack to construct the actual shortest paths
    for node in 0..num_nodes {
        if shortest_paths[node].len() > 0 {
            let mut path = vec![node];
            let mut current_node = node;
            while current_node != start_node {
                let prev_node = shortest_paths[current_node][0];
                path.insert(0, prev_node);
                current_node = prev_node;
            }
            shortest_paths[node] = path;
        }
    }

    // Remove duplicate nodes in paths
    for path in &mut shortest_paths {
        path.dedup();
    }

    shortest_paths
}