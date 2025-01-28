use nom::{
    bytes::complete::tag,
    character::complete::{char, u16},
    combinator::map,
    multi::separated_list0,
    IResult, Parser,
};

fn parse_moves(data: &str) -> IResult<&str, Vec<(usize, usize, usize)>> {
    separated_list0(
        char('\n'),
        map(
            (tag("move "), u16, tag(" from "), u16, tag(" to "), u16),
            |(_, times, _, start, _, end)| (times.into(), start.into(), end.into()),
        ),
    )
    .parse(data)
}

pub fn main() {
    let data = include_str!("../data/day_2022_5.data")
        .split_once("\n\n")
        .unwrap()
        .1;

    let moves = parse_moves(data).unwrap().1;

    let stacks = [
        "DZTH", "QRWYGCS", "PBFQNRCH", "LCNFHZ", "GLFQS", "VPWZBRCS", "ZFJ", "DLVZRHQ", "BHGNFZLD",
    ]
    .iter()
    .map(|chrs| chrs.chars().collect::<Vec<char>>())
    .collect::<Vec<Vec<char>>>();

    let mut part1_stacks = stacks.clone();
    for instr in &moves {
        let curent_stack = part1_stacks.get_mut(instr.1 - 1).unwrap();
        let vals = curent_stack.split_off(curent_stack.len() - instr.0);
        part1_stacks
            .get_mut(instr.2 - 1)
            .unwrap()
            .extend(vals.iter().rev());
    }

    println!("Part 1: {}", to_part_result(&part1_stacks));

    let mut part2_stacks = stacks;
    for instr in &moves {
        let curent_stack = part2_stacks.get_mut(instr.1 - 1).unwrap();
        let vals = curent_stack.split_off(curent_stack.len() - instr.0);
        part2_stacks.get_mut(instr.2 - 1).unwrap().extend(vals);
    }

    println!("Part 2: {}", to_part_result(&part2_stacks));
}

fn to_part_result(stacks: &[Vec<char>]) -> String {
    stacks.iter().map(|stack| stack.last().unwrap()).collect()
}
