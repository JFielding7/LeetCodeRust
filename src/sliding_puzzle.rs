use std::collections::{HashSet, VecDeque};

pub struct Solution {}

const BOARD_LEN: i32 = 18;
const ROW_BITS: usize = 9;
const COL_BITS: usize = 3;
const CELL_MASK: i32 = 0b111;
const SOLVED: i32 = 1 + (2 << 3) + (3 << 6) + (4 << 9) + (5 << 12) + (15 << BOARD_LEN);
const DELTA: [(i32, i32); 4] = [(-3, 9), (3, 6), (-9, -1), (9, -1)];

impl Solution {
    pub fn sliding_puzzle(board: Vec<Vec<i32>>) -> i32 {
        let mut start: i32 = 0;
        for (i, row) in board.iter().enumerate() {
            for (j, &num) in row.iter().enumerate() {
                let shift = (i * ROW_BITS + j * COL_BITS) as i32;
                start |= num << shift;
                if num == 0 {
                    start |= shift << BOARD_LEN;
                }
            }
        }
        if start == SOLVED {
            return 0;
        }

        let mut queue: VecDeque<(i32, i32)> = VecDeque::from([(start, 0)]);
        let mut visited: HashSet<i32> = HashSet::from([start]);
        while let Some((pos, dist)) = queue.pop_front() {
            let empty_pos = pos >> BOARD_LEN;
            for (change, illegal_cell) in DELTA {
                let next_empty_pos = empty_pos + change;
                if empty_pos != illegal_cell && next_empty_pos > -1 && next_empty_pos < BOARD_LEN {
                    let mut next_pos = pos + (change << BOARD_LEN)
                        & !(CELL_MASK << next_empty_pos)
                        | ((pos >> next_empty_pos & CELL_MASK) << empty_pos);
                    if next_pos == SOLVED {
                        return dist + 1;
                    }
                    if !visited.contains(&next_pos) {
                        visited.insert(next_pos);
                        queue.push_back((next_pos, dist + 1));
                    }
                }
            }
        }
        -1
    }
}