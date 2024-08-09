pub struct Solution {}

impl Solution {
    pub fn find_min(nums: Vec<i32>) -> i32 {
        let mut start = 0;
        let mut end = nums.len() - 1;
        while start < end && nums[start] == nums[end] {
            start += 1;
            end -= 1;
        }
        let num0 = nums[0];
        let first = nums[start];
        let last = nums[end];
        if num0 < first && num0 < last {
            return num0;
        }
        if start >= end || first < last {
            return first;
        }
        while start <= end {
            let mid = (start + end) >> 1;
            if nums[mid] >= first {
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