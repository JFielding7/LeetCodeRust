use std::cmp::min;

pub struct Solution {}

impl Solution {
    fn build_delimited_string(s: &String) -> String {
        let mut string = String::from(' ');
        for char in s.chars() {
            string.push(char);
            string.push(' ');
        }
        string.push(' ');
        string
    }

    fn longest_palindromic_prefix_length(s: &String) -> usize {
        let delimited_string = Self::build_delimited_string(s);
        let string = delimited_string.as_bytes();
        let length = string.len();
        let mut lengths = vec![0; length].into_boxed_slice();
        let mut center = 0;
        let mut left_bound = length - 1;

        for i in (0..length).rev() {
            if i == left_bound || i - left_bound == lengths[2 * center - i] {
                center = i;
                let mut right_bound = 2 * i - left_bound;
                while right_bound < length && string[left_bound] == string[right_bound] {
                    if left_bound == 0 { return center; }
                    left_bound -= 1;
                    right_bound += 1;
                }
                lengths[i] = i - left_bound;
            }
            else {
                lengths[i] = min(i - left_bound, lengths[2 * center - i]);
            }
        }
        1
    }

    pub fn shortest_palindrome(s: String) -> String {
        let front: String = s[Self::longest_palindromic_prefix_length(&s)..].chars().rev().collect();
        front + s.as_str()
    }
}
