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
mod max_points_with_cost;
mod stone_game_ii;
mod strange_printer;

use strange_printer::Solution;

fn main() {
    // acadababcb
    // acadabcb
    let s = "bbbbbbbbbbb";
    println!("{}", Solution::strange_printer(s.to_string()));
    unsafe { println!("{} {}", s.len(), strange_printer::x); }
}