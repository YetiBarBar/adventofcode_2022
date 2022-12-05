use nom::{
    bytes::complete::tag,
    character::complete::{char, u64},
    combinator::map,
    multi::separated_list0,
    sequence::tuple,
    IResult,
};
use std::collections::HashMap;

fn moves(data: &str) -> IResult<&str, Vec<(u64, u64, u64)>> {
    separated_list0(
        char('\n'),
        map(
            tuple((tag("move "), u64, tag(" from "), u64, tag(" to "), u64)),
            |(_, times, _, start, _, end)| (times, start, end),
        ),
    )(data)
}

pub fn main() {
    let data = include_str!("../data/day_2022_5.data")
        .split_once("\n\n")
        .unwrap()
        .1;

    let moves = moves(data).unwrap().1;

    let stacks = (1_u64..)
        .zip([
            "DZTH", "QRWYGCS", "PBFQNRCH", "LCNFHZ", "GLFQS", "VPWZBRCS", "ZFJ", "DLVZRHQ",
            "BHGNFZLD",
        ])
        .map(|(idx, chrs)| (idx, chrs.chars().collect::<Vec<char>>()))
        .collect::<HashMap<u64, Vec<char>>>();

    let mut part1_stacks = stacks.clone();
    for instr in &moves {
        for _ in 0..instr.0 {
            let value = part1_stacks.get_mut(&instr.1).unwrap().pop().unwrap();
            part1_stacks.get_mut(&instr.2).unwrap().push(value);
        }
    }

    println!("Part 1: {}", hmap_to_part_result(&part1_stacks));

    let mut part2_stacks = stacks.clone();
    for instr in &moves {
        let mut vals = vec![];
        for _ in 0..instr.0 {
            let value = part2_stacks.get_mut(&instr.1).unwrap().pop().unwrap();
            vals.push(value);
        }
        part2_stacks
            .get_mut(&instr.2)
            .unwrap()
            .extend(vals.iter().rev());
    }

    println!("Part 2: {}", hmap_to_part_result(&part2_stacks));
}

fn hmap_to_part_result(hmap: &HashMap<u64, Vec<char>>) -> String {
    (1..=9)
        .map(|idx| hmap.get(&idx).unwrap().last().unwrap())
        .collect()
}
