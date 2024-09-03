pub struct Solution {}

const MOD: u64 = 1_000_000_007;

impl Solution {
    fn get_digit(chr: char) -> usize {
        chr.to_digit(10).unwrap() as usize
    }

    pub fn count_palindromes(s: String) -> i32 {
        let mut right_digits = [0; 10];
        let mut right_counts = [0; 100];
        for digit in s.chars().rev().map(Self::get_digit) {
            let offset = digit * 10;
            for num in 0..10 {
                let i = offset + num;
                right_counts[i] = (right_counts[i] + right_digits[num]) % MOD;
            }
            right_digits[digit] += 1;
        }

        let mut total = 0;
        let mut count = 0;
        let mut left_digits = [0; 10];
        let mut left_counts = [0; 100];
        for digit in s.chars().map(Self::get_digit) {
            right_digits[digit] -= 1;
            let offset = digit * 10;
            for num in 0..10 {
                let i = offset + num;
                let prev_count = (left_counts[i] * right_counts[i]) % MOD;
                right_counts[i] = (right_counts[i] - right_digits[num] + MOD) % MOD;
                count = (count + (left_counts[i] * right_counts[i]) % MOD - prev_count + MOD) % MOD;
            }
            total = (total + count) % MOD;
            for num in 0..10 {
                let i = offset + num;
                let prev_count = (left_counts[i] * right_counts[i]) % MOD;
                left_counts[i] = (left_counts[i] + left_digits[num]) % MOD;
                count += ((left_counts[i] * right_counts[i]) % MOD - prev_count + MOD) % MOD;
            }
            left_digits[digit] += 1;
        }
        (total % MOD) as i32
    }
}