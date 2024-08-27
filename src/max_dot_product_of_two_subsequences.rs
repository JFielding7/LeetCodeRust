use std::cmp::max;

pub struct Solution {}

impl Solution {
    pub fn max_dot_product(nums0: Vec<i32>, nums1: Vec<i32>) -> i32 {
        let length = nums1.len();
        let mut prev_max = vec![i32::MIN; length];
        let m0 = nums1[0];
        let mut max_start_dot = i32::MIN;
        for n in &nums0 {
            max_start_dot = max(max_start_dot, n * m0);
            let mut curr_max = Vec::with_capacity(length);
            curr_max.push(max_start_dot);
            for (i, m) in nums1[1..].iter().enumerate() {
                curr_max.push(curr_max[i].max(prev_max[i + 1]).max(n * m + prev_max[i].max(0)));
            }
            prev_max = curr_max;
        }
        prev_max[length - 1]
    }
}