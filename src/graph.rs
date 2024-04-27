use std::collections::HashMap;

// Define the struct for the graph
pub struct MyGraph {
    pub adjacency_list: Vec<Vec<usize>>,
}

impl MyGraph {
    /// Creates a new empty graph.
    pub fn new() -> MyGraph {
        MyGraph { adjacency_list: vec![] }
    }

    /// Adds an edge to the graph.
    pub fn add_edge(&mut self, src: usize, dest: usize) {
        // Ensure adjacency list is large enough to accommodate both vertices
        while self.adjacency_list.len() <= src.max(dest) {
            self.adjacency_list.push(vec![]);
        }
        // undirected graph, so add twice
        if !self.adjacency_list[src].contains(&dest) {
            self.adjacency_list[src].push(dest);
        }
        if !self.adjacency_list[dest].contains(&src) {
            self.adjacency_list[dest].push(src);
        }
    }

    /// Calculates the degree of each node in the graph.
    pub fn degree_of_node(&self) -> Vec<usize> {
        let mut degrees = vec![0; self.adjacency_list.len()];
        // Count the number of neighbors for each vertex
        for (vertex, neighbors) in self.adjacency_list.iter().enumerate() {
            degrees[vertex] = neighbors.len();
        }
        degrees
    }

    // Calculates the number of neighbors at a distance of 2 for each vertex.
    pub fn neighbors_at_distance_2(&self) -> Vec<usize> {
        // Initialize a vector to store the number of neighbors at distance 2 for each vertex
        let mut neighbors_at_distance_2 = vec![0; self.adjacency_list.len()];
    
        // Iterate over each vertex in the graph
        for (vertex, neighbors) in self.adjacency_list.iter().enumerate() {
            // Initialize a vector to keep track of visited vertices
            let mut visited = vec![false; self.adjacency_list.len()];
            // Mark the current vertex as visited
            visited[vertex] = true;
    
            // Iterate over each neighbor of the current vertex
            for &neighbor in neighbors {
                // Mark the neighbor as visited
                visited[neighbor] = true;
    
                // Iterate over each neighbor of the neighbor (second hop neighbors)
                for &second_hop_neighbor in &self.adjacency_list[neighbor] {
                    // Check if the second hop neighbor has not been visited and is not a direct neighbor of the current vertex
                    if !visited[second_hop_neighbor] && !neighbors.contains(&second_hop_neighbor) {
                        // Increment the count of neighbors at distance 2 for the current vertex
                        neighbors_at_distance_2[vertex] += 1;
                        // Mark the second hop neighbor as visited
                        visited[second_hop_neighbor] = true;
                    }
                }
            }
        }
    
        // Return the vector containing the number of neighbors at distance 2 for each vertex
        neighbors_at_distance_2
    }
    

    // Function to calculate betweenness centrality
    pub fn betweenness_centrality(adjacency_list: &[Vec<usize>], all_shortest_paths: &[(usize, usize, Vec<usize>)]) -> HashMap<usize, f64> {
        let mut node_counts: HashMap<usize, usize> = HashMap::new();
        for (_, _, path) in all_shortest_paths {
            // Count the number of times each node has appeared in the middle of a shortest path(exclude the times it starts or ends at that node)
            for &node in path.iter().skip(1).take(path.len() - 2) {
                *node_counts.entry(node).or_insert(0) += 1;
            }
        }

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