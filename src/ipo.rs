pub struct Solution {}

impl Solution {
    fn heapify_down(heap: &mut Vec<i32>, mut i: usize) {
        let len = heap.len();
        while i < len - 1 >> 1 {
            let mut child = (i << 1) + 1;
            if heap[child + 1] > heap[child] {
                child += 1;
            }

            if heap[i] < heap[child] {
                heap.swap(i, child);
                i = child;
            } else {
                break;
            }
        }

        if (len & 1) == 0 && ((i + 1) << 1) == len && heap[i] < heap[(i << 1) + 1]  {
            heap.swap(i, (i << 1) + 1);
        }
    }

    fn heap_push(heap: &mut Vec<i32>, n: i32) {
        let mut i = heap.len();
        heap.push(n);

        while i > 0 {
            let parent = (i - 1) >> 1;
            if heap[i] > heap[parent] {
                heap.swap(i, parent);
                i = parent;
            } else {
                break;
            }
        }
    }

    fn heap_pop(heap: &mut Vec<i32>) -> i32 {
        let val = heap[0];
        let last = heap.pop().unwrap();
        if heap.len() > 0 {
            heap[0] = last;
            Self::heapify_down(heap, 0);
        }
        val
    }

    fn push_all(heap: &mut Vec<i32>, projects: &[(i32, i32)], i: &mut usize, max_cap: i32) {
        projects[*i..].iter().take_while(|(cap, profit)| *cap <= max_cap)
            .for_each(|(cap, profit)| {
                Self::heap_push(heap, *profit);
                *i += 1;
            });
    }

    pub fn find_maximized_capital(mut k: i32, mut w: i32, profits: Vec<i32>, capital: Vec<i32>) -> i32 {
        let mut projects: Vec<(i32, i32)> = capital.into_iter().zip(profits).collect();
        projects.sort_unstable();

        let mut heap: Vec<i32> = Vec::new();
        let mut i = 0;
        Self::push_all(&mut heap, &projects, &mut i, w);

        while k > 0 && heap.len() > 0 {
            w += Self::heap_pop(&mut heap);
            Self::push_all(&mut heap, &projects, &mut i, w);
            k -= 1;
        }

        w
    }
}
