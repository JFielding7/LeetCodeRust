use std::collections::HashMap;

struct FreqStack {
    freq_stacks: Vec<Vec<i32>>,
    frequencies: HashMap<i32, usize>
}

impl FreqStack {

    fn new() -> Self {
        Self { freq_stacks: Vec::new(), frequencies: HashMap::new() }
    }

    fn push(&mut self, val: i32) {
        let freq = self.frequencies.entry(val).or_insert(0);

        if self.freq_stacks.len() == *freq {
            self.freq_stacks.push(vec![val]);
        } else {
            self.freq_stacks[*freq].push(val);
        }

        *freq += 1;
    }

    fn pop(&mut self) -> i32 {
        let stack = self.freq_stacks.last_mut().unwrap();
        let val = stack.pop().unwrap();
        self.frequencies.entry(val).and_modify(|freq| *freq -= 1);

        if stack.len() == 0 {
            self.freq_stacks.pop();
        }

        val
    }
}
