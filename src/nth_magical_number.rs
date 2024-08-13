pub struct Solution {}

const MOD: i64 = 1_000_000_007;

impl Solution {
    fn gcd(mut a: i64, mut b: i64) -> i64 {
        while b != 0 {
            let temp = a;
            a = b;
            b = temp % b;
        }
        a
    }

    pub fn nth_magical_number(n: i32, a: i32, b: i32) -> i32 {
        let x = a as i64;
        let y = b as i64;
        let gcd = Self::gcd(x, y);
        let lcm = x * y / gcd;

        let period = (x + y) / gcd - 1;
        let index = n as i64;
        let full_periods = index / period;
        let i = index - full_periods * period;

        let mut start = 0;
        let mut end = lcm - 1;
        while start <= end {
            let mid = (start + end) >> 1;
            let count = mid / x + mid / y;
            if count < i { start = mid + 1; }
            else { end = mid - 1; }
        }
        ((lcm % MOD * full_periods % MOD + start) % MOD) as i32
    }
}