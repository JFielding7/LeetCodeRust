pub struct Solution {}

impl Solution {
    pub fn find_min(nums: Vec<i32>) -> i32 {
        let num0 = nums[0];
        let max_index = nums.len() - 1;
        if max_index == 0 || num0 < nums[max_index] {
            return num0;
        }
        let mut start = 1;
        let mut end = max_index;
        while start <= end {
            let mid = (start + end) >> 1;
            if nums[mid] > num0 {
                start = mid + 1;
            }
            else {
                if mid == 0 { break; }
                end = mid - 1;
            }
        }
        nums[start]
    }
}