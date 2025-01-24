#[derive(Copy, Clone, PartialEq)]
enum NodeState {
    Unseen,
    Processing,
    Safe,
}

impl Solution {
    pub fn eventual_safe_nodes(graph: Vec<Vec<i32>>) -> Vec<i32> {
        let n = graph.len();
        let mut node_state = vec![NodeState::Unseen; n];
        (0..n)
            .filter(|&i| Self::dfs(i, &graph, &mut node_state))
            .map(|i| i as i32)
            .collect()
    }

    fn dfs(node: usize, graph: &[Vec<i32>], node_state: &mut [NodeState]) -> bool {
        if NodeState::Unseen == node_state[node] {
            node_state[node] = NodeState::Processing;

            for &neighbor in &graph[node] {
                let neighbor = neighbor as usize;
                if !Self::dfs(neighbor, graph, node_state) {
                    return false;
                }
            }

            node_state[node] = NodeState::Safe;
        }
        matches!(node_state[node], NodeState::Safe)
    }
}