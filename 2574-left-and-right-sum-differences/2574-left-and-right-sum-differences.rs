impl Solution {
    fn left_right_difference(nums: Vec<i32>) -> Vec<i32> {
        let mut answer = vec![0; nums.len()];

        for i in 0..nums.len() {
            let left: i32 = nums.iter().take(i).sum();
            let right: i32 = nums.iter().skip(i + 1).sum();

            answer[i] = (left - right).abs();
        }

        answer
    }
}