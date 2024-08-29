impl Solution {
    pub fn remove_stones(stones: Vec<Vec<i32>>) -> i32 {
        let mut connected_component_count = 0;
        let mut set_representatives = std::collections::HashMap::new();

        fn find_representative(element: i32, set_representatives: &mut std::collections::HashMap<i32, i32>, connected_component_count: &mut i32) -> i32 {
            if !set_representatives.contains_key(&element) {
                set_representatives.insert(element, element);
                *connected_component_count += 1;
            }
            let rep = *set_representatives.get(&element).unwrap();
            if rep != element {
                let new_rep = find_representative(rep, set_representatives, connected_component_count);
                set_representatives.insert(element, new_rep);
                new_rep
            } else {
                element
            }
        }

        fn merge_components(element_a: i32, element_b: i32, set_representatives: &mut std::collections::HashMap<i32, i32>, connected_component_count: &mut i32) {
            let rep_a = find_representative(element_a, set_representatives, connected_component_count);
            let rep_b = find_representative(element_b, set_representatives, connected_component_count);
            if rep_a != rep_b {
                set_representatives.insert(rep_b, rep_a);
                *connected_component_count -= 1;
            }
        }

        for stone in stones.iter() {
            merge_components(stone[0] + 1, stone[1] + 10002, &mut set_representatives, &mut connected_component_count);
        }

        stones.len() as i32 - connected_component_count
    }
}