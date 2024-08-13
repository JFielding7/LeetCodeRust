pub struct Solution {}

impl Solution {
    pub fn candy(ratings: Vec<i32>) -> i32 {
        let length = ratings.len();
        if length == 1 { return 1; }
        let mut count = 0;
        let mut start = 0;
        let mut prev = ratings[1];
        let mut prev_diff = ratings[1] - ratings[0];
        for i in 2..length {
            let rating = ratings[i];
            let curr_diff = rating - prev;
            if curr_diff == 0 || (curr_diff > 0) != (prev_diff > 0) {
                let is_local_min = (curr_diff > 0 && prev_diff < 0) as usize;
                count += ((i - start) * (i - start + 1) >> 1) - is_local_min;
                start = i - is_local_min;
                println!("{count} {start}");
            }
            prev = rating;
            prev_diff = curr_diff;
        }
        (count + ((length - start) * (length - start + 1) >> 1)) as i32
    }
}