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