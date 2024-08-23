use std::cmp::max;

pub struct Solution {}

const ALPHABET_SIZE: usize = 26;

impl Solution {
    fn calculate_max_score(words: &Vec<String>, start: usize, letters: [i32; ALPHABET_SIZE], scores: &Vec<i32>) -> i32 {
        let mut max_score = 0;
        for i in start..words.len() {
            let mut next_letters = letters.clone();
            let mut valid = true;
            let mut word_score = 0;
            for chr in words[i].as_bytes() {
                let index = (chr - b'a') as usize;
                word_score += scores[index];
                if next_letters[index] == 0 {
                    valid = false;
                    continue;
                }
                next_letters[index] -= 1;
            }
            if valid {
                max_score = max(max_score, word_score + Self::calculate_max_score(words, i + 1, next_letters, scores));
            }
        }
        max_score
    }

    pub fn max_score_words(words: Vec<String>, letters: Vec<char>, score: Vec<i32>) -> i32 {
        let mut letter_counts = [0; ALPHABET_SIZE];
        for chr in letters {
            letter_counts[(chr as u8 - b'a') as usize] += 1;
        }
        Self::calculate_max_score(&words, 0, letter_counts, &score)
    }
}