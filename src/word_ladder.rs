use std::collections::{HashSet, VecDeque};

pub struct Solution {}

impl Solution {
    pub fn ladder_length(begin_word: String, end_word: String, word_list: Vec<String>) -> i32 {
        let words: HashSet<&String> = word_list.iter().collect();
        if !words.contains(&end_word) { return 0; }

        let mut visited = HashSet::from([begin_word.clone()]);
        let mut queue = VecDeque::from([(begin_word, 1)]);

        while let Some((word, length)) = queue.pop_front() {
            let mut curr_word = word.clone();
            for i in 0..curr_word.len() {
                unsafe {
                    for b in b'a'..(b'z' + 1) {
                        curr_word.as_bytes_mut()[i] = b;
                        if curr_word == end_word {
                            return length + 1;
                        }
                        if words.contains(&curr_word) && !visited.contains(&curr_word) {
                            visited.insert(curr_word.clone());
                            queue.push_back((curr_word.clone(), length + 1));
                        }
                    }
                    curr_word.as_bytes_mut()[i] = word.as_bytes()[i];
                }
            }
        }
        0
    }
}