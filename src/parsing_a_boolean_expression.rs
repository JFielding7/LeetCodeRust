pub struct Solution;

impl Solution {
    fn get_matches(expression: &String) -> Vec<usize> {
        let mut stack = Vec::new();
        let mut matches = vec![0; expression.len()];

        for (i, token) in expression.chars().enumerate()  {
            match token {
                '(' => stack.push(i),
                ')' => {
                    let prev = stack.pop().unwrap();
                    matches[prev] = i;
                }
                ',' => {
                    let prev = stack.last_mut().unwrap();
                    matches[*prev] = i;
                    *prev = i;
                }
                _ => {}
            }
        }

        matches
    }

    fn eval_literal(expression: &String, i: usize) -> bool {
        match expression.as_bytes()[i] {
            b't' => true,
            _ => false
        }
    }

    fn eval_list(expression: &String, start: usize, end: usize, matches: &Vec<usize>, short_circuit_val: bool) -> bool {
        let mut curr_start = start + 1;
        while curr_start + 1 < end {
            let curr_end = matches[curr_start];
            if Self::evaluate(expression, curr_start + 1, curr_end, matches) == short_circuit_val {
                return short_circuit_val;
            }
            curr_start = curr_end;
        }

        !short_circuit_val
    }

    fn evaluate(expression: &String, start: usize, end: usize, matches: &Vec<usize>) -> bool {
        if start + 1 == end {
            return Self::eval_literal(expression, start);
        }

        match expression.as_bytes()[start] {
            b'!' => !Self::evaluate(expression, start + 2, end - 1, matches),
            b'&' => Self::eval_list(expression, start, end, matches, false),
            b'|' => Self::eval_list(expression, start, end, matches, true),
            _ => false
        }
    }

    pub fn parse_bool_expr(expression: String) -> bool {
        Self::evaluate(&expression, 0, expression.len(), &Self::get_matches(&expression))
    }
}
