impl Solution {
    pub fn prefix_count(words: Vec<String>, pref: String) -> i32 {
        let mut ans = 0;
        for word in words{
            if (word.starts_with(&pref)){
                ans+=1;
            }
        }
        ans
    }
}