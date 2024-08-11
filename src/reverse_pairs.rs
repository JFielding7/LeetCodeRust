pub struct Solution {}

impl Solution {
    fn binary_search(nums: &Vec<i32>, num: i32, mut end: usize) -> usize {
        let mut start = 0;
        while start <= end {
            let mid = start + end >> 1;
            if num > nums[mid] {
                start = mid + 1;
            }
            else {
                if mid == 0 { return start; }
                end = mid - 1;
            }
        }
        start
    }

    pub fn reverse_pairs(nums: Vec<i32>) -> i32 {
        let mut sorted_nums: Vec<i32> = nums.clone();
        sorted_nums.sort();
        let offset = nums.len() - 1;
        let mut tree = vec![0; offset * 2 + 1];

        let mut count = 0;
        for &num in nums.iter().rev() {
            let mut left = offset;
            let mut right = offset + Self::binary_search(&sorted_nums, (num >> 1) + (num & 1), offset);
            while left != right {
                if (left & 1) == 0 {
                    count += tree[left];
                    left += 1;
                }
                if (right & 1) == 0 {
                    right -= 1;
                    count += tree[right];
                }
                left >>= 1;
                right >>= 1;
            }

            let mut i = offset + Self::binary_search(&sorted_nums, num, offset);
            while i > 0 {
                tree[i] += 1;
                i = (i - 1) >> 1;
            }
            tree[0] += 1;
        }
        count
    }
}