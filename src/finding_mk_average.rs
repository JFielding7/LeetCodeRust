pub struct MKAverage {
    nums: Box<[i32]>,
    length: usize,
    count: usize,
    start: usize,
    m: usize,
    k: usize,
    sum: i32
}

impl MKAverage {
    pub(crate) fn new(m: i32, k: i32) -> Self {
        let a = m as usize;
        let b = k as usize;
        let length = a - b;
        MKAverage {
            nums: vec![0; length].into_boxed_slice(),
            length,
            count: 0,
            start: 0,
            m: a,
            k: b,
            sum: 0
        }
    }

    pub(crate) fn add_element(&mut self, num: i32) {
        if self.length >= self.m {
            self.sum += self.nums[(self.start + self.length - self.k) % self.length] - self.nums[self.start];
        } else if self.count >= self.k && self.count < self.m - self.k {
            self.sum += num;
        }
        self.nums[self.start] = num;
        self.start = (self.start + 1) % self.length;
        self.count += 1;
        println!("{:?}", self.nums);
    }

    pub(crate) fn calculate_mk_average(&self) -> i32 {
        if self.count < self.m { -1 } else { self.sum / (self.m - (self.k << 1)) as i32 }
    }
}
