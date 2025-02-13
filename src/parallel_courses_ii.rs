use std::cmp::max;
use std::collections::HashMap;

pub struct Solution {}

impl Solution {
    fn total_time(course: i32, graph: &HashMap<i32, Vec<i32>>, totals: &mut Vec<i32>, times: &Vec<i32>) -> i32 {
        let course_index = course as usize;
        if totals[course_index] != -1 {
            return totals[course_index];
        }

        let total = if let Some(next) = graph.get(&course) {
            let mut max_time = 0;
            for &next_course in next {
                max_time = max(max_time, Self::total_time(next_course, graph, totals, times));
            }
            max_time + times[course_index]
        } else {
            times[course_index]
        };

        totals[course_index] = total;
        total
    }

    pub fn minimum_time(n: i32, relations: Vec<Vec<i32>>, times: Vec<i32>) -> i32 {
        let mut graph = HashMap::new();
        for edge in relations {
            graph.entry(edge[0] - 1).or_insert(Vec::new()).push(edge[1] - 1);
        }

        let mut totals = vec![-1; n as usize];
        (0..n).fold(0, |max_time, course| max(max_time, Self::total_time(course, &graph, &mut totals, &times)))
    }
}