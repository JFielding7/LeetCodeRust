pub struct Solution {}

impl Solution {
    pub fn find_kth_number(mut m: i32, mut n: i32, k: i32) -> i32 {
        if n < m {
            m ^= n;
            n ^= m;
            m ^= n;
        }

        let mut lower = 1;
        let mut upper = m * n;
        loop {
            let mid = (upper + lower) >> 1;
            let mut occur = 0;
            let full_rows_below = (mid - 1) / n;
            let mut nums_below = full_rows_below * n;

            for i in full_rows_below + 1..=m {
                nums_below += (mid - 1) / i;
                if mid % i == 0 {
                    occur += 1;
                }
            }

            if nums_below < k && nums_below + occur >= k {
                return mid;
            } else if nums_below < k {
                lower = mid + 1;
            } else {
                upper = mid - 1;
            }
        }
    }
}