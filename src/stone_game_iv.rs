pub struct Solution {}

pub(crate) static mut x: i32 = 0;

impl Solution {
    fn is_win(n: usize, cache: &mut Vec<u8>) -> u8 {
        unsafe { x += 1; }
        if cache[n] != 2 {
            return cache[n]
        }
        let mut i = 1;
        let mut inc = 1;
        while i <= n {
            if Self::is_win(n - i, cache) == 0 {
                cache[n] = 1;
                return 1;
            }
            inc += 2;
            i += inc;
        }
        cache[n] = 0;
        0
    }

    pub fn winner_square_game(n: i32) -> bool {
        let stones = n as usize;
        Self::is_win(stones, &mut vec![2; stones + 1]) == 1
    }
}