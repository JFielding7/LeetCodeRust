pub struct Solution {}

impl Solution {
    fn largest_pal_divisible_by_7(digits: usize) -> String {
        let pad = "9".repeat((digits - 1) >> 1);
        let mut prefix_pow = 1;
        let mut curr_pow = 1;
        let mut num = 0;

        for i in 0..digits {
            if digits >> 1 != i + (((i + 1) << 1 == digits) && ((digits & 1) == 0)) as usize {
                num = (num + 9 * curr_pow) % 7;
            }
            else if i == (digits - 1) >> 1 {
                prefix_pow = curr_pow;
            }
            curr_pow = curr_pow * 10 % 7;
        }

        let inc = 1 + 10 * ((digits & 1) ^ 1);
        let mut mid_val = inc * 9;
        while (mid_val * prefix_pow + num) % 7 != 0 {
            mid_val -= inc;
        }

        pad.clone() + &mid_val.to_string() + &pad
    }

    pub fn largest_palindrome(n: i32, k: i32) -> String {
        let digits = n as usize;
        match k {
            1 | 3 | 9 => "9".repeat(digits),
            2 => {
                if digits > 1 {
                    "8".to_string() + &"9".repeat(digits - 2) + "8"
                } else {
                    "8".to_string()
                }
            },
            4 => {
                if digits > 3 {
                    "88".to_string() + &"9".repeat(digits - 4) + "88"
                } else {
                    "8".repeat(digits)
                }
            },
            5 => {
                if digits > 1 {
                    "5".to_string() + &"9".repeat(digits - 2) + "5"
                } else {
                    "5".to_string()
                }
            },
            6 => {
                if digits > 2 {
                    let mid = if (digits & 1) == 0 {
                        "77"
                    } else {
                        "8"
                    };
                    let nines = "9".repeat((digits - 3) >> 1);
                    "8".to_string() + &nines + mid + &nines + "8"
                } else {
                    "6".repeat(digits)
                }
            },
            7 => Self::largest_pal_divisible_by_7(digits),
            8 => {
                if digits > 6 {
                    "888".to_string() + &"9".repeat(digits - 6) + "888"
                } else {
                    "8".repeat(digits)
                }
            },
            _ => "".to_string()
        }
    }
}
