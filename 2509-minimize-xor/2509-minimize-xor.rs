impl Solution {
    pub fn minimize_xor(num1: i32, num2: i32) -> i32 {
         let ones1 = num1.count_ones();
         let ones2 = num2.count_ones();

         if ones1 >= ones2 {
            (0..32)
              .rev()
              .map(|i| (i, (num1 >> i) & 1))
              .filter(|&(_, bit)| bit == 1)
              .take(ones2 as usize)
              .fold(0_i32, |res, (i, bit)| res | (1 << i))
         } else {
            (0..32)
              .map(|i| (i, (num1 >> i) & 1))
              .fold((ones2 - ones1, 0_i32), |(extra_bits, res), (i, bit)| {
                if bit == 1 {
                    (extra_bits, res | (1 << i))
                } else if extra_bits > 0 {
                    (extra_bits - 1, res | (1 << i))
                } else {
                    (extra_bits, res)
                }
              })
              .1
         }
    }
}