use std::collections::VecDeque;

struct SignalHandler<'a> {
    values: VecDeque<&'a str>,
    current_val: isize,
    to_hold: usize,
    next: isize,
}

impl<'a> SignalHandler<'a> {
    fn new(data: &'a str) -> Self {
        SignalHandler {
            values: data.split('\n').collect(),
            current_val: 1,
            to_hold: 0,
            next: 0,
        }
    }
}

impl<'a> Iterator for SignalHandler<'a> {
    type Item = isize;

    fn next(&mut self) -> Option<Self::Item> {
        if self.to_hold == 0 {
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
        } else {
            self.to_hold -= 1;
            let ret_val = self.current_val;
            if self.to_hold == 0 {
                self.current_val += self.next;
                self.next = 0;
            }
            Some(ret_val)
        }
    }
}

pub fn main() {
    let step1_iter = SignalHandler::new(include_str!("../data/day_2022_10.data"));
    let vals: Vec<_> = (1_usize..221).zip(step1_iter).collect();
    let res: isize = vals
        .iter()
        .filter(|v| [20, 60, 100, 140, 180, 220].contains(&v.0))
        .map(|&(x, y)| isize::try_from(x).unwrap() * y)
        .sum();

    println!("Part 1: {}", res);

    let step2_iter = SignalHandler::new(include_str!("../data/day_2022_10.data"));
    let res = step2_iter
        .take(240)
        .enumerate()
        .map(|(x, y)| {
            if (isize::try_from(x % 40).unwrap() - y).abs() < 2 {
                '\u{2588}'
            } else {
                ' '
            }
        })
        .collect::<Vec<char>>();
    for line in res.chunks(40) {
        println!("{}", line.iter().take(39).collect::<String>());
    }
}
