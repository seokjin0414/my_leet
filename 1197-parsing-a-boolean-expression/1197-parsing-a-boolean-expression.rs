impl Solution {
    pub fn parse_bool_expr(expression: String) -> bool {
        Self::parse(&mut expression.bytes())
    }

    fn parse(iter: &mut std::str::Bytes) -> bool {
        match iter.next() {
            Some(b't') => true,
            Some(b'f') => false,
            Some(b'!') => {
                iter.next(); // '('
                let ans = !Self::parse(iter);
                iter.next(); // ')'
                ans
            }
            Some(op) => { // '&' or '|'
                let flag = op == b'&';
                let mut ans = flag;
                while iter.next() != Some(b')') { // '(' or ','
                    if Self::parse(iter) != flag {
                        ans = !flag;
                    }
                } // ')'
                ans
            }
            _ => unreachable!(),
        }
    }
}