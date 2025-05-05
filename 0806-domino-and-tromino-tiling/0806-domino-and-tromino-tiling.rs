impl Solution {
    pub fn num_tilings(n: i32) -> i32 {
        let z = 1000_000_007;
        let (mut empty, mut single, mut both) = (0u32, 0u32, 1u32);
        for _ in 0..n {
            let (e, s, b) = (empty, single, both);
            empty = b;
            single = (e + s) % z;
            both = (e + 2 * s + b) % z;
        }
        both as i32
    }
}