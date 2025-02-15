impl Solution {
    pub fn punishment_number(n: i32) -> i32 {
        use std::fmt::{self, Write};

        struct StackBuffer<const N: usize> {
            buf: [u8; N],
            len: usize,
        }

        impl<const N: usize> StackBuffer<N> {
            fn new() -> Self {
                Self {
                    buf: [0; N],
                    len: 0,
                }
            }

            fn as_bytes(&self) -> &[u8] {
                &self.buf[..self.len]
            }
        }

        impl<const N: usize> Write for StackBuffer<N> {
            fn write_str(&mut self, s: &str) -> fmt::Result {
                let new_len = self.len + s.len();
                self.buf
                    .get_mut(self.len..new_len)
                    .ok_or(fmt::Error)?
                    .copy_from_slice(s.as_bytes());
                self.len = new_len;

                Ok(())
            }
        }

        fn backtrack(buf: &[u8], target: i32, curr_sum: i32) -> bool {
            if curr_sum > target {
                return false;
            }

            if buf.is_empty() {
                return target == curr_sum;
            }

            buf.iter()
                .copied()
                .enumerate()
                .scan(0, |num, (i, b)| {
                    *num = *num * 10 + (b - b'0') as i32;
                    Some((i, *num))
                })
                .any(|(i, num)| backtrack(&buf[i + 1..], target, curr_sum + num))
        }

        (1..=n)
            .filter(|&i| {
                let mut buf = StackBuffer::<7>::new();
                buf.write_fmt(format_args!("{}", i * i)).unwrap();
                backtrack(buf.as_bytes(), i, 0)
            })
            .map(|i| i * i)
            .sum()
    }
}