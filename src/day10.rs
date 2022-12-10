use std::collections::VecDeque;

struct SignalHandler {
    // indexes: Vec<usize>,
    values: VecDeque<String>,
    current_val: isize,
    to_hold: usize,
    next: isize,
}

impl SignalHandler {
    fn new(data: &str) -> Self {
        SignalHandler {
            values: data.split('\n').map(|s| s.into()).collect(),
            current_val: 1,
            to_hold: 0,
            next: 0,
        }
    }
}

impl Iterator for SignalHandler {
    type Item = isize;

    fn next(&mut self) -> Option<Self::Item> {
        if self.to_hold != 0 {
            self.to_hold -= 1;
            let ret_val = self.current_val;
            if self.to_hold == 0 {
                self.current_val += self.next;
                self.next = 0;
            }
            Some(ret_val)
        } else {
            let popped = self.values.pop_front().unwrap();
            if popped == "noop" {
                Some(self.current_val)
            } else {
                self.to_hold = 1;
                self.next += popped
                    .split_ascii_whitespace()
                    .nth(1)
                    .map(str::parse::<isize>)
                    .unwrap()
                    .unwrap();
                Some(self.current_val)
            }
        }
    }
}

pub fn main() {
    let step1_iter = SignalHandler::new(include_str!("../data/day_2022_10.data"));
    let vals: Vec<_> = (1_usize..221).zip(step1_iter).collect();
    let res: isize = vals
        .iter()
        .filter(|v| [20, 60, 100, 140, 180, 220].contains(&v.0))
        .map(|(x, y)| *x as isize * *y)
        .sum();

    println!("Part 1: {}", res);

    let step2_iter = SignalHandler::new(include_str!("../data/day_2022_10.data"));
    let vals: Vec<_> = (1_usize..=240).zip(step2_iter).collect();
    let res = vals
        .iter()
        .map(|(x, y)| {
            if (((x % 40) as isize) - y - 1).abs() < 2 {
                '#'
            } else {
                ' '
            }
        })
        .collect::<Vec<char>>();
    for line in res.chunks(40) {
        println!("{}", line.iter().take(39).collect::<String>());
    }
}
