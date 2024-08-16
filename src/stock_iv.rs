use std::cmp::max;

pub struct Solution {}

impl Solution {
    pub fn max_profit(k: i32, prices: Vec<i32>) -> i32 {
        let n = prices.len() + 1;
        let mut prev_maxes = vec![0; n];
        for _ in 0..k {
            let mut maxes = vec![0; n];
            let mut curr_max = -i32::MAX;
            for (i, price) in prices.iter().enumerate() {
                maxes[i + 1] = max(maxes[i], price + curr_max);
                curr_max = max(curr_max, prev_maxes[i] - price);
            }
            prev_maxes = maxes;
        }
        *prev_maxes.last().unwrap()
    }
}