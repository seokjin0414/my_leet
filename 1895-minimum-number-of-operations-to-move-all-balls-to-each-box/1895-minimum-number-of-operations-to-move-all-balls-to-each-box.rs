impl Solution {
	pub fn min_operations(boxes: String) -> Vec<i32> {
		let mut ans = vec![];
		let (mut stp, mut cnt) = (0, 0);
		
        for c in boxes.bytes() {
			(stp, cnt) = (stp + cnt, cnt + (c == b'1') as i32);
			ans.push(stp);
		}
		(stp, cnt) = (0, 0);

		for (c, a) in boxes.bytes().zip(&mut ans).rev() {
            (stp, cnt) = (stp + cnt, cnt + (c == b'1') as i32);
			*a += stp;
		}
		ans
	}
}