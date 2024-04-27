use std::collections::BinaryHeap;
use std::cmp::Reverse;

/// Computes the shortest paths from a start node to all other nodes using Dijkstra's algorithm.
pub fn dijkstra_shortest_paths(adj_list: &[Vec<usize>], start_node: usize) -> Vec<Vec<usize>> {
    // Initialize variables
    let num_nodes = adj_list.len();
    let mut distances = vec![usize::MAX; num_nodes];
    let mut shortest_paths = vec![vec![]; num_nodes];
    distances[start_node] = 0;
    let mut heap = BinaryHeap::new();
    heap.push(Reverse((0, start_node)));

    // Main loop of Dijkstra's algorithm
    while let Some(Reverse((dist, node))) = heap.pop() {
        // Check if the current distance is greater than the shortest distance found so far
        if dist > distances[node] {
            continue; // Skip this node if it has already been processed
        }

        // Explore neighbors of the current node
        for &next_node in &adj_list[node] {
            let next_dist = dist + 1;
            if next_dist < distances[next_node] {
                // Found a shorter path to the neighbor node
                distances[next_node] = next_dist;
                heap.push(Reverse((next_dist, next_node)));
                shortest_paths[next_node] = vec![node];
            } else if next_dist == distances[next_node] {
                // Found another shortest path to the neighbor node
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
