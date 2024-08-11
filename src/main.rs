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

use sliding_puzzle::Solution;

fn main() {
    let b = [[3,2,4],[1,5,0]];
    let board: Vec<Vec<i32>> = b.into_iter().map(|arr| Vec::from(arr)).collect();
    let mut moves = &Solution::sliding_puzzle(board);
    println!("{moves}");
    // while let Some(node) = curr {
    //     let prev = &node.prev;
    //     let mut i = 0;
    //     let pos = node.pos;
    //     while i < 18 {
    //         print!("{} ", pos >> i & 7);
    //         if i % 9 == 6 { println!(); }
    //         i += 3;
    //     }
    //     println!("{}", pos >> 18);
    //     println!("{moves}");
    //     println!();
    //     moves += 1;
    //     curr = prev;
    // }
    // let mut m = MedianFinder::new();
    // m.add_num(32);
    // m.add_num(12);
    // m.add_num(45);
    // println!("{}", m.get(2));
    // m.print()
}
