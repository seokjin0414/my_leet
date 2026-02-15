impl Solution {
    pub fn add_binary(a: String, b: String) -> String {
        let x: Vec<char> = a.chars().collect();
        let y: Vec<char> = b.chars().collect();

        let (n_sm, n_lg, sm, lg) = if x.len() < y.len() {
            (x.len(), y.len(), x, y)
        } else {
            (y.len(), x.len(), y, x)
        };

        let off = n_lg - n_sm;

        let mut b = vec![' '; n_lg];
        let mut carry = 0;

        for i in (0..n_sm).rev() {
            let k = i + off;
            let dsm = sm[i];
            let dlg = lg[k];

            let d = (dsm as u8 - b'0') + (dlg as u8 - b'0') + carry;
            if d == 0 {
                b[k] = '0';
                carry = 0;
            } else if d == 1 {
                b[k] = '1';
                carry = 0;
            } else if d == 2 {
                b[k] = '0';
                carry = 1;
            } else if d == 3 {
                b[k] = '1';
                carry = 1;
            }
        }

        for i in (0..off).rev() {
            let v = lg[i];
            let d = (v as u8 - b'0') + carry;
            if d == 0 {
                b[i] = '0';
                carry = 0;
            } else if d == 1 {
                b[i] = '1';
                carry = 0;
            } else if d == 2 {
                b[i] = '0';
                carry = 1;
            } else if d == 3 {
                b[i] = '1';
                carry = 1;
            }
        }

        if carry == 1 {
            b.insert(0, '1');
        }
        let result: String = b.into_iter().collect();

        result
    }
}