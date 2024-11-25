use std::collections::{HashSet, VecDeque};

impl Solution {
    fn find_shortest_path(start: Vec<u8>, target: Vec<u8>) -> i32{
        if start == target {
            return 0;
        }
        
        let moves = vec![
            vec![1, 3],      
            vec![0, 2, 4],   
            vec![1, 5],      
            vec![0, 4],      
            vec![1, 3, 5],   
            vec![2, 4],
        ];

        let mut queue: VecDeque<Vec<u8>> = VecDeque::new();
        queue.push_back(start.to_vec());

        let mut visited:HashSet<Vec<u8>> = HashSet::new();
        visited.insert(start);
        let mut n_moves = 0;
        while !queue.is_empty() {
            let size = queue.len();
            for _ in 0..size {
                if let Some(current) = queue.pop_front() {
                    if current == target {
                        return n_moves;
                    }
                    let zero_pos = current.iter().position(|&x| x == 0).unwrap();
                    for &next_pos in &moves[zero_pos] {
                        let mut new_state = current.clone();
                        new_state.swap(zero_pos, next_pos);
                        
        
                        if visited.insert(new_state.clone()) {
                            queue.push_back(new_state);
                        }
                    }
                    
                }
            }
            n_moves += 1;
        }
        -1
    }
    pub fn sliding_puzzle(board: Vec<Vec<i32>>) -> i32 {
        let mut target:Vec<u8> = vec![1,2,3,4,5,0];
        let mut start:Vec<u8> = Vec::new();
        for row in board {
            for val in row {
                start.push(val as u8);
            }
        }
        Self::find_shortest_path(start, target)
    }
}