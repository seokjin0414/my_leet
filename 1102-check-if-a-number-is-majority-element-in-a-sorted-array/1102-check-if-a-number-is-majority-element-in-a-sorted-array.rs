impl Solution {
    pub fn is_majority_element(vec: Vec<i32>, target: i32) -> bool {        
        let p0 = vec.partition_point(|&e| e < target);
        let p1 = vec.partition_point(|&e| e <= target);
        
        p1 - p0 > vec.len() / 2
    }
}