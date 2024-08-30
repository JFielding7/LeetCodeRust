use std::cmp::{max, min};

pub struct Solution {}

impl Solution {
    pub fn stone_game_viii(stones: Vec<i32>) -> i32 {
        let mut prev0_min = 0;
        let mut prev1_min = 0;
        let mut curr_min = i32::MAX;
        let mut sum = 0;
        let mut score = 0;
        let mut scores = vec![];
        for stone in stones.iter().rev() {
            sum += stone;
            score = max(sum, -curr_min);
            scores.push(score);
            println!("{score}");
            // println!("{curr_min} {prev1_min} {prev0_min}");
            prev0_min = prev1_min;
            prev1_min = curr_min;
            curr_min = min(curr_min, score);
        }
        println!("{:?}", scores.iter().rev().collect::<Vec<_>>());
        max(sum, -prev0_min)
    }
}