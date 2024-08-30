pub struct Solution {}

impl Solution {
    pub fn min_number_operations(target: Vec<i32>) -> i32 {
        let mut prev = 0;
        let mut count = 0;
        for num in target {
            count += (num > prev) as i32 * (num - prev);
            prev = num;
        }
        count
    }
}