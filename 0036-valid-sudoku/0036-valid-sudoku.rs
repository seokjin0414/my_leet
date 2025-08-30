use std::collections::HashSet;

impl Solution {
    pub fn is_valid_sudoku(board: Vec<Vec<char>>) -> bool {
        let mut rows = vec![HashSet::new();9];
        let mut cols = vec![HashSet::new();9];
        let mut boxes = vec![HashSet::new();9];;

        for row in 0..9 {
            for col in 0..9 {
                let ch = board[row][col];

                if ch == '.' {
                    continue;
                }

                let box_idx = (row/3)*3 + col/3;

                if rows[row].contains(&ch) || cols[col].contains(&ch) || boxes[box_idx].contains(&ch) {
                    return false;
                }

                rows[row].insert(ch);
                cols[col].insert(ch);
                boxes[box_idx].insert(ch);
            }
        }
        true
    }
}