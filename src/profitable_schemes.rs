use std::cmp::max;

pub struct Solution {}

const MOD: i32 = 1_000_000_007;

impl Solution {
    pub fn profitable_schemes(n: i32, min_profit: i32, group: Vec<i32>, profit: Vec<i32>) -> i32 {
        let sizes = n + 1;
        let mut prev_schemes = vec![0; ((min_profit + 1) * sizes) as usize];
        prev_schemes.iter_mut().take(sizes as usize).for_each(|count| *count = 1);

        for (p, &g) in profit.iter().zip(group.iter()) {
            if g > n {
                continue;
            }

            let mut schemes = prev_schemes.clone();
            for i in 0..=min_profit {
                let curr_min_profit = max(0, i - p);
                for j in g..sizes {
                    schemes[(i * sizes + j) as usize] = (schemes[(i * sizes + j) as usize] + prev_schemes[(curr_min_profit * sizes + j - g) as usize]) % MOD;
                }
            }
            prev_schemes = schemes;
        }
        prev_schemes[(min_profit * sizes + n) as usize]
    }
}
