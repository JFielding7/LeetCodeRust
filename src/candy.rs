use std::cmp::min;

pub struct Solution {}

macro_rules! update_count {
    ($curr:expr, $prev:expr) => { ($curr * ($curr + 1) >> 1) - min($curr, $prev) };
}

impl Solution {
    pub fn candy(ratings: Vec<i32>) -> i32 {
        let length = ratings.len();
        if length == 1 { return 1; }

        let mut count = 0;
        let mut start = 0;
        let mut prev_len = 0;
        let mut prev_diff = 0;
        let mut prev = ratings[0];

        for i in 1..length {
            let rating = ratings[i];
            let diff = rating - prev;
            if diff == 0 {
                count += update_count!(i - start, prev_len);
                prev_len = 0;
                start = i;
            }
            else if prev_diff != 0 && (diff > 0) != (prev_diff > 0) {
                count += update_count!(i - start, prev_len) - ((prev_diff < 0) as usize);
                prev_len = if prev_diff > 0 { i - start } else { 0 };
                start = i - 1;
            }
            prev_diff = diff;
            prev = rating;
        }
        (count + update_count!(length - start, prev_len)) as i32
    }
}