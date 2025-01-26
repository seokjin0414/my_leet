impl Solution {
    pub fn maximum_invitations(favorite: Vec<i32>) -> i32 {
        let n = favorite.len();
        let mut dgree = vec![0;n];
        let mut dep = vec![0;n];
        for &p in favorite.iter(){
            dgree[p as usize] += 1;
        }
        let mut q = vec![];
        for i in 0..n{
            if dgree[i] == 0 {
                q.push(i as usize);
            }
        }
        while q.len() !=0 {
            let x = q.pop().unwrap();
            let y = favorite[x as usize] as usize;
            dgree[y] -=1 ;
            dep[y] = dep[y].max(dep[x]+1);

            if dgree[y] == 0{
                q.push(y);
            }
        }
        let mut ans = 0;
        let mut two = 0; 
        for i in 0..n{
            if dgree[i] != 0 {
                dgree[i] = 0;
                let mut j = favorite[i] as usize;
                let mut tmp = 1;
                while dgree[j]!=0{
                    tmp+=1;
                    dgree[j]=0;
                    j = favorite[j] as usize;
                }
                if tmp == 2 {
                    two += (2 + dep[i] + dep[favorite[i] as usize]);
                }else {
                    ans = ans.max(tmp);
                }
            }
        }
        ans = ans.max(two);
        ans
    }
}