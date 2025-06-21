impl Solution {
    pub fn minimum_deletions(word: String, k: i32) -> i32 {
        let mut counts = [0; 26];
        for c in word.chars() {
            counts[c as usize - 'a' as usize] += 1
        }
        let mut min1 = 100000;
        let mut max1 = 0;
        for &count in &counts {
            if count > 0 {
                if min1 > count {
                    min1 = count;
                }
                if max1 < count {
                    max1 = count;
                }
            }
        }
        let mut ret = 100000;
        for &i in &counts {
            if i > 0 {
                let mut tmp = 0;
                for &j in &counts {
                    if j < i {
                        tmp += j;
                    }
                    if j > i + k {
                        tmp += j - i - k;
                    }
                }
                if ret > tmp {
                    ret = tmp;
                }
            }
        }
        ret
    }
}