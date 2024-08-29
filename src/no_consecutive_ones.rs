pub struct Solution {}

impl Solution {
    fn fibonacci(n: usize) -> Vec<i32> {
        let mut fibs = Vec::with_capacity(n);
        fibs.push(1);
        fibs.push(2);
        let mut prev = 1;
        let mut curr = 2;
        for _ in 0..(n - 2) {
            let temp = curr;
            curr += prev;
            prev = temp;
            fibs.push(curr);
        }
        fibs
    }

    pub fn find_integers(mut n: i32) -> i32 {
        n += 1;
        let bits = 32 - n.leading_zeros();
        let fib = Self::fibonacci(bits as usize);
        let mut count = 0;
        let mut shift = bits as i32 - 1;
        while shift > -1 {
            if ((n >> shift + 1) & 1) == 1 {
                return count + fib[shift as usize + 1];
            } else if ((n >> shift) & 1) == 1 {
                count += fib[shift as usize];
                shift -= 2;
            } else {
                shift -= 1;
            }
        }
        count + (shift == -1 && (n & 1) == 1) as i32
    }
}