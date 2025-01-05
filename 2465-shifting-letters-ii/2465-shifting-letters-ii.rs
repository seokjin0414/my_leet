impl Solution {
    pub fn shifting_letters(s: String, shifts: Vec<Vec<i32>>) -> String {
        let mut net_shifts = vec![0; s.len()];
        for v in shifts {
            let (start, end, direction) = (v[0] as usize, v[1] as usize, v[2]);
            let value = if direction == 1 { 1 } else { -1 };

            net_shifts[start] += value;
            if end + 1 < s.len() {
                net_shifts[end + 1] -= value;
            }
        }

        let mut cummulative_shift = 0;
        s.bytes() .enumerate() 
            .map(|(i, byte)| {
                cummulative_shift = cummulative_shift + net_shifts[i] % 26;
                cummulative_shift = (cummulative_shift + 26) % 26;
                let char_index = ((byte - b'a') as i32 + cummulative_shift) % 26; 
                (b'a' + char_index as u8) as char
            })
            .collect::<String>()
    }
}