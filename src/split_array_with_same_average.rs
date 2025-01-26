use std::collections::HashMap;

pub struct Solution {}

impl Solution {
    fn sum_exists(nums: &Vec<i32>, length: usize, sum: usize, n: usize, cache: &mut HashMap<(usize, usize, usize), bool>) -> bool {
        if length == 0 {
            return sum == 0;
        }
        if n == 0 {
            return false;
        }
        if let Some(e) = cache.get(&(length, sum, n)) {
            return *e;
        }

        let exists = Self::sum_exists(nums, length, sum, n - 1, cache)
            || Self::sum_exists(nums, length - 1, sum - nums[n - 1] as usize, n - 1, cache);
        cache.insert((length, sum, n), exists);
        exists
    }

    pub fn split_array_same_average(nums: Vec<i32>) -> bool {
        let len = nums.len();
        let sum = nums.iter().sum::<i32>() as usize;
        let mut cache = HashMap::new();

        for curr_len in 1..len {
            if sum * curr_len % len == 0 {
                for n in curr_len..=len {
                   if Self::sum_exists(&nums, curr_len, sum * curr_len / len, n, &mut cache) {
                       return true;
                   }
                }
            }
        }

        false
    }
}
