use std::cmp::min;

pub struct Solution {}

impl Solution {
    fn next_subsequence(subsequence: usize) -> usize {
        let mut p = subsequence;
        let mut i = 0;
        let mut ones = 0;
        loop {
            if (p & 0b1) == 0b1 {
                if (p & 0b11) == 0b1 {
                    return ((subsequence + (1 << i)) >> i << i) + ((1 << ones) - 1);
                }
                ones += 1;
            }
            p >>= 1;
            i += 1;
        }
    }

    pub fn minimum_xor_sum(v0: Vec<i32>, v1: Vec<i32>) -> i32 {
        let len = v0.len();
        let max_subsequence = (1 << len) - 1;
        let mut optimal = vec![0; 1 << len];

        for curr_len in 1..=len {
            let mut subsequence = (1 << curr_len) - 1;
            while subsequence <= max_subsequence {
                let mut curr_min = i32::MAX;
                for i in 0..len {
                    let bit = 1 << i;
                    if (subsequence & bit) != 0 {
                        curr_min = min(curr_min, (v0[curr_len - 1] ^ v1[i]) + optimal[subsequence ^ bit]);
                    }
                }

                optimal[subsequence] = curr_min;
                subsequence = Self::next_subsequence(subsequence);
            }
        }

        optimal[max_subsequence]
    }
}
