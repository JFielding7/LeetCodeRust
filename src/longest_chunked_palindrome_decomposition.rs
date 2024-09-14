pub struct Solution {}

const BASE: u64 = 29;
const MOD: u64 = (1 << 31) - 1;

impl Solution {
    pub fn longest_decomposition(text: String) -> i32 {
        let bytes = text.as_bytes();
        let len = text.len();
        let mut prefix = 0;
        let mut suffix = 0;
        let mut offset = 0;
        let mut chunks = 1;
        let mut pow = 1;

        for i in 0..len >> 1 {
            prefix = (prefix * BASE + (bytes[i] - b'a') as u64) % MOD;
            suffix = (suffix + (bytes[len - i - 1] - b'a') as u64 * pow) % MOD;
            pow = pow * BASE % MOD;

            if prefix == suffix {
                let mut j = offset;
                let mut k = len - i - 1;
                let mut found = true;

                while j <= i {
                    if bytes[j] != bytes[k] {
                        found = false;
                        break;
                    }
                    j += 1;
                    k += 1;
                }

                if found {
                    chunks += 2;
                    prefix = 0;
                    suffix = 0;
                    offset = i + 1;
                    pow = 1;
                }
            }
        }
        chunks - ((len & 1) == 0 && pow == 1) as i32
    }
}