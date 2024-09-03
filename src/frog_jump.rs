use std::collections::{HashMap, HashSet};

pub struct Solution {}

impl Solution {
    fn solve(pos: i32, k: i32, end: i32, cache: &mut HashMap<i32, HashSet<i32>>) -> bool {
        if pos == end {
            return true;
        }
        if cache.get(&pos).unwrap().contains(&k) {
            return false;
        }
        for j in k - 1..k + 2 {
            if j > 0 && cache.contains_key(&(pos + j)) && Self::solve(pos + j, j, end, cache) {
                return true;
            }
        }
        cache.get_mut(&pos).unwrap().insert(k);
        false
    }

    pub fn can_cross(stones: Vec<i32>) -> bool {
        if stones[1] != 1 {
            return false;
        }
        let mut cache: HashMap<i32, HashSet<i32>> = stones.iter().map(|&stone| (stone, HashSet::new())).collect();
        Self::solve(1, 1, *stones.last().unwrap(), &mut cache)
    }
}