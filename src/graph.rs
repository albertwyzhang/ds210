use std::collections::HashMap;

pub struct MyGraph {
    pub adjacency_list: Vec<Vec<usize>>,
}

impl MyGraph {
    pub fn new() -> MyGraph {
        MyGraph { adjacency_list: vec![] }
    }

    pub fn add_edge(&mut self, src: usize, dest: usize) {
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

    pub fn degree_distribution(&self) -> Vec<usize> {
        let mut degrees = vec![0; self.adjacency_list.len()];
        for (vertex, neighbors) in self.adjacency_list.iter().enumerate() {
            degrees[vertex] = neighbors.len();
        }
        degrees
    }

    pub fn neighbors_at_distance_2(&self) -> Vec<usize> {
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
    pub fn betweenness_centrality(adjacency_list: &[Vec<usize>], all_shortest_paths: &[(usize, usize, Vec<usize>)]) -> HashMap<usize, f64> {
        let mut node_counts: HashMap<usize, usize> = HashMap::new();
        for (_, _, path) in all_shortest_paths {
            for &node in path.iter().skip(1).take(path.len() - 2) {
                *node_counts.entry(node).or_insert(0) += 1;
            }
        }

        let num_nodes = adjacency_list.len();
        let mut betweenness_centralities = HashMap::new();
        for (node, count) in &node_counts {
            let centrality = *count as f64 / (num_nodes * (num_nodes - 1)) as f64;
            betweenness_centralities.insert(*node, centrality);
        }

        betweenness_centralities
    }
}