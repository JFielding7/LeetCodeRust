pub struct Solution {}

impl Solution {
    fn sort_reverse(vec: &mut Vec<i32>) -> () {
        vec.sort_unstable_by(|a, b| b.cmp(a));
    }

    pub fn minimum_cost(m: i32, n: i32, mut horizontal_cut: Vec<i32>, mut vertical_cut: Vec<i32>) -> i64 {
        Self::sort_reverse(&mut horizontal_cut);
        Self::sort_reverse(&mut vertical_cut);

        let h_max = (m - 1) as usize;
        let v_max = (n - 1) as usize;
        let mut cost = 0;
        let mut i = 0;
        let mut j = 0;
        let mut h_cuts = 1;
        let mut v_cuts = 1;

        while i < h_max && j < v_max {
            if horizontal_cut[i] > vertical_cut[j] {
                cost += horizontal_cut[i] as i64 * h_cuts;
                v_cuts += 1;
                i += 1;
            } else {
                cost += vertical_cut[j] as i64 * v_cuts;
                h_cuts += 1;
                j += 1;
            }
        }

        while i < h_max {
            cost += horizontal_cut[i] as i64 * h_cuts;
            i += 1;
        }
        while j < v_max {
            cost += vertical_cut[j] as i64 * v_cuts;
            j += 1;
        }

        cost
    }
}