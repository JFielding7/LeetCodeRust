use std::cmp::max;

pub struct Solution {}

impl Solution {
    fn get_byte(i: usize, bytes0: &[u8], len0: usize, bytes1: &[u8]) -> u8 {
        if i < len0 {
            bytes0[i]
        } else {
            bytes1[i - len0]
        }
    }

    pub fn longest_palindrome(word0: String, word1: String) -> i32 {
        let len0 = word0.len();
        let bytes0 = word0.as_bytes();

        let len1 = word1.len();
        let bytes1 = word1.as_bytes();

        let mut prev_lengths = vec![1, 0];
        let mut max_len = 0;

        for j in 1..len0 + len1 {
            let end = Self::get_byte(j, bytes0, len0, bytes1);
            let mut curr_lengths = vec![0; j + 2];
            curr_lengths[j] = 1;

            for i in (0..j).rev() {
                let curr_len = if end == Self::get_byte(i, bytes0, len0, bytes1) {
                    let len = prev_lengths[i + 1] + 2;
                    if i < len0 && j >= len0 && len > max_len {
                        max_len = len;
                    }
                    len
                } else {
                    max(prev_lengths[i], curr_lengths[i + 1])
                };
                curr_lengths[i] = curr_len;
            }
            prev_lengths = curr_lengths;
        }
        max_len
    }
}