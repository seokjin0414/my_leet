impl Solution {
    pub fn prime_sub_operation(mut nums: Vec<i32>) -> bool {
        (0..nums.len() - 1).rev().all(|i| {
            if nums[i + 1] > nums[i] {
                return true;
            }
            let Some(prime) = Self::PRIMES
                .iter()
                .find(|&&prime| nums[i] > prime && prime > nums[i] - nums[i + 1])
            else {
                return false;
            };
            nums[i] -= prime;
            true
        })
    }

    const PRIMES: [i32; 168] = {
        let mut result = [0; 168];
        result[0] = 2;
        result[1] = 3;
        let mut i = 2;
        while i < result.len() {
            let mut p = result[i - 1] + 2;
            while Self::is_not_prime(p) {
                p += 2;
            }
            result[i] = p;
            i += 1;
        }
        result
    };

    const fn is_not_prime(n: i32) -> bool {
        if n & 1 == 0 {
            return true;
        }
        let mut i = 3;
        while i * i <= n {
            if n % i == 0 {
                return true;
            }
            i += 2;
        }
        false
    }
}