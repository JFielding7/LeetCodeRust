use std::cmp::min;

pub struct Solution {}

impl Solution {
    fn min_edits(string: &[u8], k: usize, start: usize, end: usize, edits: &Vec<i32>, cache: &mut Vec<i32>) -> i32 {
        if end - start == k {
            return 0;
        }
        if k == 1 {
            return edits[start * end + (end - 1)];
        }
        let cache_index = (k - 1) * end + start;
        let cached_min = cache[cache_index];
        if cached_min != -1 {
            return cached_min;
        }

        let mut min_edits = i32::MAX;
        for i in start..(end - k + 1) {
            min_edits = min(min_edits, edits[start * end + i]
                + Self::min_edits(string, k - 1, i + 1, end, edits, cache));
        }
        cache[cache_index] = min_edits;
        min_edits
    }

    fn get_edits(string: &[u8], length: usize) -> Vec<i32> {
        let mut edits = vec![0; length * length];
        for diff in 1..length {
            for end in diff..length {
                let start = end - diff;
                edits[start * length + end] = edits[(start + 1) * length + (end - 1)]
                    + ((string[start] != string[end]) as i32);
            }
        }
        edits
    }

    pub fn palindrome_partition(s: String, k: i32) -> i32 {
        let len = s.len();
        let string = s.as_bytes();
        Self::min_edits(string, k as usize, 0, len, &Self::get_edits(string, len), &mut vec![-1; (k as usize) * len])
    }
}