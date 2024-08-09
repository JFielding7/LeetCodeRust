use std::cmp::min;
use std::collections::HashMap;

pub struct Solution {}

static mut calls: i32 = 0;

impl Solution {
    fn min_largest_sum(i: usize, k: usize, nums: &Vec<i32>, prefix_sums: &Vec<i32>, cache: &mut HashMap<(usize, usize), i32>) -> i32 {
        unsafe { calls += 1; }
        if k == 1 {
            return prefix_sums[i];
        }
        if let Some(&min) = cache.get(&(i, k)) {
            // println!("hello");
            return min;
        }
        let mut min_largest = i32::MAX;
        let mut start = 0;
        let mut end = i;
        while start <= end {
            let mid = start + end >> 1;
            if mid + 3 > k {
                let front_min_largest = Self::min_largest_sum(mid, k - 1, nums, prefix_sums, cache);
                let back = prefix_sums[i] - prefix_sums[mid];
                if front_min_largest > back {
                    min_largest = min(min_largest, front_min_largest);
                    if mid == 0 { break; }
                    end = mid - 1;
                } else {
                    min_largest = min(min_largest, back);
                    start = mid + 1;
                }
            } else {
                start = mid + 1;
            }
        }
        cache.insert((i, k), min_largest);
        min_largest
    }

    pub fn split_array(nums: Vec<i32>, k: i32) -> i32 {
        let prefix_sums = nums.iter().scan(0, |sum, num| {
            *sum += num;
            Some(*sum)
        }).collect();
        let mut cache = HashMap::new();
        let res = Self::min_largest_sum(nums.len() - 1, k as usize, &nums, &prefix_sums, &mut cache);
        unsafe { println!("{}", calls); }
        println!("{}", cache.len());
        res
    }
}