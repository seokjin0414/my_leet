impl Solution {
	pub fn array_rank_transform(mut arr: Vec<i32>) -> Vec<i32> {
		let mut ranked = arr.clone();
		ranked.sort_unstable();
        ranked.dedup();

		for n in &mut arr {
			*n = ranked.partition_point(|&x| x < *n) as i32 + 1;
		}
		arr
	}
}