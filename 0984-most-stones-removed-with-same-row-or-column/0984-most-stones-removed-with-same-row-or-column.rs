use std::collections::HashMap;
const OFFSET: i32 = 10001;

struct DSU {
    parent: HashMap<i32, i32>,
    sizes: HashMap<i32, i32>,
    pub sets: i32,
}

impl DSU {
    pub fn new() -> Self {
        DSU {parent: HashMap::new(), sizes: HashMap::new(), sets: 0}
    }

    fn find(&mut self, a: i32) -> i32 {
        let ap = *self.parent(a);
        if ap != a {
            *self.parent(a) = self.find(ap);
        }
        *self.parent(a)
    }

    pub fn insert(&mut self, a: i32, b: i32) {
        let (ap, bp) = (self.find(a), self.find(b));
        let (apsz, bpsz) = (*self.size(ap), *self.size(bp));

        if ap == bp {
            return;
        }

        if apsz < bpsz {
            *self.size(bp) += apsz;
            *self.parent(ap) = bp;
        } else {
            *self.size(ap) += bpsz;
            *self.parent(bp) = ap;
        }

        self.sets -= 1;
    }

    fn together(&mut self, a: i32, b: i32) -> bool {
        self.find(a) == self.find(b)
    }

    fn parent(&mut self, node: i32) -> &mut i32 {
        self.parent.entry(node).or_insert_with(|| {
            self.sets += 1;
            node
        })
    }

    fn size(&mut self, node: i32) -> &mut i32 {
        self.sizes.entry(node).or_insert(1)
    }
}

impl Solution {
    pub fn remove_stones(stones: Vec<Vec<i32>>) -> i32 {
        let mut g = DSU::new();
        let n = stones.len() as i32;
        for coord in stones {
            let [x, y] = coord[..] else { return 0; };
            g.insert(x, y + OFFSET);
            println!("components: {}", g.sets);
        }
        
        n - g.sets
    }
}