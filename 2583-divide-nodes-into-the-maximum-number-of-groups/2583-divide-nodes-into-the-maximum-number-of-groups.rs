impl Solution {
   fn magnificent_sets(n: i32, edges: Vec<Vec<i32>>) -> i32 {
    use std::collections::{HashSet, VecDeque};

    // Create the adjacency list for the graph
    let mut graph = vec![vec![]; n as usize + 1];
    for edge in edges {
        let (u, v) = (edge[0] as usize, edge[1] as usize);
        graph[u].push(v);
        graph[v].push(u);
    }

    // Helper function to perform BFS and check if the graph is bipartite
    fn bfs_max_groups(node: usize, graph: &Vec<Vec<usize>>) -> Option<i32> {
        let mut queue = VecDeque::new();
        let mut visited = vec![false; graph.len()];
        let mut distances = vec![-1; graph.len()];

        queue.push_back(node);
        visited[node] = true;
        distances[node] = 0;
        let mut max_distance = 0;

        while let Some(current) = queue.pop_front() {
            for &neighbor in &graph[current] {
                if !visited[neighbor] {
                    visited[neighbor] = true;
                    distances[neighbor] = distances[current] + 1;
                    max_distance = max_distance.max(distances[neighbor]);
                    queue.push_back(neighbor);
                } else if distances[neighbor] % 2 == distances[current] % 2 {
                    return None; // Not bipartite
                }
            }
        }

        Some(max_distance + 1) // Number of groups is max distance + 1
    }

    let mut visited = vec![false; n as usize + 1];
    let mut result = 0;

    for i in 1..=n as usize {
        if !visited[i] {
            let mut component = vec![];
            let mut queue = VecDeque::new();
            queue.push_back(i);
            visited[i] = true;

            while let Some(node) = queue.pop_front() {
                component.push(node);
                for &neighbor in &graph[node] {
                    if !visited[neighbor] {
                        visited[neighbor] = true;
                        queue.push_back(neighbor);
                    }
                }
            }

            let mut local_max = 0;
            for &node in &component {
                if let Some(max_groups) = bfs_max_groups(node, &graph) {
                    local_max = local_max.max(max_groups);
                } else {
                    return -1; // Not possible to divide nodes into groups
                }
            }

            result += local_max;
        }
    }

    result
  }
}