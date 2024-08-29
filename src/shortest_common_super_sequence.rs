use std::cmp::min;

pub struct Solution {}

impl Solution {
    pub fn shortest_common_supersequence(s0: String, s1: String) -> String {
        let len0 = s0.len() + 1;
        let len1 = s1.len() + 1;
        let mut optimal = vec![vec![0; len1]; len0];
        optimal[0] = (0..len1).collect();

        for (i, c0) in s0.chars().enumerate() {
            optimal[i + 1][0] = i + 1;
            for (j, c1) in s1.chars().enumerate() {
                optimal[i + 1][j + 1] = if c0 == c1 {
                    optimal[i][j] + 1
                } else {
                    min(optimal[i + 1][j], optimal[i][j + 1]) + 1
                };
            }
        }

        let string0 = s0.as_bytes();
        let string1 = s1.as_bytes();
        let mut len = optimal[len0 - 1][len1 - 1];
        let mut i = len0 - 1;
        let mut j = len1 - 1;
        let mut super_sequence = String::with_capacity(len);

        while len > 0 {
            super_sequence.push(if j > 0 && optimal[i][j - 1] + 1 == len {
                j -= 1;
                string1[j]
            } else if i > 0 && optimal[i - 1][j] + 1 == len {
                i -= 1;
                string0[i]
            } else {
                i -= 1;
                j -= 1;
                string0[i]
            } as char);
            len -= 1;
        }
        super_sequence.chars().rev().collect()
    }
}