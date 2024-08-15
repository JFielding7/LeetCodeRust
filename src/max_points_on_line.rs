use std::cmp::max;
use std::collections::HashMap;

pub struct Solution {}

impl Solution {
    pub fn gcd(mut a: i32, mut b: i32) -> i32 {
        let b_neg = b < 0;
        while b != 0 {
            let t = a;
            a = b;
            b = t % b;
        }
        if b_neg != (a < 0) { -a } else { a }
    }

    pub fn max_points(points: Vec<Vec<i32>>) -> i32 {
        let mut max_collinear_points = 0;
        for (i, point0) in (1..points.len()).zip(&points[1..]) {
            let x0 = point0[0];
            let y0 = point0[1];
            let mut collinear_points: HashMap<(i32, i32, i32, i32), i32> = HashMap::new();
            for point1 in &points[..i] {
                let x1 = point1[0];
                let y1 = point1[1];
                let entry = if x0 == x1 {
                    (x0, 1, 1, 0)
                }
                else {
                    let m_gcd = Self::gcd(y0 - y1, x0 - x1);
                    let dy = (y0 - y1) / m_gcd;
                    let dx = (x0 - x1) / m_gcd;
                    let numer = y1 * dx - dy * x1;
                    let y_gcd = Self::gcd(numer, dx);
                    (numer / y_gcd, dx / y_gcd, dy, dx)
                };
                let count = collinear_points.entry(entry).or_insert(1);
                *count += 1;
                max_collinear_points = max(max_collinear_points, *count);
            }
        }
        max_collinear_points
    }
}
