impl Solution 
{
    pub fn robot_with_string(s: String) -> String 
    {
        let mut paper: Vec<u8> = Vec::with_capacity(s.len());
        let mut stack: Vec<u8> = Vec::with_capacity(s.len());
        let mut bytes = s.as_bytes();
        
        // [1] determine the position of last occurence for each letter;
        //     for absent letters, the value is usize::MAX
        let mut last_pos: [usize;26] = [usize::MAX;26];
        let mut letters: u32 = 0b00000000_00000000_00000000_00000000;
        for (pos, ch) in s.bytes().enumerate().rev()
        {
            let ch = (ch - b'a') as usize;
            if last_pos[ch] == usize::MAX 
            {
                last_pos[ch] = pos;
                letters |= (1<<ch);
            }
            if letters == 0b00000011_11111111_11111111_11111111 { break; }
        }

        // [2] to obtain the lexicographically smallest string,
        //     we always have to print the smallest possible letter;
        //     thus, we first try to print all instances of each
        //     unique letter (see [4]) in the alphabetic order
        let mut lp: usize = 0;
        for (ch, &pos) in last_pos.iter().enumerate().filter(|&(_, pos)| *pos < usize::MAX)
        {
            let ch  = ch as u8 + b'a';
            
            // [3] if on some iteration there are letters on top
            //     of the stack that are smaller than the currently
            //     considered letter, move them to the paper
            while !stack.is_empty() && stack[stack.len()-1] <= ch
            {
                paper.push(stack.pop().unwrap());
            }

            // [4] main block where we move all instances of each 
            //     unique letter to the paper and push all encountered 
            //     characters (before the last instance) to the stack
            if lp <= pos
            {
                bytes[lp..=pos as usize]
                    .iter()
                    .for_each(|&c| if c == ch { paper.push(c); } else { stack.push(c); });
                lp = pos + 1;
            }
        }
        
        return String::from_utf8(paper).unwrap();
    }
}