pub struct Solution;

impl Solution {
    pub fn min_distance(word0: String, word1: String) -> i32 {
        let lengths = word1.len() + 1;
        let mut prev_distances: Vec<i32> = (0..lengths as i32).collect();

        for (i, a) in word0.chars().enumerate() {
            let mut distances = Vec::with_capacity(lengths);
            distances.push(i as i32 + 1);

            for (j, b) in word1.chars().enumerate() {
                distances.push(if a == b {
                    prev_distances[j]
                } else {
                    1 + prev_distances[j].min(prev_distances[j + 1]).min(distances[j])
                });
            }

            prev_distances = distances;
        }

        *prev_distances.last().unwrap()
    }
}