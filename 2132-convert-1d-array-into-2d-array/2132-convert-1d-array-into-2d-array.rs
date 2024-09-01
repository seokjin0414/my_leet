impl Solution {
    pub fn construct2_d_array(original: Vec<i32>, m: i32, n: i32) -> Vec<Vec<i32>> {
        if (m * n) as usize != original.len() {
            return Vec::new(); 
        }
        
        let mut vec = Vec::with_capacity(m as usize);
        
      
        for i in 0..m {
            let start = (i * n) as usize;
            let end = start + n as usize;
            vec.push(original[start..end].to_vec());
        }
        
        vec
    }
}