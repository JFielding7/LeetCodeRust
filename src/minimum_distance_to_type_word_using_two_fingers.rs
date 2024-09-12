use std::cmp::min;

pub struct Solution {}

const ROW_LEN: i32 = 6;

impl Solution {
    fn letter_dist(a: u8, b: u8) -> i32 {
        let v0 = (a - b'A') as i32;
        let v1 = (b - b'A') as i32;
        (v0 / ROW_LEN - v1 / ROW_LEN).abs() + (v0 % ROW_LEN - v1 % ROW_LEN).abs()
    }

    pub fn minimum_distance(word: String) -> i32 {
        let string = word.as_bytes();
        let max_index = word.len() - 1;
        let mut prefix_dist = 0;
        let mut prev_chr = string[0];

        for &chr in word[1..max_index].as_bytes() {
            prefix_dist += Self::letter_dist(chr, prev_chr);
            prev_chr = chr;
        }

        let mut min_dist = prefix_dist;
        let mut distances = vec![0; max_index];

        for i in (0..max_index - 1).rev() {
            let mut dist = 0;
            for j in (i + 1..max_index).rev() {
                dist = min(distances[j] + Self::letter_dist(string[i], string[j + 1]),
                                    dist + Self::letter_dist(string[j], string[j + 1]));
            }
            distances[i] = dist;
            prefix_dist -= Self::letter_dist(string[i], string[i + 1]);
            min_dist = min(min_dist, dist + prefix_dist);
        }
        min_dist
    }
}