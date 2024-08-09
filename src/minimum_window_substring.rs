pub struct Solution {}

use std::collections::HashMap;

impl Solution {
    pub fn min_window(s: String, t: String) -> String {
        let mut cache = HashMap::new();
        t.chars().for_each(|char| *cache.entry(char).or_insert(0) -= 1);

        let mut t_chars: Vec<(usize, char)> = vec![];
        let mut t_chars_index = 0;
        let mut total_remaining = cache.len();
        let mut min_start = 0;
        let mut min_end = s.len();

        for (i, char) in s.chars().enumerate() {
            let option = cache.get_mut(&char);
            if option.is_some() {
                t_chars.push((i, char));
                let diff = option.unwrap();
                *diff += 1;
                if *diff == 0 && total_remaining > 0 { total_remaining -= 1; }
                if total_remaining == 0 {
                    let (mut j, mut t_char) = t_chars[t_chars_index];
                    let mut t_diff = cache.get_mut(&t_char).unwrap();
                    while *t_diff > 0 {
                        *t_diff -= 1;
                        t_chars_index += 1;
                        (j, t_char) = t_chars[t_chars_index];
                        t_diff = cache.get_mut(&t_char).unwrap();
                    }
                    if i - j < min_end - min_start {
                        min_start = j;
                        min_end = i;
                    }
                }
            }
        }
        if total_remaining != 0 { "".to_string() }
        else { s[min_start..(min_end + 1)].to_string() }
    }
}