pub struct Solution {}

impl Solution {
    fn binary_search(nums: &Vec<i32>, num: i32, mut end: usize) -> usize {
        let mut start = 0;
        while start <= end {
            let mid = start + end >> 1;
            let mid_num = nums[mid];
            if num == mid_num {
                return mid;
            }
            else if num > mid_num {
                start = mid + 1;
            }
            else {
                if mid == 0 { return start; }
                end = mid - 1;
            }
        }
        start
    }
    pub fn median_sliding_window(nums: Vec<i32>, k: i32) -> Vec<f64> {
        let mut sorted_nums = nums.clone();
        sorted_nums.sort();
        // let mut seen = Vec::with_capacity(nums.len());
        vec![]
    }
}