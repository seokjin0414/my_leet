use std::cmp::Ordering;

impl Solution {
    pub fn valid_tree(n: i32, edges: Vec<Vec<i32>>) -> bool {
        let mut idx = 0;
        let mut union_find = UnionFind::new(n as usize);

        while idx < edges.len() {
            let (a, b) = (edges[idx][0], edges[idx][1]);
            if !union_find.union(a, b) {
                return false;
            }

            idx += 1;
        }

        union_find.count == 1
    }
}

pub struct UnionFind {
    pub root: Vec<i32>,
    pub rank: Vec<i32>,
    pub count: i32,
}

impl UnionFind {
    pub fn new(size: usize) -> Self {
        UnionFind { 
            root: (0i32..(size as i32)).collect(), 
            rank: vec![1i32; size], 
            count: size as i32 
        }
    }

    pub fn find(&mut self, node: i32) -> i32 {
        let parent = self.root[node as usize];
        if node == parent { 
            return node; 
        }

        let parent = self.find(parent);
        self.root[node as usize] = parent;
        parent
    }

    pub fn union(&mut self, x: i32, y: i32) -> bool {
        let rx = self.find(x);
        let ry = self.find(y);

        if rx == ry { return false }
        self.count -= 1;

        match self.rank[rx as usize].cmp(&self.rank[ry as usize]) {
            Ordering::Greater => self.root[ry as usize] = rx,
            Ordering::Less    => self.root[rx as usize] = ry,
            Ordering::Equal   => {
                self.root[ry as usize] = rx;
                self.rank[rx as usize] += 1;
            },
            
        }

        true
    }

    pub fn connected(&mut self, x: i32, y: i32) -> bool {
        self.find(x) == self.find(y)
    }
}
