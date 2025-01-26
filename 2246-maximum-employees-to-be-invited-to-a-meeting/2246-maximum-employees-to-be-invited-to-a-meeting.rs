use std::collections::VecDeque;

impl Solution {
    pub fn maximum_invitations(favorite: Vec<i32>) -> i32 {
        let n = favorite.len();
        let mut visited = vec![false; n];
        let mut to_leaf = vec![0; n];
        let mut in_degree = vec![0; n];

        for &fav in favorite.iter() {
            in_degree[fav as usize] += 1;
        }

        let mut queue = VecDeque::new();
        for (i, &deg) in in_degree.iter().enumerate() {
            if deg == 0 {
                queue.push_back(i);
                visited[i] = true;
            }
        }

        while let Some(node) = queue.pop_front() {
            let next = favorite[node] as usize;
            to_leaf[next] = to_leaf[next].max(1 + to_leaf[node]);
            in_degree[next] -= 1;
            if in_degree[next] == 0 {
                visited[next] = true;
                queue.push_back(next);
            }
        }

        let (mut max_cycle, mut pairs_with_chains) = (0, 0);
        for i in 0..n {
            if !visited[i] {
                let mut len = 0;
                let mut curr = i;

                while !visited[curr] {
                    visited[curr] = true;
                    len += 1;
                    curr = favorite[curr] as usize;
                }

                if len == 2 {
                    pairs_with_chains += 2 + to_leaf[i] + to_leaf[favorite[i] as usize];
                } else {
                    max_cycle = max_cycle.max(len);
                }
            }
        }

        max_cycle.max(pairs_with_chains)
    }
}