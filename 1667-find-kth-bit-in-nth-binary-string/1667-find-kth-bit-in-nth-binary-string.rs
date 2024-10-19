impl Solution {
    fn inverter(invert: bool, c: char) -> char {
        if invert {
            match c {
                '1' => '0',
                '0' => '1',
                _ => 'x',
            }
        }else{
            c
        }
    }

    pub fn find_kth_bit(n: i32, k: i32) -> char {
        let mut mid = 1 << n - 1;
        let mut k = k;
        let mut invert = false;
        loop {
            if mid == 1 { return Self::inverter(invert, '0'); }
            if mid == k { return Self::inverter(invert, '1'); }
            if k > mid {
                k = 2 * mid - k;
                invert = !invert;
            }
            mid >>= 1;
        }
    }
}