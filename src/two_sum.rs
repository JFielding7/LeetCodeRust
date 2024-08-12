use std::collections::HashMap;

pub struct Solution {}

impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let mut seen = HashMap::new();
        for (i, num) in nums.iter().enumerate() {
            let opt = seen.get(&(target - num));
            if opt.is_some() { return vec![*opt.unwrap() as i32, i as i32]; }
            seen.insert(num, i);
        }
        vec![]
    }
}