pub struct Solution {}

use std::cmp::max;

impl Solution {
    fn update_area(areas: &mut Box<[i32]>, heights: &Box<[i32]>, l_stack: &mut Vec<usize>, r_stack: &mut Vec<usize>, height: i32, i: usize) -> i32 {
        let mut r = r_stack.len();
        loop {
            if r == 0 {
                r_stack.clear();
                break;
            }
            r -= 1;
            let j = r_stack[r];
            let curr_height = heights[j];
            if curr_height <= height {
                r_stack.truncate(r + 1);
                break;
            }
            areas[j] += curr_height * (i - j) as i32;
        }
        let mut l = l_stack.len();
        loop {
            if l == 0 { break; }
            l -= 1;
            let j = l_stack[l];
            let curr_height = heights[j];
            if curr_height < height {
                l_stack.truncate(l + 1);
                return height * (i - j - 1) as i32;
            }
        }
        l_stack.clear();
        height * i as i32
    }

    fn largest_rectangle_area(heights: &Box<[i32]>) -> i32 {
        let mut areas: Box<[i32]> = vec![0; heights.len()].into_boxed_slice();
        let mut r_stack: Vec<usize> = vec![];
        let mut l_stack: Vec<usize> = vec![];
        let mut prev = 0;

        for (i, &height) in heights.iter().enumerate() {
            if height < prev {
                areas[i] += Self::update_area(&mut areas, &heights, &mut l_stack, &mut r_stack, height, i);
            }
            r_stack.push(i);
            l_stack.push(i);
            prev = height;
        }
        Self::update_area(&mut areas, &heights, &mut l_stack, &mut r_stack, 0, heights.len());
        *areas.iter().max().unwrap()
    }

    pub fn maximal_rectangle(matrix: Vec<Vec<char>>) -> i32 {
        let mut heights: Box<[i32]> = vec![0; matrix[0].len()].into_boxed_slice();
        let mut max_area = 0;
        for row in matrix {
            for (i, &col) in row.iter().enumerate() {
                if col == '1' {
                    heights[i] += 1;
                }
                else {
                    heights[i] = 0;
                }
            }
            max_area = max(max_area, Self::largest_rectangle_area(&heights));
        }
        max_area
    }
}