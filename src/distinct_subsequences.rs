use std::collections::HashMap;

pub struct Solution {}

impl Solution {
    pub fn num_distinct(s: String, t: String) -> i32 {
        let mut indicies = HashMap::new();
        for (i, chr) in t.chars().enumerate() {
            indicies.entry(chr).or_insert(vec![]).push(i);
        }
        let len = t.len();
        let mut counts = vec![0; len + 1];
        counts[0] = 1;
        for chr in s.chars() {
            let index_vec_opt = indicies.get(&chr);
            if index_vec_opt.is_some() {
                index_vec_opt.unwrap().iter().rev().for_each(|&i| counts[i + 1] += counts[i]);
            }
        }
        counts[len]
    }
}