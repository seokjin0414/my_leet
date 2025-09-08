impl Solution {
    pub fn get_no_zero_integers(n: i32) -> Vec<i32> {
        let mut first = 1;
        let mut second = n-first;
        while first < n && first.to_string().contains("0") || second.to_string().contains("0") {
            first +=1;
            second = n-first;
        }
        vec![first,second]
    }
}