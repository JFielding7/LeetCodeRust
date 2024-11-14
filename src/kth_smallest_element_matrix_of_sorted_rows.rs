use std::collections::{BinaryHeap, HashSet};

pub struct Solution {}

impl Solution {
    pub fn kth_smallest(mat: Vec<Vec<i32>>, k: i32) -> i32 {
        let rows = mat.len();
        let cols = mat[0].len();

        let mut curr_sum = 0;
        mat.iter().for_each(|v| curr_sum += v[0]);
        
        let mut heap = BinaryHeap::new();
        heap.push((-curr_sum, vec![1; rows]));
        
        let mut visited: HashSet<Vec<usize>> = HashSet::new();
        
        for _ in 0..k {
            let (sum, indices) = heap.pop().unwrap();
            for ((r, row), &index) in (0..rows).zip(&mat).zip(&indices) {
                if index < cols {
                    let mut next_indices = indices.clone();
                    next_indices[r] += 1;
                    if !visited.contains(&next_indices) {
                        heap.push((sum - row[index] + row[index - 1], next_indices.clone()));
                        visited.insert(next_indices);
                    }
                }
            }
            curr_sum = sum;
        }
        -curr_sum
    }
}