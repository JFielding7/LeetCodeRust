pub struct Solution {}

impl Solution {
    pub fn max_sum_of_three_subarrays(nums: Vec<i32>, k: i32) -> Vec<i32> {
        let n = k as usize;
        let mut optimal = vec![(i32::MIN, 0, i32::MIN, 0, 0); n];
        let mut sum: i32 = nums[..n - 1].iter().sum();
        let mut start = 0;
        let mut max_sum = i32::MIN;
        let (mut max0, mut s, mut max1, mut s0, mut s1) = (0, 0, 0, 0, 0);
        let mut indices = [0, 0, 0];
        for (i, num) in nums[n - 1..].iter().enumerate() {
            sum += num;
            let (m0, a, m1, a0, a1) = optimal[start];
            if i >= (n << 1) && sum + m1 > max_sum {
                max_sum = sum + m1;
                indices = [a0, a1, i];
            }
            if i >= n && sum + m0 > max1 {
                max1 = sum + m0;
                s0 = a;
                s1 = i;
            }
            if sum > max0 {
                max0 = sum;
                s = i;
            }
            optimal[start] = (max0, s, max1, s0, s1);
            start = (start + 1) % n;
            sum -= nums[i];
        }
        indices.iter().map(|&i| i as i32).collect()
    }
}