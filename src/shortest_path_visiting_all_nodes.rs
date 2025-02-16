use std::collections::VecDeque;

pub struct Solution {}

impl Solution {
    fn update_lengths(graph: &Vec<Vec<i32>>, start: i32, start_dist: i32, lengths: &mut Vec<i32>) {
        let mut visited = vec![false; graph.len()];
        let mut queue = VecDeque::new();

        queue.push_back((start, start_dist));
        visited[start as usize] = true;
        lengths[start as usize] = start_dist;

        while let Some((node, dis)) = queue.pop_front() {
            for &neighbor in &graph[node as usize] {
                let neigh_index = neighbor as usize;
                if !visited[neigh_index] {
                    queue.push_back((neighbor, dis + 1));
                    visited[neigh_index] = true;
                    lengths[neigh_index] = lengths[neigh_index].min(dis + 1);
                }
            }
        }
    }

    fn shortest_paths(graph: &Vec<Vec<i32>>, nodes: usize, cache: &mut Vec<Option<Vec<i32>>>) {
        if cache[nodes] != None {
            return;
        }

        let mut lengths = vec![i32::MAX; graph.len()];

        for n in 0..graph.len() {
            if ((nodes >> n) & 1) == 1 {
                Self::shortest_paths(graph, nodes ^ (1 << n), cache);
                let start_dist = cache[nodes ^ (1 << n)].as_ref().unwrap()[n];
                Self::update_lengths(&graph, n as i32, start_dist, &mut lengths);
            }
        }

        cache[nodes] = Option::from(lengths);
    }

    pub fn shortest_path_length(graph: Vec<Vec<i32>>) -> i32 {
        let node_count = graph.len();
        let mut cache = vec![None; 1 << node_count];
        cache[0] = Option::from(vec![0; node_count]);

        Self::shortest_paths(&graph, (1 << node_count) - 1, &mut cache);
        *cache[(1 << node_count) - 1].as_ref().unwrap().iter().min().unwrap()
    }
}
