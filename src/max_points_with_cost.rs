use std::cmp::max;

pub struct Solution {}

impl Solution {
    pub fn max_points(points: Vec<Vec<i32>>) -> i64 {
        let cols = points[0].len();
        let mut prev_optimal = vec![0; cols];
        for row in points {
            let mut optimal = Vec::with_capacity(cols);
            let mut max_points = 0;
            for (i, score) in row.iter().enumerate() {
                max_points = max(max_points - 1, (*score as i64) + prev_optimal[i]);
                optimal.push(max_points);
            }
            for i in (0..(cols - 1)).rev() {
                optimal[i] = max(optimal[i], optimal[i + 1] - 1);
            }
            prev_optimal = optimal;
        }
        *prev_optimal.iter().max().unwrap()
    }
}