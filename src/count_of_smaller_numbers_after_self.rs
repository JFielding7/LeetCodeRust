use std::collections::HashMap;

pub struct Solution {}

impl Solution {
    fn index_map(mut nums: Vec<i32>) -> HashMap<i32, usize> {
        nums.sort_unstable();

        let mut map = HashMap::new();
        nums.iter().enumerate().for_each(|(i, &num)| { map.insert(num, i); });
        map
    }

    fn count(counts: &Vec<i32>, mut start: usize, mut end: usize) -> i32 {
        let mut total = 0;

        while start != end {
            if (start & 1) == 0 {
                total += counts[start];
                start += 1;
            }
            if (end & 1) == 0 {
                end -= 1;
                total += counts[end];
            }

            start >>= 1;
            end >>= 1;
        }

        total
    }

    fn inc_count(counts: &mut Vec<i32>, mut index: usize) {
        while index > 0 {
            counts[index] += 1;
            index = (index - 1) >> 1;
        }
    }

    pub fn count_smaller(nums: Vec<i32>) -> Vec<i32> {
        let len = nums.len();
        let offset = len - 1;
        let indices = Self::index_map(nums.clone());
        let mut counts = vec![0; (len << 1) - 1];
        let mut smaller: Vec<i32> = vec![0; len];

        for (i, &num) in nums.iter().rev().enumerate() {
            let index = *indices.get(&num).unwrap();
            smaller[len - i - 1] = Self::count(&counts, offset, index + offset);
            Self::inc_count(&mut counts, index + offset);
        }

        smaller
    }
}