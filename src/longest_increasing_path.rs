// pub struct Solution {}
//
// impl Solution {
//     const DELTA: [(i32, i32); 4] = [(0, -1), (0, 1), (-1, 0), (1, 0)];
//
//     fn dfs(matrix: &Vec<Vec<i32>>, rows: i32, cols: i32, i: usize, j: usize, paths: &mut Box<[i32]>) {
//         paths[i * (cols as usize) + j] = 1;
//         let mut stack: Vec<(i32, i32, usize)> = vec![(i as i32, j as i32, 0)];
//
//         let mut stack_index = 0;
//         let mut length = 0;
//         while let Some(&(r0, c0, mut delta_index)) = stack.get(stack_index) {
//             let value = matrix[r0 as usize][c0 as usize];
//             while delta_index < 4 {
//                 let (dr, dc) = Self::DELTA[delta_index];
//                 let r1 = r0 + dr;
//                 let c1 = c0 + dc;
//                 if r1 > -1 && r1 < rows && c1 > -1 && c1 < cols {
//                     let neighbor = matrix[r1 as usize][c1 as usize];
//                     if neighbor > value {
//                         let paths_index = (r1 * cols + c1) as usize;
//                         if paths[paths_index] == 0 {
//                             stack[stack_index].2 = delta_index + 1;
//                             stack_index += 1;
//                             stack.push((r1, c1, 0));
//                             break;
//                         }
//                     }
//                 }
//                 delta_index += 1;
//             }
//             if !found_neighbor { length = 1; }
//             paths[i * (cols as usize) + j] = length;
//             length += 1;
//             stack_index -= 1;
//             stack.pop();
//         }
//     }
//
//     pub fn longest_increasing_path(matrix: Vec<Vec<i32>>) -> i32 {
//         let rows = matrix.len();
//         let cols = matrix[0].len();
//         let mut paths = vec![0; (rows * cols)].into_boxed_slice();
//         for (i, row) in matrix.iter().enumerate() {
//             for (j, col) in row.iter().enumerate() {
//                 if paths[i * cols + j] == 0 {
//                     Self::dfs(&matrix, rows as i32, cols as i32, i, j, &mut paths);
//                 }
//             }
//         }
//         *paths.iter().max().unwrap()
//     }
// }