impl Solution {
    pub fn rotate_the_box(mut bx: Vec<Vec<char>>) -> Vec<Vec<char>> {
        let mut result = vec![vec!['.';bx.len()];bx[0].len()];
        for row in 0..bx.len() {
            let mut bottom = bx[row].len()-1;
            for col in (0..bx[row].len()).rev() {
                if bx[row][col] == '*' { bottom = col-1; }
                if bx[row][col] == '#' {
                    if bottom != col {
                        (bx[row][bottom],bx[row][col]) = ('#','.');
                        result[bottom][bx.len()-row-1] = '#';
                        bottom -= 1;
                    } else {
                        bottom = col-1;
                    }
                }
                result[col][bx.len()-row-1] = bx[row][col];                                
            }
        }
        result
    }
}
