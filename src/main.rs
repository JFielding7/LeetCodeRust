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
mod max_score_formed_by_letters;
mod stone_game_iii;
mod jump_game_v;
mod distinct_subsequences;
mod binary_tree_cameras;
mod reducing_dishes;
mod max_dot_product_of_two_subsequences;
mod shortest_common_super_sequence;
mod no_consecutive_ones;
mod stone_game_iv;
mod minimum_cost_to_cut_stick;
mod stone_game_viii;
mod minimum_number_of_increments_on_subarrays_to_form_a_target_array;
mod count_palindromic_subsequences;
mod maximum_sum_of_3_non_overlapping_subarrays;
mod frog_jump;
mod russian_doll_envelopes;
mod russian_doll_slow;
mod finding_mk_average;
mod arithmetic_slices_ii_subsequences;
mod minimum_distance_to_type_word_using_two_fingers;
mod count_unique_characters_of_all_substrings_of_a_given_string;
mod longest_chunked_palindrome_decomposition;
mod maximize_palindrome_length_from_subsequences;
mod kth_smallest_element_matrix_of_sorted_rows;
mod max_value_of_equation;
mod maximum_performance_of_a_team;
mod n_queens;
mod profitable_schemes;
mod distinct_echo_substrings;
mod largest_palindrome_divisible_by_k;
mod count_different_palindromic_subsequences;
mod n_queens1;
mod split_array_with_same_average;

use rand::Rng;
use split_array_with_same_average::Solution;

fn random_array(n: usize, a: i32, b: i32) -> Vec<i32> {
    let mut rng = rand::thread_rng();
    (0..n).map(|_| rng.gen_range(a..b)).collect()
}

fn main() {
    let nums = random_array(100, 0, 1000);
    println!("{}", Solution::split_array_same_average(nums.to_vec()));
}