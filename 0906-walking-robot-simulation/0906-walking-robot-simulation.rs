use std::iter::FromIterator;
use std::collections::HashSet;
use std::ops::RangeInclusive;
use std::cmp;

#[derive(Copy, Clone)]
pub enum Direction {
    North, East, South, West
}

pub struct DirectionIterator {
    direction: Direction,
    x: i32,
    y: i32,
    step: i32,
    steps_taken: i32,
}

impl Iterator for DirectionIterator {
    type Item = (i32, i32);

    fn next(&mut self) -> Option<Self::Item> {
        if self.steps_taken >= self.step {
            return None;
        }

        match self.direction {
            Direction::North => self.y += 1,
            Direction::East => self.x += 1,
            Direction::South => self.y -= 1,
            Direction::West => self.x -= 1,
        }

        self.steps_taken += 1;
        Some((self.x, self.y))
    }
}


impl Direction {
    pub fn iterate(&self, x_start: i32, y_start: i32, k: i32) -> DirectionIterator {
        DirectionIterator {
            direction: *self,
            x: x_start,
            y: y_start,
            step: k,
            steps_taken: 0,
        }
    }

    pub fn rotate_left(&self) -> Direction {
        // println!("left");
        match self {
            Direction::North => Direction::West,
            Direction::East => Direction::North,
            Direction::South => Direction::East,
            Direction::West => Direction::South,
        }
    }

    pub fn rotate_right(&self) -> Direction {
        // println!("right");
        match self {
            Direction::North => Direction::East,
            Direction::East => Direction::South,
            Direction::South => Direction::West,
            Direction::West => Direction::North,
        }
    }
}

impl Solution {
    pub fn robot_sim(commands: Vec<i32>, obstacles: Vec<Vec<i32>>) -> i32 {
        let obstacles: HashSet<(i32, i32)> = obstacles
            .into_iter()
            .filter_map(|inner_vec| {
                if inner_vec.len() >= 2 {
                    Some((inner_vec[0], inner_vec[1]))
                } else {
                    None
                }
            })
            .collect();

        let mut pos: (i32, i32) = (0, 0);
        let mut dir = Direction::North;
        let mut last: Option<(i32, i32)> = None;
        let mut dist: i32 = 0;

        for c in commands {
            match c {
                -2 => { dir = dir.rotate_left(); },
                -1 => { dir = dir.rotate_right(); },
                1..=9 => {
                    // get final pos
                    last = None;
                    for next_pos in dir.iterate(pos.0, pos.1, c) {
                        if obstacles.contains(&next_pos) { break; }
                        last = Some(next_pos);
                    }

                    if let Some(x) = last {
                        pos = x;
                    }

                    dist = cmp::max(
                        dist,
                        pos.0.pow(2) + pos.1.pow(2)
                    );

                    // println!("{} => moved to {:?}, dist {}", c, &pos, dist);
                }
                _ => {},
            }
        }

        dist
    }
}