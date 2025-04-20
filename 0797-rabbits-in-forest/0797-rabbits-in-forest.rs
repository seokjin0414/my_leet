impl Solution {
    pub fn num_rabbits(mut answers: Vec<i32>) -> i32 {
        answers.sort_unstable();
        answers.chunk_by(|a, b| a == b).fold(0, |sum, chunk| {
            let v = chunk[0] + 1;
            sum + (chunk[0] + chunk.len() as i32) / v * v
        })
    }
}