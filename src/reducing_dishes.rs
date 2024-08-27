pub struct Solution {}

impl Solution {
    pub fn max_satisfaction(mut satisfaction: Vec<i32>) -> i32 {
        satisfaction.sort_unstable();
        let mut max_amount = 0;
        let mut sum = 0;
        for num in satisfaction.iter().rev() {
            if -num < sum {
                max_amount += num + sum;
                sum += num;
            }
            else {
                break;
            }
        }
        max_amount
    }
}