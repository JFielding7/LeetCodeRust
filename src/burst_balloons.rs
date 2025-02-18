pub struct Solution;

impl Solution {
    pub fn max_coins(nums: Vec<i32>) -> i32 {
        let balloons = nums.len();
        let mut coins = Vec::with_capacity(balloons + 1);
        coins.push(vec![0; balloons + 1]);

        for count in 1..=balloons {
            let mut curr_coins = vec![];

            for start in 0..=(balloons - count) {
                let left = if start == 0 { 1 } else { nums[start - 1] };
                let end = start + count;
                let right = if end == balloons { 1 } else { nums[end] };

                let mut curr_max = 0;
                for last_balloon in start..end {
                    curr_max = curr_max.max(
                        left * nums[last_balloon] * right
                        + coins[last_balloon - start][start]
                        + coins[end - last_balloon - 1][last_balloon + 1]
                    );
                }

                curr_coins.push(curr_max);
            }

            coins.push(curr_coins);
        }

        coins[balloons][0]
    }
}