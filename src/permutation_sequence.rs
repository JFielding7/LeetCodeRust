pub struct Solution {}

impl Solution {
    pub fn get_permutation(n: i32, mut k: i32) -> String {
        let mut digits: Vec<char> = (1..=n).map(|num| char::from_digit(num as u32, 10).unwrap()).collect();
        let mut string = String::new();
        let len = n as usize;

        let mut fact: usize = 1;
        for i in 2..len { fact *= i; }

        let mut m = k as usize - 1;
        for i in (1..len).rev() {
            let j = m / fact;
            string.push(digits[j]);
            digits.remove(j);
            m -= j * fact;
            fact /= i;
        }
        string.push(digits[0]);
        string
    }
}