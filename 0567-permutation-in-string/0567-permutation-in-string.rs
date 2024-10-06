impl Solution {
    pub fn check_inclusion(s1: String, s2: String) -> bool {
        if s1.len() > s2.len() { return false; }

        let mut book: Vec<usize> = vec![0;27];
        let s2_letters: Vec<char> = s2.chars().collect();

        s1.chars().for_each(|letter| book[letter as usize -97] +=1);

        (0..s1.len()).for_each(|index| book[s2_letters[index]as usize -97] -=1);

        if book.iter().all(|num| *num == 0) { return true; }

        for index in s1.len()..s2.len() {
            book[s2_letters[index - s1.len()]as usize -97] +=1;
            book[s2_letters[index]as usize -97] -= 1;
            if book.iter().all(|num| *num == 0) { return true; }
        }
        false
    }
}