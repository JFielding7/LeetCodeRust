pub struct Solution {}

impl Solution {
    pub fn smallest_distance_pair(mut nums: Vec<i32>, k: i32) -> i32 {
        nums.sort();
        let count = k as usize;
        let mut lower_dist = 1;
        let mut upper_dist = nums[nums.len() - 1] - nums[0];
        while lower_dist <= upper_dist {
            let dist = lower_dist + upper_dist >> 1;
            let mut pairs_below_diff = 0;
            for (i, num) in nums.iter().enumerate() {
                let target = num - dist;
                let mut start = 0;
                let mut end = i;
                while start <= end {
                    let mid = start + end >> 1;
                    if nums[mid] > target {
                        if mid == 0 { break; }
                        end = mid - 1;
                    } else {
                        start = mid + 1;
                    }
                }
                pairs_below_diff += i - start;
            }
            if pairs_below_diff < count {
                lower_dist = dist + 1;
            } else {
                upper_dist = dist - 1;
            }
        }
        upper_dist
    }
}