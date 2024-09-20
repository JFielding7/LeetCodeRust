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

use longest_chunked_palindrome_decomposition::Solution;

fn main() {
    let s = "elvtoelvto";
    println!("{}", Solution::longest_decomposition(s.to_string()));
    // let mut obj = MKAverage::new(3, 1);
    // obj.add_element(3);        // current elements are [3]
    // obj.add_element(1);        // current elements are [3,1]
    // obj.calculate_mk_average(); // return -1, because m = 3 and only 2 elements exist.
    // obj.add_element(10);       // current elements are [3,1,10]
    // obj.calculate_mk_average(); // The last 3 elements are [3,1,10].
    // // After removing smallest and largest 1 element the container will be [3].
    // // The average of [3] equals 3/1 = 3, return 3
    // obj.add_element(5);        // current elements are [3,1,10,5]
    // obj.add_element(5);        // current elements are [3,1,10,5,5]
    // obj.add_element(5);        // current elements are [3,1,10,5,5,5]
    // obj.calculate_mk_average(); // The last 3 elements are [5,5,5].
    // After removing smallest and largest 1 element the container will be [5].
    // The average of [5] equals 5/1 = 5, return 5
}