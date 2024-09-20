impl Solution {
    pub fn gcd_of_strings(str1: String, str2: String) -> String {
        if str1 == str2 { return str1; }

        let mut result:String = String::new();
        let letters1 = str1.chars().collect::<Vec<char>>();
        let letters2 = str2.chars().collect::<Vec<char>>();
        
        for word_length in 1..=str1.len().min(str2.len()) {
            let substring = &letters1[..word_length];

            if str1.len() % substring.len() != 0 || str2.len() % substring.len() != 0 { continue; }
            if !letters1.chunks(substring.len()).all(|chunk| chunk == substring ) { continue; }
            if !letters2.chunks(substring.len()).all(|chunk| chunk == substring ) { continue; }

            result = substring.iter().collect::<String>();
        }
        
        result
    }
}
