impl Solution {
    pub fn push_dominoes(dominoes: String) -> String {
        let mut ds = dominoes.into_bytes();
        let mut l = 0;
        let mut r = 0;
        while l + 1 != ds.len(){
            while r + 1 < ds.len() && ds[r] == b'.' {
                r+=1;
            }
            match (ds[l], ds[r]){
                (b'R', b'L')=>{
                    let h = (r-l+1)/2;
                    &mut ds[l..l+h].fill(b'R');
                    &mut ds[r-h+1..=r].fill(b'L');
                },
                (b'R', _)=>{&mut ds[l..=r].fill(b'R');},
                (_, b'L')=>{&mut ds[l..=r].fill(b'L');},
                _=>{}
            }
            l = r;
            r+=1;
        } 
        String::from_utf8(ds).unwrap()
    }
}