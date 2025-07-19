use std::collections::HashMap;

pub struct TrieNode {
    children: HashMap<char, Box<TrieNode>>,
    is_end: bool,
}

impl TrieNode {
    pub fn new() -> Self {
        Self {
            children: HashMap::new(),
            is_end: false,
        }
    }
}

pub struct Trie {
    root: TrieNode
}

impl Trie {
    fn new() -> Self{
        Self{
            root: TrieNode::new()
        }
    }

    fn insert(&mut self, word: &str) {
        let mut node = &mut self.root;
        for ch in word.chars() {
            node = node.children.entry(ch).or_insert_with(|| Box::new(TrieNode::new()));
        }
        node.is_end = true;
    }

    fn get_words_with_prefix(&self, prefix: &str) -> Vec<String> {
        let mut node = &self.root;
        for ch in prefix.chars(){
            match node.children.get(&ch) {
                Some(child) => node = child,
                None => return vec![],
            }
        }

        // at this point node is the end of prefix
        let mut cur = prefix.to_string();
        let mut res = Vec::new();

        Self::dfs(node,&mut cur, &mut res);

        return res;
    }

    fn dfs(node: &TrieNode, cur: &mut String, res: &mut Vec<String>) {
        if node.is_end {
            res.push(cur.clone());
        }

        for (&ch, child) in &node.children {
            cur.push(ch);
            Self::dfs(child, cur, res);
            cur.pop();
        }
    }
}

impl Solution {
    pub fn word_squares(words: Vec<String>) -> Vec<Vec<String>> {
        let mut trie = Trie::new();
        for word in words.iter(){
            trie.insert(&word);
        }

        let mut ans = vec![];
        for word in words{
            let mut cur = vec![word];
            Self::backtrack(&mut cur, &trie, &mut ans);
        }

        ans
    }

    pub fn backtrack(cur: &mut Vec<String>, trie: &Trie, ans: &mut Vec<Vec<String>>) {
        let n = cur.len();
        if n == cur[0].len(){
            ans.push(cur.clone());
            return;
        }

        let prefix: String = cur.iter().map(|w| w.as_bytes()[n] as char).collect();
        let valid_words = trie.get_words_with_prefix(&prefix);
        for word in valid_words{
            cur.push(word);
            Self::backtrack(cur, trie, ans);
            cur.pop();
        }
    }
}