impl Solution {
    pub fn are_almost_equal(s1: String, s2: String) -> bool {

        let mut swapc = (' ', ' ');
        for (c1, c2) in s1.chars().zip(s2.chars()){
            if c1 != c2 {
                if swapc.0 == ' '{
                    swapc = (c1, c2);
                }else if swapc != (c2, c1) {
                    return false;
                }else {
                    swapc.0 = '0';
                }
            }
        }
        return swapc.0 == ' '||swapc.0 == '0';
    }
}