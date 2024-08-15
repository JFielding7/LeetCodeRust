// fn prefix_suffix_arr(pattern: &[u8], start: usize, end: usize) -> Vec<usize> {
//     let mut arr: Vec<usize> = vec![0; end - start];
//     let mut i = start;
//     let mut j = start + 1;
//     while j < end {
//         loop {
//             if pattern[i] == ('?' as u8) || pattern[j] == ('?' as u8) || pattern[i] == pattern[j] {
//                 i += 1;
//                 arr[j - start] = i - start;
//                 break;
//             }
//             if i == start {
//                 break;
//             }
//             else {
//                 i = arr[i - start - 1] + start;
//             }
//         }
//         j += 1;
//     }
//     return arr;
// }
//
// fn find_match_end(string: &[u8], mut i: usize, string_length: usize, pattern: &[u8], pattern_start: usize, pattern_end: usize) -> Option<usize> {
//     println!("{} {} {}", i, pattern_start, pattern_end);
//     if pattern_start == pattern_end {
//         return Option::from(i);
//     }
//     let arr = prefix_suffix_arr(pattern, pattern_start, pattern_end);
//     println!("{:?}", arr);
//     let mut j = pattern_start;
//     while i < string_length {
//         loop {
//             if pattern[j] == ('?' as u8) || string[i] == pattern[j] {
//                 j += 1;
//                 if j == pattern_end {
//                     return Option::from(i + 1);
//                 }
//                 break;
//             }
//             if j == pattern_start {
//                 break;
//             }
//             else {
//                 j = arr[j - pattern_start - 1] + pattern_start;
//             }
//         }
//         i += 1;
//     }
//     None
// }

// use std::collections::{HashSet, VecDeque};
// use std::rc::Rc;
//
// pub struct Solution {}
//
// const BOARD_LEN: i32 = 18;
// const ROW_BITS: usize = 9;
// const COL_BITS: usize = 3;
// const CELL_MASK: i32 = 0b111;
// const SOLVED: i32 = 1 + (2 << 3) + (3 << 6) + (4 << 9) + (5 << 12) + (15 << BOARD_LEN);
// const DELTA: [(i32, i32); 4] = [(-3, 9), (3, 6), (-9, -1), (9, -1)];
//
// pub struct Node {
//     pub pos: i32,
//     pub prev: Option<Rc<Node>>
// }
//
// impl Solution {
//     pub fn sliding_puzzle(board: Vec<Vec<i32>>) -> Option<Rc<Node>> {
//         let mut start: i32 = 0;
//         for (i, row) in board.iter().enumerate() {
//             for (j, &num) in row.iter().enumerate() {
//                 let shift = (i * ROW_BITS + j * COL_BITS) as i32;
//                 start |= num << shift;
//                 if num == 0 {
//                     start |= shift << BOARD_LEN;
//                 }
//             }
//         }
//         if start == SOLVED {
//             return Some(Rc::new(Node { pos: 0, prev: None }));
//         }
//
//         let mut queue: VecDeque<(i32, Rc<Node>)> = VecDeque::from([(start, Rc::new(Node { pos: start, prev: None }))]);
//         let mut visited: HashSet<i32> = HashSet::from([start]);
//         while let Some((pos, node)) = queue.pop_front() {
//             let empty_pos = pos >> BOARD_LEN;
//             for (change, illegal_cell) in DELTA {
//                 let next_empty_pos = empty_pos + change;
//                 if empty_pos != illegal_cell && next_empty_pos > -1 && next_empty_pos < BOARD_LEN {
//                     let mut next_pos = pos + (change << BOARD_LEN)
//                         & !(CELL_MASK << next_empty_pos)
//                         | ((pos >> next_empty_pos & CELL_MASK) << empty_pos);
//                     if next_pos == SOLVED {
//                         return Some(Rc::new(Node { pos: next_pos, prev: Some(node) }));
//                     }
//                     if !visited.contains(&next_pos) {
//                         visited.insert(next_pos);
//                         queue.push_back((next_pos, Rc::new(Node { pos: next_pos, prev: Some(Rc::clone(&node)) })));
//                     }
//                 }
//             }
//         }
//         None
//     }
// }

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

// pub fn pow(b: u64, mut n: u64, m: u64) -> u64 {
//     let mut curr_pow = b;
//     let mut res = 1;
//     while n != 0 {
//         if (n & 1) == 1 {
//             res = res * curr_pow % m;
//         }
//         curr_pow = curr_pow * curr_pow % m;
//         n >>= 1;
//     }
//     res
// }