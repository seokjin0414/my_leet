impl Solution {
    pub fn push_dominoes(dominoes: String) -> String {
        let mut dominoes = dominoes.into_bytes();
        let mut prev = 0;
        let mut right = false;
        for i in 0..dominoes.len() {
            match dominoes[i] {
                b'L' => {
                    if right {
                        for j in prev + 1..(prev + i + 1) / 2 {
                            dominoes[j] = b'R';
                        }
                        for j in (prev + i + 2) / 2..i {
                            dominoes[j] = b'L';
                        }
                        prev = i;
                        right = false;
                    } else {
                        for j in prev..i {
                            dominoes[j] = b'L';
                        }
                        prev = i;
                    }
                },
                b'R' => {
                    if right {
                        for j in prev + 1..i {
                            dominoes[j] = b'R';
                        }
                    }
                    prev = i;
                    right = true;
                },
                _ => {},
            }
        }
        if right {
            for j in prev + 1..dominoes.len() {
                dominoes[j] = b'R';
            }
        }
        String::from_utf8(dominoes).unwrap()
    }
}