use std::collections::VecDeque;

impl Solution {
    pub fn magnificent_sets(n: i32, edges: Vec<Vec<i32>>) -> i32 {
        let n = n as usize;
        let mut graph = vec![vec![]; n];

        for edge in edges {
            let (u, v) = ((edge[0] - 1) as usize, (edge[1] - 1) as usize);
            graph[u].push(v);
            graph[v].push(u);
        }

        let mut heights = vec![None; n];
        let mut total_diameters = 0;
        for start in 0..n {
            match Self::bfs_check_and_find_height(start, &graph, &mut heights) {
                (false, _) => return -1,
                (true, Some(d)) => total_diameters += d,
                _ => {}
            }
        }

        total_diameters
    }

    fn bfs_check_and_find_height(start: usize, graph: &[Vec<usize>], heights: &mut [Option<i32>]) -> (bool, Option<i32>) {
        let mut level = vec![0 as i32; graph.len()]; // Track BFS levels
        let mut queue = VecDeque::new();
        queue.push_back(start);
        level[start] = 1;

        let mut max_height = -1;
        let mut max_level = 1;
        let mut all_heights_assigned = true;

        while let Some(v) = queue.pop_front() {
            max_level = level[v];
            
            if v != start && heights[v].is_none() {
                all_heights_assigned = false;
            } else {
                max_height = max_height.max(heights[v].unwrap_or(-1));
            }

            for &u in &graph[v] {
                if level[u] == 0 {
                    level[u] = level[v] + 1;
                    queue.push_back(u);
                } else if level[u].abs_diff(level[v]) != 1 {
                    return (false, None);
                }
            }
        }

        heights[start] = Some(max_level);
        
        if all_heights_assigned {
            (true, Some(max_height.max(max_level)))
        } else {
            (true, None)
        }
    }
}