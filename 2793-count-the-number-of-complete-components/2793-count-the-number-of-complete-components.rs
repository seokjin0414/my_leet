use std::collections::{HashMap, HashSet};

impl Solution {
    pub fn count_complete_components(n: i32, edges: Vec<Vec<i32>>) -> i32 {
        let n = n as usize;
        let mut links: HashMap<usize, Vec<usize>> = HashMap::new();
        for edge in edges {
            let from = edge[0] as usize;
            let to = edge[1] as usize;
            links.entry(from).or_default().push(to);
            links.entry(to).or_default().push(from);
        }

        let mut result = 0;
        let mut nodes: HashSet<usize> = (0..n).collect();
        while let Some(&node) = nodes.iter().next() {
            let (mut ncount, mut ecount) = (0, 0);
            let mut queue = vec![node];
            nodes.remove(&node);
            while let Some(node) = queue.pop() {
                ncount += 1;
                for node in links.entry(node).or_default() {
                    ecount += 1;
                    if nodes.contains(node) {
                        nodes.remove(&node);
                        queue.push(*node);
                    }
                }
            }
            if ecount == ncount * (ncount - 1) { result += 1; }
        }
        result
    }
}