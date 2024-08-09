pub struct Solution {}

impl Solution {
    fn swap(nums: &mut Box<[(i32, usize)]>, indices: &mut Box<[usize]>, i: usize, j:usize) {
        let element0 = nums[i];
        let element1 = nums[j];
        nums[i] = element1;
        nums[j] = element0;
        indices[element0.1] = j;
        indices[element1.1] = i;
    }

    fn heapify(heap: &mut Box<[(i32, usize)]>, mut parent: usize, max_parent: usize, length: usize, indices: &mut Box<[usize]>) {
        while parent < max_parent {
            let left_val = heap[(parent << 1) + 1];
            let right_val = heap[(parent << 1) + 2];
            let (max_val, child) = if left_val > right_val {
                (left_val, (parent << 1) + 1)
            }
            else {
                (right_val, (parent << 1) + 2)
            };
            if heap[parent] < max_val {
                Self::swap(heap, indices, parent, child);
                parent = child;
            }
            else {
                break;
            }
        }
        if (length & 1) == 0 && parent == max_parent && heap[parent] < heap[parent * 2 + 1] {
            Self::swap(heap, indices, parent, parent * 2 + 1);
        }
    }

    pub fn max_sliding_window(nums: Vec<i32>, k: i32) -> Vec<i32> {
        let n = k as usize;
        let length = nums.len();
        let mut window: Box<[(i32, usize)]> = vec![(0, 0); n].into_boxed_slice();
        let mut indices: Box<[usize]> = vec![0; length].into_boxed_slice();
        for (i, num) in nums[0..n].iter().enumerate() {
            window[i] = (*num, i);
            indices[i] = i;
        }
        let length = window.len();
        let max_parent = (length - 1) >> 1;
        for i in (0..max_parent + (length & 1 ^ 1)).rev() {
            Self::heapify(&mut window, i, max_parent, length, &mut indices);
        }

        let mut window_maxes: Vec<i32> = Vec::with_capacity(length - n + 1);
        window_maxes.push(window[0].0);
        for (i, num) in (n..).zip(nums[n..].iter()) {
            let heap_index = indices[i - n];
            window[heap_index] = (*num, i);
            indices[i] = heap_index;

            let mut curr = heap_index;
            while curr > 0 && window[curr] > window[(curr - 1) >> 1] {
                Self::swap(&mut window, &mut indices, curr, (curr - 1) >> 1);
                curr = (curr - 1) >> 1;
            }

            Self::heapify(&mut window, curr, max_parent, length, &mut indices);
            window_maxes.push(window[0].0);
        }
        window_maxes
    }
}
