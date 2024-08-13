use std::cmp::max;

pub struct Solution {}

macro_rules! max_valid {
    ($iter:expr, $cmp:tt) => {{
        let mut longest = 0;
        let mut opened = 0;
        let mut closed = 0;
        for chr in $iter {
            if chr == '(' {
                opened += 1;
            } else {
                closed += 1;
            }
            if opened == closed {
                longest = max(longest, opened + closed);
            } else if opened $cmp closed {
                opened = 0;
                closed = 0;
            }
        }
        longest
    }};
}

impl Solution {
    pub fn longest_valid_parentheses(s: String) -> i32 {
        max(max_valid!(s.chars(), <), max_valid!(s.chars().rev(), >))
    }
}