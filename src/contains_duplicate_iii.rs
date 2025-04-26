use std::collections::BTreeMap;

pub struct Solution;

impl Solution {
    fn query_range(window: &BTreeMap<i32, i32>, num: i32, value_diff: i32) -> bool {
        window.range(num - value_diff..=num + value_diff).next().is_some()
    }

    pub fn contains_nearby_almost_duplicate(nums: Vec<i32>, index_diff: i32, value_diff: i32) -> bool {
        let i_diff = index_diff as usize;
        let mut window = BTreeMap::new();

        for &num in &nums[..i_diff] {
            if Self::query_range(&window, num, value_diff) {
                return true;
            }
            *window.entry(num).or_insert(0) += 1;
        }

        for (i, &num) in nums[i_diff..].iter().enumerate() {
            if Self::query_range(&window, num, value_diff) {
                return true;
            }
            *window.entry(num).or_insert(0) += 1;

            let count = window.get_mut(&nums[i]).unwrap();
            if *count == 1 {
                window.remove(&nums[i]).unwrap();
            } else {
                *count -= 1;
            }
        }

        false
    }
}