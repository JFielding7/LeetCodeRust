use std::cmp::min;

pub struct Solution {}

pub(crate) static mut x: i32 = 0;

impl Solution {
    fn min_turns(string: &[u8], start: usize, end: usize, length: usize, cache: &mut Vec<i32>) -> i32 {
        unsafe {x += 1};
        if start == end {
            return 1;
        }
        let cache_index = start * length + end;
        let cached_val = cache[cache_index];
        if cached_val != -1 {
            return cached_val;
        }
        if string[start] == string[end] {
            let turns = min(Self::min_turns(string, start + 1, end, length, cache),
                       Self::min_turns(string, start, end - 1, length, cache));
            cache[cache_index] = turns;
            return turns;
        }
        let mut turns = i32::MAX;
        for split in start..end {
            turns = min(turns, Self::min_turns(string, start, split, length, cache)
                + Self::min_turns(string, split + 1, end, length, cache));
        }
        cache[cache_index] = turns;
        turns
    }

    pub fn strange_printer(s: String) -> i32 {
        let len = s.len();
        Self::min_turns(s.as_bytes(), 0, len - 1, len, &mut vec![-1; len * len])
    }
}