pub struct Solution {}

impl Solution {
    pub fn max_envelopes(mut envelopes: Vec<Vec<i32>>) -> i32 {
        envelopes.sort_unstable_by(|a, b| (a[0], -a[1]).cmp(&(b[0], -b[1])));
        let mut cache = Vec::with_capacity(envelopes.len());
        let mut max_count = 0;
        for envelop in envelopes {
            let height = envelop[1];
            let mut start = 0;
            let mut end = max_count - 1;
            while start <= end {
                let mid = (start + end) >> 1;
                if height > cache[mid as usize] {
                    start = mid + 1;
                } else {
                    end = mid - 1;
                }
            }
            if start == max_count {
                cache.push(height);
                max_count += 1;
            } else {
                cache[start as usize] = height;
            }
        }
        max_count
    }
}