pub struct Solution {}

const ALPHABET_SIZE: usize = 26;

impl Solution {
    pub fn unique_letter_string(s: String) -> i32 {
        let mut indices: [(i32, i32); ALPHABET_SIZE] = [(-1, -1); 26];
        let mut count = 0;
        let mut i = 0;
        for chr in s.as_bytes().iter().map(|&b| (b - b'A') as usize) {
            let (i0, i1) = indices[chr];
            count += (i1 - i0) * (i - i1);
            indices[chr] = (i1, i);
            i += 1;
        }
        let len = s.len() as i32;
        indices.iter().for_each(|(i, j)| count += (j - i) * (len - j));
        count
    }
}