use std::collections::{VecDeque, HashSet};

#[derive(Debug, Copy,Clone,PartialEq, Eq, Hash)]
struct Location(usize,usize);

fn find_person(map : &Vec<Vec<char>>) -> Location {
    for (i,&ref arr) in map.iter().enumerate() {
        for (j, &c) in arr.iter().enumerate() {
            if c == '*' {
                return Location(i,j);
            }
        }
    }
    panic!("No * in input map!!!");
}

fn find_food(map : &Vec<Vec<char>>) -> HashSet<Location> {
    let mut solution : HashSet<Location> = HashSet::new();
    for (i,&ref arr) in map.iter().enumerate() {
        for (j, &c) in arr.iter().enumerate() {
            if c == '#' {
                solution.insert(Location(i,j));
            }
        }
    }
    return solution;
}

impl Solution {
    pub fn get_food(grid: Vec<Vec<char>>) -> i32 {
        let you : Location = find_person(&grid);
        let food: HashSet<Location> = find_food(&grid);
        let mut queue: VecDeque<(Location, i32)> = VecDeque::new();
        let mut in_queue : HashSet<Location> = HashSet::new();
        let mut visited : HashSet<Location> = HashSet::new();

        queue.push_back((you, 0));
        in_queue.insert(you);
        visited.insert(you);

        let m = grid.len() - 1;
        let n = grid[0].len() - 1;

        while !queue.is_empty() {
            let (current, dist_travelled) = queue.pop_front().unwrap();
            match grid[current.0][current.1] {
                '#' => return dist_travelled,
                'X' => continue, 
                'O' | '*' => {
                    visited.insert(current);
                    let l : Location = Location(current.0,current.1.saturating_sub(1));
                    let r : Location = Location(current.0,current.1 + 1);
                    let u : Location = Location(current.0.saturating_sub(1),current.1);
                    let d : Location = Location(current.0 + 1, current.1);
                  
                    if current.1 > 0 && !visited.contains(&l) && !in_queue.contains(&l) { 
                        queue.push_back((l, dist_travelled + 1));
                        in_queue.insert(l);
                    }
                    if current.1 < n && !visited.contains(&r) && !in_queue.contains(&r) { 
                        queue.push_back((r, dist_travelled + 1));
                        in_queue.insert(r);    
                    }
                    if current.0 > 0 && !visited.contains(&u) && !in_queue.contains(&u) {
                        queue.push_back((u, dist_travelled + 1)); 
                        in_queue.insert(u);
                    }
                    if current.0 < m && !visited.contains(&d) && !in_queue.contains(&d) {
                        queue.push_back((d, dist_travelled + 1)); 
                        in_queue.insert(d);    
                    }
                },
                _ => panic!("Should never hit this condition"),
            }
        }

        return -1;
    }
}