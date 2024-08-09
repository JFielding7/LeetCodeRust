use std::cmp::min;

pub struct Solution {}

impl Solution {
    pub fn min_cut(s: String) -> i32 {
        let string = s.as_bytes();
        let length = string.len();
        let mut partitions = vec![i32::MAX; length].into_boxed_slice();
        for i in 0..((length << 1) - 1) {
            let mut left = i >> 1;
            let mut right = left + (i & 1);
            while right < length && string[left] == string[right] {
                if left == 0 {
                    partitions[right] = 0;
                    break;
                }
                left -= 1;
                partitions[right] = min(partitions[right], partitions[left] + 1);
                right += 1;
            }
        }
        *partitions.last().unwrap()
    }
}