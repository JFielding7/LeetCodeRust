use std::collections::HashMap;

pub struct Solution {}

impl Solution {
    pub fn number_of_arithmetic_slices(nums: Vec<i32>) -> i32 {
        let mut count = 0;
        let len = nums.len();
        let mut cache: Vec<HashMap<i64, i32>> = Vec::with_capacity(len);
        for &num0 in nums.iter() {
            let n0 = num0 as i64;
            let mut curr_counts = HashMap::new();
            for (&num1, counts) in nums.iter().zip(&cache) {
                let n1 = num1 as i64;
                let curr_count = curr_counts.entry(n0 - n1).or_insert(0);
                if let Some(c) = counts.get(&(n0 - n1)) {
                    *curr_count += c + 1;
                    count += c + 1;
                } else {
                    *curr_count += 1;
                    count += 1;
                }
            }
            cache.push(curr_counts);
        }
        count - (len * (len - 1) >> 1) as i32
    }
}