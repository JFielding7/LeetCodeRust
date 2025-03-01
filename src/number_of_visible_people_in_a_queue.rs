pub struct Solution;

impl Solution {
    pub fn can_see_persons_count(heights: Vec<i32>) -> Vec<i32> {
        let people = heights.len();
        let mut visible = vec![0; people];
        let mut visible_stack = Vec::with_capacity(people);

        for (i, &height) in heights.iter().rev().enumerate() {
            let len = visible_stack.len();

            let mut j = len;
            while let Some(&h) = visible_stack.get(j - 1) {
                if h > height {
                    break;
                }
                j -= 1;
            }

            visible_stack.truncate(j);
            visible[people - 1 - i] = (len - j) as i32 + (j != 0) as i32;
            visible_stack.push(height);
        }

        visible
    }
}