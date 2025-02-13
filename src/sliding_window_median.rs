use std::collections::HashMap;

pub struct Solution {}

struct Heap {
    arr: Vec<i32>,
    indices: HashMap<i32, usize>,
    cmp: fn(&i32, &i32) -> bool
}

impl Heap {
    fn from(arr: Vec<i32>, cmp: fn(&i32, &i32) -> bool) -> Self {
        let len = arr.len();
        let mut heap = Self { arr, indices: HashMap::with_capacity(len), cmp };
        for (i, &num) in heap.arr.iter().enumerate() {
            heap.indices.insert(num, i);
        }
        heap
    }

    fn swap(&mut self, i: usize, j: usize) {
        self.arr.swap(i, j);
        self.indices.insert(self.arr[i], i);
        self.indices.insert(self.arr[j], j);
    }

    fn swap_down(&mut self, mut i: usize) {
        let len = self.arr.len();

        while i < (len - 1 >> 1) {
            let mut child_index = (i << 1) + 1;
            if (self.cmp)(&self.arr[child_index + 1], &self.arr[child_index]) {
                child_index += 1;
            }
            if (self.cmp)(&self.arr[child_index], &self.arr[i]) {
                self.swap(i, child_index);
                i = child_index;
            } else {
                break;
            }
        }

        if (len & 1) == 0 && ((i + 1) << 1) == len && (self.cmp)(&self.arr[i], &self.arr[(i << 1) + 1]) {
            self.swap(i, (i << 1) + 1);
        }
    }

    fn insert(&mut self, val: i32, index: usize) {
        self.arr[index] = val;
        self.indices.insert(val, index);
        self.swap_down(index);
    }

    fn peek(&self) -> i32 {
        self.arr[0]
    }

    fn len(&self) -> usize {
        self.arr.len()
    }
}

impl Solution {
    fn get_median(lower: &Heap, upper: &Heap) -> f64 {
        if lower.len() == upper.len() {
            upper.peek() as f64
        } else {
            (lower.peek() as f64) + (upper.peek() as f64) / 2f64
        }
    }

    pub fn median_sliding_window(nums: Vec<i32>, k: i32) -> Vec<f64> {
        let n = k as usize;
        let mut window = nums[..n].to_vec();
        window.sort_unstable();
        nums[..n >> 1].reverse();

        let lower = Heap::from(nums[..n >> 1].to_vec(), i32::lt);
        let upper = Heap::from(nums[n >> 1..].to_vec(), i32::gt);

        let mut medians = Vec::with_capacity(nums.len() - n);
        medians.push(Self::get_median(&lower, &upper));

        for (i, &num) in nums[n..].iter().enumerate() {
            // if ()
        }

        medians
    }
}