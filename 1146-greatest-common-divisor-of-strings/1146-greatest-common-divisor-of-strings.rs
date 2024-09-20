impl Solution {
    pub fn gcd_of_strings(str1: String, str2: String) -> String {
        if str1.clone() + &str2 == str2.clone() + &str1 {
            let gcd_len = gcd(str1.len(), str2.len());
            str1[..gcd_len].to_string()
        } else {
            "".to_string()
        }
    }
}
fn gcd(mut a: usize, mut b: usize) -> usize {
    while b != 0 {
        if b < a {
            (a, b) = (b, a);
        }
        b %= a;
    }
    a
}