use std::cmp::max;

pub struct Solution {}

impl Solution {
    pub fn max_height(mut cuboids: Vec<Vec<i32>>) -> i32 {
        cuboids.iter_mut().for_each(|mut cuboid| cuboid.sort_unstable());
        cuboids.sort_unstable();

        let mut max_total = 0;
        let mut max_heights = Vec::with_capacity(cuboids.len());

        for (i, cuboid) in cuboids.iter().enumerate() {
            let mut curr_max = 0;
            for j in 0..i {
                if cuboids[j][1] <= cuboid[1] && cuboids[j][2] <= cuboid[2] {
                    curr_max = max(curr_max, max_heights[j]);
                }
            }

            curr_max += cuboid[2];
            max_total = max(max_total, curr_max);
            max_heights.push(curr_max);
        }

        max_total
    }
}