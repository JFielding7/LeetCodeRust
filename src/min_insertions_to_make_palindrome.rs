use std::cmp::min;

pub struct Solution {}

impl Solution {
    pub fn min_insertions(s: String) -> i32 {
        let string = s.as_bytes();
        let mut prev = vec![0, 0];
        for end in 1..string.len() {
            let mut curr = vec![0; end + 2];
            for start in (0..end).rev() {
                curr[start] = if string[start] == string[end] {
                    prev[start + 1]
                } else {
                    min(curr[start + 1], prev[start]) + 1
                }
            }
            prev = curr;
        }
        prev[0]
    }
}