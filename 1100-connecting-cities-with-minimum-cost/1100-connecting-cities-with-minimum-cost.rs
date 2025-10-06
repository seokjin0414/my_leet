use std::cmp::Ordering;

struct UnionFind {
    parents: Vec<usize>,
    ranks: Vec<usize>,
    groups: usize,
    cost: i32,
}

impl UnionFind {

    fn new(n: usize) -> Self {
        let parents = (0..n).collect();
        let ranks = vec![1; n];
        let groups = n;
        let cost = 0;
        Self { parents, ranks, groups, cost }
    }

    fn find(&mut self, x: usize) -> usize {
        if self.parents[x] != x {
            self.parents[x] = self.find(self.parents[x]);
        }
        return self.parents[x];
    }

    fn union(&mut self, x: usize, y: usize, cost: i32) -> () {
        let px = self.find(x);
        let py = self.find(y);
        if px == py { return }
        self.groups -= 1;
        self.cost += cost;
        match self.ranks[px].cmp(&self.ranks[py]) {
            Ordering::Less => self.parents[px] = py,
            Ordering::Greater => self.parents[py] = px,
            Ordering::Equal => {
                self.parents[px] = py;
                self.ranks[py] += 1;
            },
        }
    }
}


impl Solution {
    pub fn minimum_cost(n: i32, connections: Vec<Vec<i32>>) -> i32 {

        let mut uf = UnionFind::new(n as usize);

        let mut connections = connections;
        connections.sort_unstable_by_key(|k| k[2]);
        for con in connections {
            let x = con[0] as usize - 1;
            let y = con[1] as usize - 1;
            let cost = con[2];
            
            uf.union(x, y, cost);

        }

        if uf.groups != 1 { return -1 }
        return uf.cost;
    }
}