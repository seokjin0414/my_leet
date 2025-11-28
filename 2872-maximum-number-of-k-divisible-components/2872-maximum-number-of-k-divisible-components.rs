impl Solution {
    pub fn max_k_divisible_components(n: i32, edges: Vec<Vec<i32>>, values: Vec<i32>, k: i32) -> i32 {
    let mut adj: Vec<Vec<i32>> = vec![vec![]; n as usize];

    for edge in edges.iter() {
        adj[edge[0] as usize].push(edge[1]);
        adj[edge[1] as usize].push(edge[0]);
    }

    let mut visited = vec![false; n as usize];
    let mut ans = 0;

    fn dfs(
        root: i32,
        values: &Vec<i32>,
        k: i32,
        adj: &Vec<Vec<i32>>,
        visited: &mut Vec<bool>,
        ans: &mut i32,
    ) -> i32 {
        let mut sum = values[root as usize];
        visited[root as usize] = true;
        for child in adj[root as usize].iter() {
            if !visited[*child as usize] {
                sum += dfs(*child, values, k, adj, visited, ans);
                sum %= k;
            }
        }

        if sum % k == 0 {
            *ans += 1;
            0
        } else {
            sum
        }
    }

    dfs(0, &values, k, &adj, &mut visited, &mut ans);

    ans
}
}