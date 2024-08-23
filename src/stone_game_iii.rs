use std::cmp::{max, min};
use std::collections::VecDeque;

pub struct Solution {}

impl Solution {
    pub fn stone_game_iii(stone_value: Vec<i32>) -> String {
        let len = stone_value.len();
        let mut minimax_values = VecDeque::from([0, 0, 0]);
        for i in (0..len).rev() {
            let mut minimax = i32::MIN;
            let mut sum = 0;
            for j in i..min(i + 3, len) {
                sum += stone_value[j];
                minimax = max(minimax, sum - minimax_values[j - i]);
            }
            minimax_values.pop_back();
            minimax_values.push_front(minimax);
        }
        let minimax = minimax_values[0];
        if minimax > 0 { "Alice" } else if minimax < 0 { "Bob" } else { "Tie" }.to_string()
    }
}