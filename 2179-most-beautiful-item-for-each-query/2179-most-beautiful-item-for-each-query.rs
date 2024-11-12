impl Solution {
    pub fn maximum_beauty(mut items: Vec<Vec<i32>>, queries: Vec<i32>) -> Vec<i32> {
        items.sort_unstable_by_key(|k| (std::cmp::Reverse(k[1]), k[0]));
        items.dedup_by(|a, b| a[0] >= b[0]);
        
        queries.into_iter()
            .map(|q| match items.binary_search_by(|p| q.cmp(&p[0])) {
                Ok(i) | Err(i) => items.get(i).map_or(0, |k| k[1]),
            })
            .collect()
    }
}