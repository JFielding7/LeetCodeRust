use std::collections::HashMap;

pub struct Solution {}

const MOD: u64 = 4_294_967_291;
const BASE: u64 = 29;

pub static mut count: i32 = 0;

macro_rules! char_val {
    ($char:expr, $pow:expr) => { (($char as u64)) * $pow };
}

impl Solution {
    fn find_duplicate(string: &[u8], end: usize, len: usize, hash: u64, rolling_hashes: &mut HashMap<u64, Vec<usize>>) -> usize {
        // unsafe { count += 1; }
        let curr_start = end - len + 1;
        match rolling_hashes.get_mut(&hash) {
            Some(indices) => {
                for j in indices.iter() {
                    let mut i0 = *j;
                    let mut i1 = curr_start;
                    while i1 <= end && string[i0] == string[i1] {
                        i0 += 1;
                        i1 += 1;
                    }
                    if i1 > end {
                        return curr_start;
                    }
                }
                indices.push(curr_start);
            },
            None => { rolling_hashes.insert(hash, vec![curr_start]); }
        };
        0
    }

    pub fn longest_dup_substring(s: String) -> String {
        let string = s.as_bytes();
        let len = s.len();

        let mut longest_dup_len = 0;
        let mut longest_dup_start = 0;
        let mut start = 1;
        let mut end = len - 1;
        while start <= end {
            let curr_len = start + end >> 1;

            let mut hash = 0;
            let mut pow = 1;
            for chr in s[1..curr_len].chars().rev() {
                hash = (hash + char_val!(chr, pow)) % MOD;
                pow = pow * BASE % MOD;
            }
            hash = (hash + char_val!(string[0], pow)) % MOD;

            let mut rolling_hashes: HashMap<u64, Vec<usize>> = HashMap::from([(hash, vec![0])]);
            let mut dup_not_found = true;
            for i in curr_len..len {
                hash = (
                    (hash + MOD - char_val!(string[i - curr_len], pow) % MOD) * BASE
                    + char_val!(string[i], 1)
                ) % MOD;
                let dup_start = Self::find_duplicate(string, i, curr_len, hash, &mut rolling_hashes);
                if dup_start != 0 {
                    dup_not_found = false;
                    longest_dup_len = curr_len;
                    longest_dup_start = dup_start;
                    start = curr_len + 1;
                    break;
                }
            }
            if dup_not_found {
                end = curr_len - 1;
            }
        }
        s[longest_dup_start..(longest_dup_start + longest_dup_len)].to_string()
    }
}
