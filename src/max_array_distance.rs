use std::cmp::max;

pub struct Solution {}

impl Solution {
    pub fn max_distance(arrays: Vec<Vec<i32>>) -> i32 {
        let mut min0 = i32::MAX;
        let mut min0_arr = 0;
        let mut min1 = i32::MAX;
        let mut min1_arr = 0;
        let mut max0 = i32::MIN;
        let mut max0_arr = 0;
        let mut max1 = i32::MIN;
        let mut max1_arr = 0;

        for (i, arr) in arrays.iter().enumerate() {
            let first = arr[0];
            if first < min0 {
                min1 = min0;
                min1_arr = min0_arr;
                min0 = first;
                min0_arr = i;
            }
            else if first < min1 {
                min1 = first;
                min1_arr = i;
            }

            let last = arr[arr.len() - 1];
            if last >= max0 {
                max1 = max0;
                max1_arr = max0_arr;
                max0 = last;
                max0_arr = i;
            }
            else if first >= max1 {
                max1 = last;
                max1_arr = i;
            }
        }
        if max0_arr == min0_arr {
            max(max0 - min1, max1 - min0)
        } else {
            max0 - min0
        }
    }
}