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
        // Check if the line contains exactly two numbers
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
    // Function to read the graph from the file
    let file_path = "facebook_combined.txt";
    let graph = match read_graph(file_path) {
        Ok(graph) => graph,
        Err(err) => {
            eprintln!("Error reading graph from file: {}", err);
            return Err(err.into());
        }
    };
    // Count degree of each node
    let node_degree = graph.degree_of_node();
    
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

    let betweenness = graph::MyGraph::betweenness_centrality(&graph.adjacency_list, &all_shortest_paths);
    
    // Sort the nodes by degree centrality in descending order
    let mut sorted_degree_centrality: Vec<_> = betweenness.iter().collect();
    sorted_degree_centrality.sort_by(|(_, a), (_, b)| b.partial_cmp(a).unwrap());
    // Print the 10 nodes with the highest betweenness centrality
    println!("Top 10 nodes with highest betweenness centrality:");
    for (i, (node, centrality)) in sorted_degree_centrality.iter().take(10).enumerate() {
        println!("{}. Node {}: {:.4}", i + 1, node, centrality);
    }

    // Sort the nodes by degree in descending order
    let mut sorted_node_degree: Vec<_> = node_degree.iter().enumerate().collect();
    sorted_node_degree.sort_by(|(_, a), (_, b)| b.partial_cmp(a).unwrap());
    // Print the 10 nodes with highest degrees
    println!("Top 10 nodes with highest degrees:");
    for (i, (vertex, degree)) in sorted_node_degree.iter().take(10).enumerate() {
        println!("{}. Node {}: {}", i + 1, vertex, degree);
    }

    // Sort the nodes by number of neighbors at distance 2 in descending order
    let mut sorted_neighbors_at_distance_2: Vec<_> = neighbors_at_distance_2.iter().enumerate().collect();
    sorted_neighbors_at_distance_2.sort_by(|(_, a), (_, b)| b.partial_cmp(a).unwrap());
    // Print the 10 nodes with the most neighbors at distance 2
    println!("Top 10 nodes with most neighbors at distance 2:");
    for (i, (vertex, neighbors)) in sorted_neighbors_at_distance_2.iter().take(10).enumerate() {
        println!("{}. Node {}: {}", i + 1, vertex, neighbors);
    }

    Ok(())
}


#[cfg(test)]
mod tests {
    use super::*;
    use graph::MyGraph; // Import MyGraph
    use algorithms::dijkstra_shortest_paths;

    #[test]
    fn test_graph_properties() {
        // Initialize the graph
        let mut graph = MyGraph::new();
        graph.add_edge(0, 1);
        graph.add_edge(0, 2);
        graph.add_edge(0, 3);
        graph.add_edge(1, 3);
        graph.add_edge(2, 3);
        graph.add_edge(2, 4);
        graph.add_edge(3, 4);

        // Test degree of node
        let node_degree = graph.degree_of_node(); // Call degree_of_node directly on graph object
        assert_eq!(node_degree[0], 3); // Node 0 has degree 3
        assert_eq!(node_degree[1], 2); // Node 1 has degree 2
        assert_eq!(node_degree[2], 3); // Node 2 has degree 3
        assert_eq!(node_degree[3], 4); // Node 3 has degree 4
        assert_eq!(node_degree[4], 2); // Node 4 has degree 2

        // Test neighbors at distance 2
        let neighbors_at_dist_2 = graph.neighbors_at_distance_2(); // Call neighbors_at_distance_2 directly on graph object
        assert_eq!(neighbors_at_dist_2[0], 1); // Node 0 has 1 neighbor at distance 2
        assert_eq!(neighbors_at_dist_2[1], 2); // Node 1 has 2 neighbors at distance 2
        assert_eq!(neighbors_at_dist_2[2], 1); // Node 2 has 1 neighbor at distance 2
        assert_eq!(neighbors_at_dist_2[3], 0); // Node 3 has 0 neighbors at distance 2
        assert_eq!(neighbors_at_dist_2[4], 2); // Node 4 has 2 neighbors at distance 2
    }


    #[test]
    fn test_dijkstra_shortest_paths() {
        // Example adjacency list representing a graph
        let adjacency_list = vec![
            vec![1, 2, 3],
            vec![0, 3],
            vec![0, 3, 4],
            vec![0, 1, 2, 4],
            vec![2, 3],
        ];

        // Expected shortest paths from node 0
        let expected_shortest_paths = vec![
            vec![],            // Shortest path to node 0: []
            vec![0, 1],        // Shortest path to node 1: [0, 1]
            vec![0, 2],        // Shortest path to node 2: [0, 2]
            vec![0, 3],        // Shortest path to node 3: [0, 3]
            vec![0, 2, 4],     // Shortest path to node 4: [0, 2, 4]
        ];

        // Compute shortest paths from node 0
        let start_node = 0;
        let shortest_paths = dijkstra_shortest_paths(&adjacency_list, start_node);

        // Assert equality of expected and computed shortest paths
        for (node, expected_path) in expected_shortest_paths.iter().enumerate() {
            assert_eq!(&shortest_paths[node], expected_path);
        }
    }
}
