impl Solution {
    pub fn count_complete_components(n: i32, edges: Vec<Vec<i32>>) -> i32 {
        let n = n as usize;
        let mut parent = (0..n).into_iter().collect::<Vec<_>>();
        for e in edges.iter() {
            let a = e[0] as usize;
            let b = e[1] as usize;
            let pa = find(a, &mut parent);
            let pb = find(b, &mut parent);
            parent[pa] = pb;
        }
        let mut vts = vec![0; n];
        let mut ans = 0;
        for i in 0..n {
            let r = find(i, &mut parent);
            vts[r] += 1;
        }
        let mut eds = vec![0; n];
        for e in edges.iter() {
            eds[find(e[0] as usize, &mut parent)] += 1;
        }
        for i in 0..n {
            let vc = vts[i];
            if vc > 0 {
                let ec = eds[i];
                if ec == vc * (vc - 1) / 2 {
                    ans += 1;
                }
            }
        }
        ans
    }
}

fn find(i: usize, parent: &mut Vec<usize>) -> usize {
    if parent[i] != i {
        parent[i] = find(parent[i], parent);
    }
    parent[i]
}