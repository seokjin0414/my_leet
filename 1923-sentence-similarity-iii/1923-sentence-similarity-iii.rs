impl Solution {
    pub fn are_sentences_similar(sentence1: String, sentence2: String) -> bool {
        let (sent1,sent2) = (sentence1.split(" ").collect::<Vec<&str>>(),sentence2.split(" ").collect::<Vec<&str>>());
        if sentence1 == sentence2 { return true }
        let mut prefix = Solution::get_longest_prefix(&sent1, &sent2);
        let mut suffix = Solution::get_longest_suffix(&sent1[prefix.len()..], &sent2[prefix.len()..]);
        prefix.append(&mut suffix);
        let concat = prefix.join(" ");
        concat == sentence1 || concat == sentence2
    }
    fn get_longest_prefix<'a>(sentence1: &'a [&'a str], sentence2: &'a [&'a str]) -> Vec<&'a str> {
        for idx in (1..sentence1.len().min(sentence2.len())+1).rev() {
            if sentence1[0..idx] == sentence2[0..idx] {
                return sentence1[0..idx].to_vec();
            }
        }
        vec![]
    }
    fn get_longest_suffix<'a>(sentence1: &'a [&'a str], sentence2: &'a [&'a str]) -> Vec<&'a str> {
        let mut result = vec![];
        for idx in 0..sentence1.len().min(sentence2.len()) {
            if sentence1[sentence1.len()-idx-1] == sentence2[sentence2.len()-idx-1] {
                result.insert(0,sentence1[sentence1.len()-idx-1]);                
            } else {
                break;
            }
        }
        result
    }
}