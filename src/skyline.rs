use std::collections::btree_map::Entry::Occupied;
use std::collections::BTreeMap;

pub struct Solution;

impl Solution {
    fn get_y_max(y_values: &BTreeMap<i32, i32>) -> i32 {
        *y_values.keys().next_back().unwrap_or(&0)
    }

    pub fn get_skyline(buildings: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let mut lines = Vec::with_capacity(buildings.len() << 1);

        for building in buildings {
            lines.push((building[0], -building[2], 0));
            lines.push((building[1], building[2], 1));
        }

        lines.sort();

        let mut skyline = Vec::new();
        let mut y_values = BTreeMap::new();
        let mut prev_x = -1;

        for (x, mut y, side) in lines {
            if side == 0 {
                y = -y;
                if x != prev_x && y > Self::get_y_max(&y_values) {
                    skyline.push(vec![x, y]);
                    prev_x = x;
                }
                *y_values.entry(y).or_insert(0) += 1;
            } else if let Occupied(mut y_val) = y_values.entry(y) {
                let count = y_val.get_mut();
                if *count > 1 {
                    *count -= 1;
                } else {
                    y_val.remove();
                    let y_max = Self::get_y_max(&y_values);
                    if y > y_max {
                        skyline.push(vec![x, y_max]);
                    }
                }
            }
        }

        skyline
    }
}
