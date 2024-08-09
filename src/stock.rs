pub struct Solution {}

use std::cmp::{max, min};

impl Solution {
    pub fn max_profit(prices: Vec<i32>) -> i32 {
        let length = prices.len();
        let mut profits: Box<[i32]> = vec![0; length + 1].into_boxed_slice();

        let mut curr_max_profit = 0;
        let mut min_price = prices[0];
        for i in 1..length {
            let price = prices[i];
            curr_max_profit = max(curr_max_profit, price - min_price);
            profits[i + 1] += curr_max_profit;
            min_price = min(min_price, price);
        }

        curr_max_profit = 0;
        let mut max_price = prices[length - 1];
        for j in (0..length - 1).rev() {
            let price = prices[j];
            curr_max_profit = max(curr_max_profit, max_price - price);
            profits[j] += curr_max_profit;
            max_price = max(max_price, price);
        }
        *profits.iter().max().unwrap()
    }
}