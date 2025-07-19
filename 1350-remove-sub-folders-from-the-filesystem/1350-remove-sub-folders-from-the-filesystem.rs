impl Solution {
    pub fn remove_subfolders(mut folder: Vec<String>) -> Vec<String> {
        let mut res: Vec<String> = Vec::new();
        folder.sort();
        for key in folder {
            if res.is_empty(){
                res.push(key);
                continue;
            }
            if key.len() < res.last().unwrap().len() ||
                &key[..res.last().unwrap().len()] != *res.last().unwrap() ||
                key.chars().nth(res.last().unwrap().len()) != Some('/') {
                res.push(key);
            }
        }
        res
    }
}