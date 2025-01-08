use std::collections::HashSet;

impl Solution {
    pub fn equal_digit_frequency(s: String) -> i32 {
        let n = s.len();
        let bytes = s.as_bytes();
        let mut valid_factors = vec![Vec::new(); n + 1]; 
        
        for L in 1..=n {
            for k in 1..=10 {
                if L % k == 0 {
                    let m = L / k;
                    valid_factors[L].push((k, m));
                }
            }
        }

        let mut result_set = HashSet::new();
        
        for L in 1..=n {
            let factors = &valid_factors[L];
            if factors.is_empty() {
                continue; 
            }
            let mut freq = [0; 10];
            for i in 0..L {
                let digit = (bytes[i] - b'0') as usize;
                freq[digit] += 1;
            }
            Self::check_and_insert(&freq, factors, &s[0..L], &mut result_set);

            for i in 0..(n - L) {
                let left_digit = (bytes[i] - b'0') as usize;
                let right_digit = (bytes[i + L] - b'0') as usize;
                freq[left_digit] -= 1;
                freq[right_digit] += 1;

                Self::check_and_insert(&freq, factors, &s[i+1..i+1+L], &mut result_set);
            }
        }

        result_set.len() as i32
    }

    fn check_and_insert(
        freq: &[i32; 10],
        factors: &[(usize, usize)], 
        sub: &str,
        set: &mut HashSet<String>,
    ) {
        for &(k, m) in factors {
            if is_valid_k_m(freq, k, m) {
                set.insert(sub.to_string());
                break;
            }
        }
    }
}

fn is_valid_k_m(freq: &[i32; 10], k: usize, m: usize) -> bool {
    let mut count_nonzero = 0;
    for &count in freq {
        if count > 0 {
            if count as usize != m {
                return false;
            }
            count_nonzero += 1;
            if count_nonzero > k {
                return false;
            }
        }
    }
    count_nonzero == k
}