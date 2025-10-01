impl Solution {
    pub fn num_water_bottles(mut num_bottles: i32, num_exchange: i32) -> i32 {
        let mut rslt = 0;
        let mut empty_bottles = 0;
        while num_bottles > 0 {
            rslt += num_bottles;
            empty_bottles += num_bottles % num_exchange;
            num_bottles = num_bottles / num_exchange + empty_bottles / num_exchange;
            empty_bottles %= num_exchange;
        }
        rslt
    }
}