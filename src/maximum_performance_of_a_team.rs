use std::cmp::{max, Reverse};
use std::collections::BinaryHeap;

const MOD: u128 = 1_000_000_007;

pub struct Solution {}

impl Solution {
    pub fn max_performance(n: i32, speed: Vec<i32>, efficiency: Vec<i32>, k: i32) -> i32 {
        let mut engineers: Vec<(&i32, &i32)> = efficiency.iter().zip(&speed).collect();
        engineers.sort_unstable_by(|&e0, &e1| e1.0.cmp(&e0.0));

        let mut speeds = BinaryHeap::with_capacity(k as usize);
        let mut max_speed_sum = 0;
        let mut max_perf = 0;

        for (curr_efficiency, curr_speed) in engineers.iter().map(|engineer| (*engineer.0 as u128, *engineer.1 as u128)) {
            max_perf = max(max_perf, (max_speed_sum + curr_speed) * curr_efficiency);
            if k > 1 {
                if speeds.len() < k as usize - 1 {
                    speeds.push(Reverse(curr_speed));
                    max_speed_sum += curr_speed;
                } else {
                    let prev_speed = speeds.peek().unwrap().0;
                    if prev_speed < curr_speed {
                        max_speed_sum += curr_speed - prev_speed;
                        speeds.pop();
                        speeds.push(Reverse(curr_speed));
                    }
                }
            }
            println!("{curr_speed} {speeds:?}");
        }

        (max_perf % MOD) as i32
    }
}