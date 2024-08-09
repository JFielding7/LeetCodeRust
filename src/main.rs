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

use rand::Rng;
use split_array_largest_sum::Solution;

fn main() {
    let mut rng = rand::thread_rng();

    // Generate a Vec of 100 random i32 values between 0 and 100
    let nums: Vec<i32> = (0..1000000).map(|_| rng.gen_range(0..100)).collect();
    let k = 23;
    println!("{}", Solution::split_array(nums, k))
}
