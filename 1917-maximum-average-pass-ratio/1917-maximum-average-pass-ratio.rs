use std::collections::BinaryHeap;

impl Solution {
    pub fn max_average_ratio(classes: Vec<Vec<i32>>, extra_students: i32) -> f64 {
        #[derive(Eq, Clone, Copy)]
        struct Class {
            pass: u32,
            total: u32,
        }

        impl Class {
            fn inc_characteristic(self) -> f64 {
                (self.total - self.pass) as f64
                    / (self.total as u64 * (self.total + 1) as u64) as f64
            }

            fn add_one_student(&mut self) {
                self.pass += 1;
                self.total += 1;
            }

            fn ratio(self) -> f64 {
                self.pass as f64 / self.total as f64
            }
        }

        impl PartialEq for Class {
            fn eq(&self, other: &Self) -> bool {
                self.inc_characteristic() == other.inc_characteristic()
            }
        }

        impl PartialOrd for Class {
            fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
                Some(self.cmp(other))
            }
        }

        impl Ord for Class {
            fn cmp(&self, other: &Self) -> std::cmp::Ordering {
                self.inc_characteristic()
                    .total_cmp(&other.inc_characteristic())
            }
        }

        let n = classes.len();
        let mut classes: BinaryHeap<_> = classes
            .into_iter()
            .map(|class| Class {
                pass: class[0] as _,
                total: class[1] as _,
            })
            .collect();

        for _ in 0..extra_students {
            classes.peek_mut().unwrap().add_one_student();
        }

        classes.into_iter().map(Class::ratio).sum::<f64>() / n as f64
    }
}