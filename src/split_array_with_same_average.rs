use std::collections::HashMap;

pub struct Solution {}

impl Solution {
    fn sum_exists(nums: &Vec<i32>, n: i32, length: i32, sum: i32, cache: &mut HashMap<(i32, i32, i32), bool>) -> bool {
        if length == 0 {
            return sum == 0;
        }
        if n == 0 {
            return false;
        }
        if let Some(e) = cache.get(&(n, length, sum)) {
            return *e;
        }

        let exists = Self::sum_exists(nums, n - 1, length, sum, cache)
            || Self::sum_exists(nums, n - 1, length - 1, sum - nums[n as usize - 1], cache);
        cache.insert((n, length, sum), exists);
        exists
    }

    pub fn split_array_same_average(nums: Vec<i32>) -> bool {
        let len = nums.len() as i32;
        let sum = nums.iter().sum::<i32>();
        let mut cache = HashMap::new();

        for curr_len in 1..=(len >> 1) {
            if sum * curr_len % len == 0 {
                for n in curr_len..=len {
                   if Self::sum_exists(&nums, n, curr_len, sum * curr_len / len, &mut cache) {
                       return true;
                   }
                }
            }
        }

        false
    }
}
