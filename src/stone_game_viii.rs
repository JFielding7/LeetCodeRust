use std::cmp::{max, min};

pub struct Solution {}

impl Solution {
    pub fn stone_game_viii(stones: Vec<i32>) -> i32 {
        let mut prefix_sum: i32 = stones.iter().sum();
        let mut suffix_sum = 0;
        let mut score = 0;
        let mut min_score = i32::MAX;
        for stone in stones[1..].iter().rev() {
            score = max(suffix_sum, -min_score) + prefix_sum;
            println!("{stone} {score}");
            min_score = min(min_score, score);
            prefix_sum -= stone;
            suffix_sum += stone;
        }
        score
    }
}