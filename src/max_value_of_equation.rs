use std::cmp::max;
use std::collections::VecDeque;

pub struct Solution {}

impl Solution {
    pub fn find_max_value_of_equation(points: Vec<Vec<i32>>, k: i32) -> i32 {
        const X: usize = 0;
        const Y: usize = 1;


        let len = points.len();
        let first = &points[0];

        let mut max_vals: VecDeque<(usize, i32)> = VecDeque::new();
        let mut curr_max = i32::MIN;
        let mut j = 1;

        for (i, point) in points.iter().enumerate() {
            if i == j {
                j += 1;
            }

            while j < len && points[j][X] - point[X] <= k {
                let val = points[j][Y] + first[Y] + points[j][X] - first[X];

                let values_len = max_vals.len();
                let mut count = 1;
                while count <= values_len && val > max_vals[values_len - count].1 {
                    count += 1;
                }

                max_vals.truncate(values_len + 1 - count);
                max_vals.push_back((j, val));
                j += 1;
            }

            if max_vals.len() != 0 {
                let (val_i, val) = max_vals[0];
                curr_max = max(curr_max, val + points[i][Y] - first[Y] - points[i][X] + first[X]);

                if i + 1 == val_i {
                    max_vals.pop_front();
                }
            }
        }

        curr_max
    }
}
