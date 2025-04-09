pub struct UnionFind{
    pub parent: Vec<i32>,
    pub rank: Vec<i32>,
}

impl UnionFind{
    pub fn new(n: usize) -> Self{
        Self{
            parent: (0..n as i32).collect(),
            rank: vec![0; n],
        }
    }

    pub fn find(&mut self, x: i32) -> i32{
        if self.parent[x as usize] != x{
            self.parent[x as usize] = self.find(self.parent[x as usize])
        }
        self.parent[x as usize]
    }

    pub fn union_set(&mut self, x: i32, y:i32, count: &mut i32) {
        let x_parent = self.find(x) as usize;
        let y_parent = self.find(y) as usize;

        if x_parent == y_parent{
            return;
        }

        *count -= 1;
        if self.rank[x_parent] < self.rank[y_parent]{
            self.parent[x_parent] = self.parent[y_parent];
        } else if self.rank[x_parent] > self.rank[y_parent]{
            self.parent[y_parent] = self.parent[x_parent];
        }else{
            self.parent[y_parent] = self.parent[x_parent];
            self.rank[x_parent] += 1
        }
    }
}

impl Solution {
    pub fn count_components(n: i32, edges: Vec<Vec<i32>>) -> i32 {
        let mut uf = UnionFind::new(n as usize);

        let mut count = n;

        for edge in edges{
            uf.union_set(edge[0], edge[1], &mut count);
        }

        count
    }
}