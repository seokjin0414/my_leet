impl Solution {
    pub fn check_if_prerequisite(num_courses: i32, prerequisites: Vec<Vec<i32>>, queries: Vec<Vec<i32>>) -> Vec<bool> {
        use std::collections::VecDeque;

        let (mut graph, mut in_degree) = prerequisites
          .into_iter()
          .fold(([0_u128; 100], [0_i32; 100]), |(mut graph, mut in_degree), prereq| {
            let (a, b) = (prereq[0], prereq[1]);  // b -> a
            in_degree[a as usize] += 1;
            graph[b as usize] |= 1 << a;
            (graph, in_degree)
          });

        let mut zero_in_degree = in_degree
          .iter()
          .copied()
          .enumerate()
          .filter(|&(_, d)| d == 0)
          .map(|(i, _)| i as i32)
          .collect::<VecDeque<_>>();

        let mut is_prereq = [0_u128; 100];
        std::iter::successors(zero_in_degree.pop_front(), |&n| {
            let adjs = graph[n as usize];  // n -> adjs
            (0..num_courses)
              .filter(|adj| (1 << adj) & adjs != 0)
              .for_each(|adj| {
                is_prereq[adj as usize] |= 1 << n;
                is_prereq[adj as usize] |= is_prereq[n as usize];
                in_degree[adj as usize] -= 1;
                if in_degree[adj as usize] == 0 {
                    zero_in_degree.push_back(adj);
                }
              });
            zero_in_degree.pop_front()
        })
        .last();

        queries
          .into_iter()
          .map(|q| {
            let (u, v) = (q[0], q[1]);  // u -> v?
            is_prereq[u as usize] & (1 << v) != 0
          })
          .collect()
    }
}