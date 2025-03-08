impl Solution {
    pub fn minimum_recolors(blocks: String, k: i32) -> i32 {
        let blocks: Vec<_> = blocks.chars().collect(); 
        let k = k as usize; 
        let mut min_ops = blocks[..k].iter().filter(|&&c| c == 'W').count(); 
        let mut current_ops = min_ops; 

        for i in k..blocks.len() { 
            if blocks[i] == 'W' {
                current_ops += 1; 
            }
            if blocks[i - k] == 'W' {
                current_ops -= 1; 
            }
            min_ops = min_ops.min(current_ops); 
        }

        min_ops as i32 
    }
}