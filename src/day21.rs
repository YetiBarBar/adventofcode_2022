use std::collections::VecDeque;

use hashbrown::HashMap;
#[derive(Clone, Copy)]
enum Operator {
    Add,
    Sub,
    Mul,
    Div,
}

impl Operator {
    fn get_value(self, v1: i64, v2: i64) -> i64 {
        match self {
            Operator::Add => v1 + v2,
            Operator::Sub => v1 - v2,
            Operator::Mul => v1 * v2,
            Operator::Div => v1 / v2,
        }
    }

    fn reverted_op(self) -> Operator {
        match self {
            Operator::Add => Operator::Sub,
            Operator::Sub => Operator::Add,
            Operator::Mul => Operator::Div,
            Operator::Div => Operator::Mul,
        }
    }
}

enum Monkey {
    Const(i64),
    Expression(usize, usize, Operator),
}

impl Monkey {
    fn get_value(&self, monkeys: &[Monkey]) -> i64 {
        match self {
            Monkey::Const(c) => *c,
            Monkey::Expression(i1, i2, op) => op.get_value(
                monkeys[*i1].get_value(monkeys),
                monkeys[*i2].get_value(monkeys),
            ),
        }
    }

    fn get_expected_left(&self, expected_result: i64, monkeys: &[Monkey]) -> i64 {
        match self {
            Monkey::Const(_) => panic!(),
            Monkey::Expression(_, i2, op) => op
                .reverted_op()
                .get_value(expected_result, monkeys[*i2].get_value(monkeys)),
        }
    }

    fn get_expected_right(&self, expected_result: i64, monkeys: &[Monkey]) -> i64 {
        match self {
            Monkey::Const(_) => panic!(),
            Monkey::Expression(i1, _, op) => {
                let v1 = monkeys[*i1].get_value(monkeys);
                match op {
                    Operator::Add | Operator::Mul => {
                        op.reverted_op().get_value(expected_result, v1)
                    }
                    Operator::Sub | Operator::Div => op.get_value(v1, expected_result),
                }
            }
        }
    }
}

fn parse(data: &str) -> (Vec<Monkey>, usize, usize) {
    let id_to_index: HashMap<&str, usize> = data
        .lines()
        .enumerate()
        .map(|(i, line)| (line.split_once(':').map(|(id, _)| id).unwrap(), i))
        .collect();

    (
        data.lines()
            .map(|line| {
                let (_, definition) = line.split_once(": ").unwrap();
                if let Ok(c) = definition.parse::<i64>() {
                    Monkey::Const(c)
                } else {
                    {
                        let mut parts = definition.split(' ');
                        let i1 = id_to_index.get(parts.next().unwrap()).unwrap();
                        let operator = match parts.next().unwrap() {
                            "+" => Operator::Add,
                            "-" => Operator::Sub,
                            "*" => Operator::Mul,
                            "/" => Operator::Div,
                            _ => panic!("bad input"),
                        };
                        let i2 = id_to_index.get(parts.next().unwrap()).unwrap();
                        Monkey::Expression(*i1, *i2, operator)
                    }
                }
            })
            .collect(),
        *id_to_index.get("root").unwrap(),
        *id_to_index.get("humn").unwrap(),
    )
}

pub fn main() {
    let input = include_str!("../data/day_2022_21.data");

    // Part 1
    let (monkeys, root_index, _) = parse(input);
    println!("Part 1: {}", monkeys[root_index].get_value(&monkeys));

    // Part 2
    let (monkeys, root_index, humn_index) = parse(input);
    let mut q: VecDeque<(usize, i64)> = VecDeque::new(); //(index, expected value)
    if let Monkey::Expression(i1, i2, _) = monkeys[root_index] {
        q.push_back((i2, monkeys[i1].get_value(&monkeys)));
        q.push_back((i1, monkeys[i2].get_value(&monkeys)));
    }

    while let Some((i, expected)) = q.pop_front() {
        if i == humn_index {
            println!("Part 2: {expected}");
            break;
        }

        if let Monkey::Expression(i1, i2, _) = monkeys[i] {
            q.push_back((i1, monkeys[i].get_expected_left(expected, &monkeys)));
            q.push_back((i2, monkeys[i].get_expected_right(expected, &monkeys)));
        }
    }
}
