use std::collections::{HashMap, HashSet};

impl Solution {
    pub fn count_complete_components(n: i32, edges: Vec<Vec<i32>>) -> i32 {
        let n = n as usize;
        let mut parent: Vec<usize> = (0..n).collect();
        let mut rank = vec![0; n];

        fn find(parent: &mut Vec<usize>, x: usize) -> usize {
            if parent[x] != x {
                parent[x] = find(parent, parent[x]);
            }
            parent[x]
        }

        fn union(parent: &mut Vec<usize>, rank: &mut Vec<usize>, x: usize, y: usize) {
            let root_x = find(parent, x);
            let root_y = find(parent, y);
            if root_x == root_y {
                return;
            }
            if rank[root_x] < rank[root_y] {
                parent[root_x] = root_y;
            } else if rank[root_x] > rank[root_y] {
                parent[root_y] = root_x;
            } else {
                parent[root_y] = root_x;
                rank[root_x] += 1;
            }
        }

        for edge in &edges {
            union(&mut parent, &mut rank, edge[0] as usize, edge[1] as usize);
        }

        let mut component_vertices: HashMap<usize, HashSet<usize>> = HashMap::new();
        let mut component_edges: HashMap<usize, i32> = HashMap::new();

        for i in 0..n {
            let root = find(&mut parent, i);
            component_vertices.entry(root).or_insert_with(HashSet::new).insert(i);
        }

        for edge in &edges {
            let root = find(&mut parent, edge[0] as usize);
            *component_edges.entry(root).or_insert(0) += 1;
        }

        let mut complete_count = 0;
        for (root, vertices) in component_vertices.iter() {
            let num_vertices = vertices.len();
            let expected_edges = (num_vertices * (num_vertices - 1)) / 2;
            if component_edges.get(root).unwrap_or(&0) == &(expected_edges as i32) {
                complete_count += 1;
            }
        }

        complete_count
    }
}