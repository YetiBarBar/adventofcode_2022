use std::fmt::Display;

use hashbrown::HashMap;
use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{newline, space0, u64, u8},
    combinator::{map, value},
    multi::separated_list0,
    sequence::{delimited, pair, preceded, terminated},
    IResult, Parser,
};

#[derive(Debug, Clone, Copy, Hash)]
enum Operation {
    Pow,
    Mult(u64),
    Add(u64),
}

#[derive(Debug, Clone, Hash)]
struct Monkey {
    id: usize,
    items: Vec<u64>,
    operation: Operation,
    test_modulo: u64,
    dest_true: usize,
    dest_false: usize,
    inspected: usize,
}

impl Display for Monkey {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Monkey : {} => {:?}", self.id, self.items)
    }
}

fn parse_operation(data: &str) -> IResult<&str, Operation> {
    alt((
        map(
            preceded(pair(space0, tag("Operation: new = old + ")), u64),
            Operation::Add,
        ),
        map(
            preceded(pair(space0, tag("Operation: new = old * ")), u64),
            Operation::Mult,
        ),
        value(
            Operation::Pow,
            pair(space0, tag("Operation: new = old * old")),
        ),
    ))
    .parse(data)
}

fn parse_monkey(data: &str) -> IResult<&str, Monkey> {
    map(
        (
            delimited(tag("Monkey "), u8, tag(":\n")),
            delimited(
                pair(space0, tag("Starting items: ")),
                separated_list0(tag(", "), u64),
                newline,
            ),
            terminated(parse_operation, newline),
            delimited(pair(space0, tag("Test: divisible by ")), u64, newline),
            delimited(pair(space0, tag("If true: throw to monkey ")), u8, newline),
            delimited(pair(space0, tag("If false: throw to monkey ")), u8, newline),
        ),
        |(id, items, operation, test_modulo, dest_true, dest_false)| Monkey {
            id: id.into(),
            items,
            operation,
            test_modulo,
            dest_true: dest_true.into(),
            dest_false: dest_false.into(),
            inspected: 0_usize,
        },
    )
    .parse(data)
}

pub fn main() {
    let monkeys_parsed = separated_list0(newline, parse_monkey)
        .parse(include_str!("../data/day_2022_11.data"))
        .unwrap()
        .1
        .into_iter()
        .map(|monkey| (monkey.id, monkey))
        .collect::<HashMap<usize, Monkey>>();

    // Regarding data, we know we must have 8 Monkey
    // so we check here
    // we could have checked that the IResult "remaining string" is "\n"
    assert!(monkeys_parsed.len() == 8);

    let mut monkeys = monkeys_parsed.clone();
    for _ in 0..20 {
        part1_turn(&mut monkeys);
    }

    let res = result_value(&monkeys);
    println!("Part 1: {res}");

    let mut monkeys = monkeys_parsed;
    for _ in 0..10_000 {
        part2_turn(&mut monkeys);
    }

    println!("Part 2: {}", result_value(&monkeys));
}

fn result_value(monkeys: &HashMap<usize, Monkey>) -> usize {
    let mut values: Vec<_> = monkeys.values().map(|m| m.inspected).collect();
    values.sort_unstable();
    values.into_iter().rev().take(2).product::<usize>()
}

fn monkey_turn(
    idx: usize,
    monkeys: &mut HashMap<usize, Monkey>,
    update_policy: impl Fn(u64) -> u64,
) {
    let current_monkey = monkeys.get(&idx).unwrap().clone();
    let items = current_monkey.items.clone();
    for item in &items {
        let worry_val = update_policy(match current_monkey.operation {
            Operation::Pow => item * item,
            Operation::Mult(v) => item * v,
            Operation::Add(v) => item + v,
        });

        monkeys
            .get_mut(if worry_val % current_monkey.test_modulo == 0 {
                &current_monkey.dest_true
            } else {
                &current_monkey.dest_false
            })
            .unwrap()
            .items
            .push(worry_val);
    }

    monkeys.get_mut(&idx).unwrap().items.clear();
    monkeys.get_mut(&idx).unwrap().inspected += items.len();
}

fn part1_play_monkey_turn(idx: usize, monkeys: &mut HashMap<usize, Monkey>) {
    monkey_turn(idx, monkeys, |x| x / 3);
}

fn part2_play_monkey_turn(idx: usize, monkeys: &mut HashMap<usize, Monkey>, lcm: u64) {
    monkey_turn(idx, monkeys, |x| x % lcm);
}

fn part1_turn(monkeys: &mut HashMap<usize, Monkey>) {
    for idx in 0..monkeys.len() {
        part1_play_monkey_turn(idx, monkeys);
    }
}

fn part2_turn(monkeys: &mut HashMap<usize, Monkey>) {
    let lcm = monkeys
        .values()
        .map(|m| m.test_modulo)
        .reduce(num::integer::lcm)
        .unwrap_or_default();
    for idx in 0..monkeys.len() {
        part2_play_monkey_turn(idx, monkeys, lcm);
    }
}
