pub struct Solution;

use std::cmp::max;
use std::collections::{HashMap};

struct Word {
    occurrences: usize,
    indices: Vec<usize>
}

impl Solution {
    pub fn find_substring(s: String, words: Vec<String>) -> Vec<i32> {
        let mut cache: HashMap<&str, Word> = HashMap::new();
        for word in &words {
            let word_entry = cache.entry(word.as_str()).or_insert(
                Word { occurrences: 0, indices: vec![0] }
            );
            word_entry.occurrences += 1;
        }

        let string_length = s.len();
        let word_len = words[0].len();
        let cat_len = words.len() * word_len;
        let mut indices = vec![];

        let mut i = 0;
        loop {
            let mut start = i;
            let mut j = i + word_len;
            while j <= string_length {
                match cache.get_mut(&s[(j - word_len)..j]) {
                    Some(word) => {
                        let word_indices = &mut word.indices;
                        let indices_len = word_indices.len();
                        if word.occurrences <= indices_len {
                            start = max(start, word_indices[indices_len - word.occurrences]);
                        }
                        word_indices.push(j);
                    },
                    None => start = j
                }
                if j - start == cat_len { indices.push(start as i32); }
                j += word_len;
            }
            i += 1;
            if i < word_len {
                words.iter().for_each(|word|
                    cache.get_mut(word.as_str()).unwrap().indices = vec![]
                );
            }
            else { break; }
        }
        indices.sort();
        indices
    }
}
