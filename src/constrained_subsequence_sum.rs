use std::collections::{HashMap, VecDeque};
use std::hash::Hash;

struct Heap<T: Clone> {
    arr: Vec<T>,
    indices: HashMap<T, usize>,
}

impl<T: Clone + Eq + Hash + PartialOrd> Heap<T> {
    fn new() -> Heap<T> {
        Heap { arr: Vec::new(), indices: HashMap::new() }
    }

    fn swap(&mut self, i: usize, j: usize) {
        self.indices.insert(self.arr[i].clone(), j);
        self.indices.insert(self.arr[j].clone(), i);
        self.arr.swap(i, j);
    }

    fn set(&mut self, i: usize, val: T) {
        self.indices.insert(val.clone(), i);
        self.arr[i] = val;
    }

    fn append(&mut self, val: T) {
        self.indices.insert(val.clone(), self.len());
        self.arr.push(val);
    }

    fn heapify_down(&mut self, mut i: usize) {
        let len = self.len();

        while i < len - 1 >> 1 {
            let mut child = (i << 1) + 1;
            if self.arr[child + 1] > self.arr[child] {
                child += 1;
            }

            if self.arr[i] < self.arr[child] {
                self.swap(i, child);
                i = child;
            } else {
                break;
            }
        }

        if (len & 1) == 0 && ((i + 1) << 1) == len && self.arr[i] < self.arr[(i << 1) + 1]  {
            self.swap(i, (i << 1) + 1);
        }
    }

    fn remove(&mut self, val: T) {
        let last = self.arr.pop().unwrap();
        let i = self.indices.remove(&val).unwrap();

        if i != self.len() {
            self.set(i, last);
            self.heapify_down(i);
        }
    }

    fn push(&mut self, val: T) {
        let mut i = self.len();
        self.append(val);

        while i > 0 {
            let parent = (i - 1) >> 1;
            if self.arr[i] > self.arr[parent] {
                self.swap(i, parent);
                i = parent;
            } else {
                break;
            }
        }
    }

    fn peek(&self) -> T {
        self.arr[0].clone()
    }

    fn len(&self) -> usize {
        self.arr.len()
    }
}

pub struct Solution;

impl Solution {
    pub fn constrained_subset_sum(nums: Vec<i32>, k: i32) -> i32 {
        let n = k as usize;

        let mut heap = Heap::new();
        heap.push((0, usize::MAX));

        let mut queue = VecDeque::with_capacity(n);
        let mut max_subsequence_sum = i32::MIN;

        for (i, num) in nums.iter().enumerate() {
            let (prev_max, j) = heap.peek();
            let max_sum = num + prev_max;

            heap.push((max_sum, i));
            if i >= n {
                heap.remove((queue.pop_front().unwrap(), i - n));
            }

            queue.push_back(max_sum);
            max_subsequence_sum = max_subsequence_sum.max(max_sum);

        }

        max_subsequence_sum
    }
}
