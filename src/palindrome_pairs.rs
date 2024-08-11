use std::cmp::min;
use std::collections::HashMap;

pub struct Solution {}

struct Node {
    end: i32,
    children: HashMap<u8, Node>
}

impl Node {
    fn new() -> Self {
        Node { end: -1, children: HashMap::new() }
    }
}

impl Solution {
    fn get_palindrome_lengths(string: &[u8]) -> Vec<usize> {
        let l = 2 * string.len();
        let mut lengths = vec![0; l + 1];
        let mut right = 1;
        let mut center = 1;
        for i in 1..l {
            if i == right || lengths[2 * center - i] == right - i - 1 {
                center = i;
                let mut curr_left = (2 * center - right) >> 1;
                let mut curr_right = right >> 1;
                let mut reaches_start = false;
                while curr_right < string.len() && string[curr_left] == string[curr_right] {
                    curr_right += 1;
                    if curr_left == 0 {
                        reaches_start = true;
                        break;
                    }
                    curr_left -= 1;
                }
                right = (curr_right << 1) + 1;
                lengths[i] = if reaches_start { curr_right - curr_left }
                else { curr_right - curr_left - 1 };
            }
            else {
                lengths[i] = min(right - i - 1, lengths[2 * center - i]);
            }
        }
        lengths
    }

    fn build_prefix_tree<'a, T: Iterator<Item=char>>(words: &'a Vec<String>, word_iter: fn(&'a String) -> T) -> Node {
        let mut root = Node::new();
        for (i, word) in words.iter().enumerate() {
            let mut curr = &mut root;
            for chr in word_iter(word) {
                curr = curr.children.entry(chr as u8).or_insert(Node::new());
            }
            curr.end = i as i32;
        }
        root
    }

    pub fn palindrome_pairs(words: Vec<String>) -> Vec<Vec<i32>> {
        let mut pairs = vec![];
        let prefix_tree = Self::build_prefix_tree(&words, |word| word.chars());
        let rev_prefix_tree = Self::build_prefix_tree(&words, |word| word.chars().rev());
        for (i, word) in words.iter().enumerate() {
            let string = word.as_bytes();
            let lengths = Self::get_palindrome_lengths(string);
            let length = word.len();

            let mut j = length;
            let mut curr = Option::from(&prefix_tree);
            while let Some(node) = curr {
                if j == lengths[j] && node.end != -1 && node.end != i as i32 {
                    pairs.push(vec![node.end, i as i32]);
                }
                if j == 0 { break; }
                j -= 1;
                curr = node.children.get(&string[j]);
            }

            j = 0;
            curr = Option::from(&rev_prefix_tree);
            while let Some(node) = curr {
                if length - j == lengths[j + length] && node.end != -1 && node.end != i as i32 {
                    pairs.push(vec![i as i32, node.end]);
                }
                curr = node.children.get(&string.get(j).unwrap_or(&0));
                j += 1;
                if j == length { break; }
            }
        }
        pairs
    }
}