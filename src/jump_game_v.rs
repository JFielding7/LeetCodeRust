use std::cmp::max;

pub struct Solution {}

impl Solution {
    pub fn max_jumps(arr: Vec<i32>, d: i32) -> i32 {
        let dist = d as usize;
        let mut stack: Vec<(usize, i32)> = Vec::with_capacity(arr.len());
        let mut max_jumps = 0;
        for (i, &num) in arr.iter().enumerate() {
            let mut curr_jumps = 1;
            let mut len = stack.len();
            let mut prev = num;
            while len > 0 {
                let (j, jumps) = stack[len - 1];
                let curr = arr[j];
                if i - j > dist || curr >= num {
                    break;
                }
                curr_jumps = max(curr_jumps, jumps + 1);
                prev = curr;
                len -= 1;
                if len > 0 {
                    let (a, b) = stack[len - 1];
                    if j - a <= dist && arr[a] != curr {
                        stack[len - 1] = (a, max(b, jumps + 1));
                    }
                }
            }
            stack.split_off(len);
            stack.push((i, curr_jumps));
            max_jumps = max(curr_jumps, max_jumps);
        }

        let last = stack.len() - 1;
        let (mut i, mut prev_jumps) = stack[last];
        for k in (0..last).rev() {
            let (j, mut curr_jumps) = stack[k];
            if i - j <= dist && arr[i] < arr[j] {
                curr_jumps = max(curr_jumps, prev_jumps + 1);
            }
            max_jumps = max(max_jumps, curr_jumps);
            i = j;
            prev_jumps = curr_jumps;
        }
        max_jumps
    }
}