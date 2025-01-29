impl Solution {
    pub fn find_redundant_connection(edges: Vec<Vec<i32>>) -> Vec<i32> {
        let n = edges.len();
        let mut parent = vec![0; n+1];
        let mut rank = vec![0; n+1];
        
        for i in 0..=n {
            parent[i] = i;
        }

        for edge in edges.iter() {
            if !Self::union(&mut parent, &mut rank, edge[0] as usize, edge[1] as usize) {
                return edge.to_vec();
            }
        }
        vec![]
    }

    fn find(parent: &mut Vec<usize>, x: usize) -> usize {
        if parent[x] != x {
            parent[x] = Self::find(parent, parent[x]);
        }
        parent[x]
    }

    fn union(parent: &mut Vec<usize>, rank: &mut Vec<usize>, x: usize, y: usize) -> bool {
        let px = Self::find(parent, x);
        let py = Self::find(parent, y);
        
        if px == py {
            return false;
        }

        if rank[px] < rank[py] {
            parent[px] = py;
        } else if rank[px] > rank[py] {
            parent[py] = px;
        } else {
            parent[py] = px;
            rank[px] += 1;
        }
        true
    }
}