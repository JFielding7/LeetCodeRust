pub struct Solution;

impl Solution {
    pub fn xor_game(nums: Vec<i32>) -> bool {
        if (nums.len() & 1) == 0 {
            return true;
        }

        nums.into_iter().reduce(|res, num| res ^ num).unwrap() == 0
    }
}