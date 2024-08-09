pub struct Solution {}

impl Solution {
    fn get_sums(nums: Vec<i32>) -> (Box<[i64]>, Box<[i64]>, Box<[usize]>) {
        let sums_length = nums.len() + 1;
        let mut sums= Vec::with_capacity(sums_length);
        sums.push(0);
        let mut sum = 0;
        for num in nums {
            sum += num as i64;
            sums.push(sum);
        }

        let mut sums_and_indices: Vec<(&i64, usize)> = sums.iter().zip(0..sums_length).collect();
        sums_and_indices.sort();

        let mut sorted_sums = Vec::with_capacity(sums_length);
        let mut sum_indices = vec![0; sums_length].into_boxed_slice();
        for (i, (sum, index)) in sums_and_indices.into_iter().enumerate() {
            sorted_sums.push(*sum);
            sum_indices[index] = i;
        }
        (sums.into_boxed_slice(), sorted_sums.into_boxed_slice(), sum_indices)
    }

    fn bisect_right(nums: &Box<[i64]>, num: i64) -> usize {
        let mut start = 0;
        let mut end = nums.len() - 1;
        while start <= end {
            let mid = (start + end) >> 1;
            if num < nums[mid] {
                if mid == 0 { return 0; }
                end = mid - 1;
            }
            else {
                start = mid + 1;
            }
        }
        start
    }

    fn count_sums(threshold: i64, offset: usize, mut end: usize, sorted_sums: &Box<[i64]>, tree: &Box<[i32]>) -> i32 {
        let mut start = Self::bisect_right(&sorted_sums, threshold) + offset;
        let mut count = 0;
        while start != end {
            if (start & 1) == 0 {
                count += tree[start];
                start += 1;
            }
            if (end & 1) == 0 {
                end -= 1;
                count += tree[end];
            }
            start >>= 1;
            end >>= 1;
        }
        count
    }

    pub fn count_range_sum(nums: Vec<i32>, lower: i32, upper: i32) -> i32 {
        let (sums, sorted_sums, sum_indices) = Self::get_sums(nums);
        let offset = sums.len() - 1;
        let tree_len = (offset << 1) + 1;
        let mut tree = vec![0; tree_len].into_boxed_slice();
        let mut count = 0;
        for (i, sum) in sum_indices.iter().zip(sums.iter()) {
            count += Self::count_sums(sum - upper as i64 - 1, offset, tree_len, &sorted_sums, &tree)
                - Self::count_sums(sum - lower as i64, offset, tree_len, &sorted_sums, &tree);
            let mut j = i + offset;
            while j > 0 {
                tree[j] += 1;
                j = (j - 1) >> 1;
            }
            tree[0] += 1;
        }
        count
    }
}