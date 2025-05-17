impl Solution {
    pub fn sort_colors(nums: &mut [i32]) {
        nums.iter()
            .fold([0; 3], |mut f, c| {
                f[*c as usize] += 1;
                f
            })
            .into_iter()
            .zip(0..)
            .flat_map(|(cc, c)| std::iter::repeat_n(c, cc))
            .zip(nums.iter_mut())
            .for_each(|(nc, oc)| *oc = nc);
    }
}