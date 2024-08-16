mod substring_cat;
mod histogram_rectangle;
mod maximal_rectangle;
mod stock;
mod longest_increasing_path;
mod minimum_window_substring;
mod shortest_palindrome;
mod sliding_window_maximum;
mod count_of_range_sum;
mod split_array_largest_sum;
mod palindrome_partitioning;
mod find_min;
mod find_min_duplicates;
mod min_stack;
mod median_finder;
mod palindrome_pairs;
mod reverse_pairs;
mod sliding_puzzle;
mod sliding_window_median;
mod palindrome_partitioning_iii;
mod min_insertions_to_make_palindrome;
mod two_sum;
mod word_ladder;
mod permutation_sequence;
mod nth_magical_number;
mod candy;
mod longest_valid_parentheses;
mod kth_smallest_pair_distance;
mod longest_duplicate_substring;
mod max_points_on_line;
mod stock_iv;
mod max_array_distance;
mod serialize_and_deserialize_binary_tree;

use std::cell::RefCell;
use std::rc::Rc;
use serialize_and_deserialize_binary_tree::{TreeNode, Codec};

fn main() {
    let root = TreeNode {
        val: 0,
        left: Some(Rc::new(RefCell::new(TreeNode { val: 1, left: None, right: None }))),
        right: Some(Rc::new(RefCell::new(TreeNode { val: 2, left: None, right: None })))
    };
    let c= Codec::new();
    let s = c.serialize(Some(Rc::new(RefCell::new(root))));
    println!("{s}");
    let t = c.deserialize(s);
    println!("{}", t.as_ref().unwrap().borrow().val);
}