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

use word_ladder::Solution;

fn main() {
    let begin_word = "hit";
    let end_word = "cog";
    let list = ["hot", "dot", "dog", "lot", "log", "cog"];
    unsafe {
        println!("{}", Solution::ladder_length(begin_word.to_string(), end_word.to_string(), list.iter().map(|s| s.to_string()).collect()));
    }
}