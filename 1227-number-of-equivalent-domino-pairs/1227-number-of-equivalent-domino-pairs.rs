impl Solution {
    pub fn num_equiv_domino_pairs(dominoes: Vec<Vec<i32>>) -> i32 {
        dominoes
            .into_iter()
            .map(|d| [d[0].min(d[1]), d[0].max(d[1])])
            .fold(std::collections::HashMap::new(), |mut map, d| {
                *map.entry(d).or_default() += 1;
                map
            })
            .into_values()
            .map(|c: i32| (c * (c - 1)) >> 1)
            .sum()
    }
}