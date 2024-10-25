impl Solution {
	pub fn remove_subfolders(mut folder: Vec<String>) -> Vec<String> {
		folder.sort_unstable();
        let mut i = 0;
		
        for j in (0..folder.len()) {
            match &folder[..i] {
                [.., prv] if !(folder[j].starts_with(prv) && folder[j][prv.len()..].starts_with("/")) => {
                    folder.swap(i, j);
                    i += 1;
                }
                [] => i += 1,
                _ => {},
            }
		}
        
        folder.truncate(i);
        folder
	}
}