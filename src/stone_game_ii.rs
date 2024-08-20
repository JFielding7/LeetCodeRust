use std::cmp::{max, min};

pub struct Solution {}

impl Solution {
    pub fn stone_game_ii(piles: Vec<i32>) -> i32 {
        let length = piles.len();
        let rows = length + 1;
        let mut cache = vec![0; rows * rows];
        for i in (0..length).rev() {
            for m in 1..(length + 1) {
                let mut curr_max = i32::MIN;
                let mut curr_stones = 0;
                for j in i..min(i + (m << 1), length) {
                    curr_stones += piles[j];
                    curr_max = max(curr_max, curr_stones - cache[(j + 1) * rows + max(j - i + 1, m)]);
                }
                cache[i * rows + m] = curr_max;
            }
        }
        (piles.iter().sum::<i32>() + cache[1]) >> 1
    }
}