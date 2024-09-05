use std::cmp::max;
use std::collections::HashMap;

pub struct Solution {}

impl Solution {
    pub fn max_envelopes_slow(mut envelopes: Vec<Vec<i32>>) -> i32 {
        envelopes.sort_unstable_by(|a, b| {
            (a[0], -a[1]).cmp(&(b[0], -b[1]))
        });
        let mut heights: Vec<i32> = envelopes.iter().map(|e| e[1]).collect();
        heights.sort_unstable();
        let mut indices = HashMap::new();
        let mut prev = -1;
        for (i, &height) in heights.iter().enumerate() {
            if height != prev {
                indices.insert(height, i);
                prev = height;
            }
        }

        let offset = envelopes.len() - 1;
        let mut tree = vec![0; offset * 2 + 1];
        let mut max_count = 0;
        for envelop in envelopes {
            let i = indices.get(&envelop[1]).unwrap();
            let mut left = offset;
            let mut right = offset + i;
            let mut count = 0;
            while left != right {
                if (left & 1) == 0 {
                    count = max(count, tree[left]);
                    left += 1;
                }
                if (right & 1) == 0 {
                    right -= 1;
                    count = max(count, tree[right]);
                }
                left >>= 1;
                right >>= 1;
            }
            count += 1;
            let mut j = offset + i;
            while j > 0 {
                tree[j] = max(count, tree[j]);
                j = j - 1 >> 1;
            }
            tree[0] = max(count, tree[0]);
            max_count = max(max_count, count);
        }
        max_count
    }
}