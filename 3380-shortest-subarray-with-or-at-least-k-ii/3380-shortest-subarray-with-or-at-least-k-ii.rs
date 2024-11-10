impl Solution {
    pub fn minimum_subarray_length(nums: Vec<i32>, k: i32) -> i32 {
        if k <= 1 { return 1 }
        let (mut left, mut current_or, mut min_len) = (0, 0, i32::MAX);
        let mut bit_counts = [0; 32];

        for (right, &num) in (1..).zip(nums.iter()) {
            if num >= k { return 1 }
            current_or |= num;
            // Update the count of bits that have been set in the current_or number.
            (0..32).for_each(|bit| bit_counts[bit as usize] += (num >> bit & 1 == 1) as i32);

            while current_or >= k {
                // Update the length of min_array if possible.
                min_len = min_len.min(right - left);
                // Decrease bits_counts for bits present in left num.
                (0..32).for_each(|bit| {
                    let to_remove = nums[left as usize];
                    let decrement = to_remove >> bit & 1 == 1;
                    // If any decrement to zero, remove that bit from the current_or.
                    if decrement && bit_counts[bit as usize] == 1 {
                        current_or ^= 1 << bit
                    }
                    bit_counts[bit as usize] -= decrement as i32;
                });

                left += 1;
            }
        }

        match min_len {
            i32::MAX => -1,
            _ => min_len
        }
    }
}