use std::cmp::min;

pub struct Solution {}

impl Solution {
    pub fn min_cost(n: i32, mut cuts: Vec<i32>) -> i32 {
        cuts.push(0);
        cuts.push(n);
        cuts.sort();
        let len = cuts.len();

        let mut optimal = Vec::with_capacity(len - 1);
        optimal.push(vec![0; len - 1]);
        for offset in 2..len {
            optimal.push(Vec::with_capacity(len - offset));
            for start in 0..len - offset {
                let end = start + offset;
                let curr_cost = cuts[end] - cuts[start];
                let mut min_cost = i32::MAX;
                for cut in start + 1..end {
                    min_cost = min(min_cost, optimal[cut - start - 1][start] + optimal[end - cut - 1][cut]);
                }
                optimal[offset - 1].push(curr_cost + min_cost);
            }
        }
        optimal[len - 2][0]
    }
}