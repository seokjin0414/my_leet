use std::collections::{HashMap, HashSet};
impl Solution {
    pub fn spellchecker(wordlist: Vec<String>, queries: Vec<String>) -> Vec<String> {
        let rv = |s: &String| -> String {
            s.chars()
                .map(|c| match "eiou".contains(c) {
                    true => 'a',
                    false => c,
                })
                .collect()
        };
        let em = wordlist.iter().collect::<HashSet<_>>();
        let lc = wordlist
            .iter()
            .zip(0..wordlist.len() as i32)
            .rev()
            .map(|(w, i)| (w.to_lowercase(), i))
            .collect::<HashMap<_, _>>();
        let vs = lc.iter().fold(HashMap::new(), |mut map, (w, &i)| {
            map.entry(rv(w)).and_modify(|v| *v = i.min(*v)).or_insert(i);
            map
        });
        queries
            .into_iter()
            .map(|q| {
                if em.contains(&q) {
                    return q;
                }
                let ql = q.to_lowercase();
                if let Some(&p) = lc.get(&ql) {
                    return wordlist[p as usize].to_owned();
                }
                if let Some(&p) = vs.get(&rv(&ql)) {
                    return wordlist[p as usize].to_owned();
                }
                String::new()
            })
            .collect()
    }
}