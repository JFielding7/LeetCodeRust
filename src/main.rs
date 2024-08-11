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

use palindrome_pairs::Solution;

fn main() {
    let words = ["a",""];
    let v: Vec<String> = words.iter().map(|s| s.to_string()).collect();
    let pairs = Solution::palindrome_pairs(v);
    println!("{pairs:?}");
    // let mut m = MedianFinder::new();
    // m.add_num(32);
    // m.add_num(12);
    // m.add_num(45);
    // println!("{}", m.get(2));
    // m.print()
}
