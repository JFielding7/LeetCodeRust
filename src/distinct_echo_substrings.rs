use std::collections::HashSet;

pub struct Solution {}

const BASE: u64 = 37;
const MOD: u64 = (1 << 31) - 1;

impl Solution {
    fn pow_mod(mut exponent: u64) -> u64 {
        let mut res = 1;
        let mut curr_pow = BASE;

        while exponent > 0 {
            if (exponent & 1) == 1 {
                res = (res * curr_pow) % MOD;
            }
            curr_pow = (curr_pow * curr_pow) % MOD;
            exponent >>= 1;
        }
        res
    }

    fn get_hash(string: &str) -> u64 {
        let mut hash = 0;
        for (e, &b) in string.as_bytes().iter().rev().enumerate() {
            hash = (hash + (b as u64) * Self::pow_mod(e as u64)) % MOD;
        }
        hash
    }

    pub fn distinct_echo_substrings(text: String) -> i32 {
        let len = text.len();
        let mut count = 0;

        let mut seen: HashSet<&str> = HashSet::new();
        for curr_len in 1..=(len >> 1) {
            let factor = Self::pow_mod(curr_len as u64);
            let mut hash0 = Self::get_hash(&text[..curr_len - 1]);
            let mut hash1 = Self::get_hash(&text[curr_len..(curr_len << 1) - 1]);
            let mut prev0 = 0;
            let mut prev1 = 0;

            for i in curr_len..=(len - curr_len) {
                hash0 = (hash0 * BASE + text.as_bytes()[i - 1] as u64 + MOD - prev0 * factor) % MOD;
                hash1 = (hash1 * BASE + text.as_bytes()[i + curr_len - 1] as u64 + MOD - prev1 * factor) % MOD;

                let str0 = &text[(i - curr_len)..i];
                let str1 = &text[i..(i + curr_len)];
                if hash0 == hash1 && str0 == str1 && !seen.contains(&str0) {
                    seen.insert(str0);
                    count += 1;
                }

                prev0 = text.as_bytes()[i - curr_len] as u64;
                prev1 = text.as_bytes()[i] as u64;
            }
        }

        count
    }
}