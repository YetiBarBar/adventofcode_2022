use hashbrown::HashMap;
use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{alpha0, alpha1, char, newline, space1, u64},
    combinator::{all_consuming, map},
    multi::{separated_list0, separated_list1},
    sequence::{delimited, preceded},
    IResult, Parser,
};

#[derive(Debug)]
struct Valve<'a> {
    name: &'a str,
    rate: u64,
    connected_valve: Vec<&'a str>,
}

fn valve(data: &str) -> IResult<&str, Valve> {
    map(
        (
            delimited(tag("Valve "), alpha0, space1),
            delimited(tag("has flow rate="), u64, char(';')),
            alt((
                preceded(
                    tag(" tunnels lead to valves "),
                    separated_list1(tag(", "), alpha1),
                ),
                preceded(
                    tag(" tunnel leads to valve "),
                    separated_list0(tag(", "), alpha1),
                ),
            )),
        ),
        |(name, rate, connected_valve)| Valve {
            name,
            rate,
            connected_valve,
        },
    )
    .parse(data)
}

pub fn main() {
    let valve = all_consuming(separated_list1(newline, valve))
        .parse(include_str!("../data/day_2022_16_s.data").trim())
        .unwrap()
        .1;

    let valve: HashMap<&str, Valve> = valve.into_iter().map(|v| (v.name, v)).collect();

    let mut cache = HashMap::new();
    let part1 = dfs("AA", &valve, &[], 29, &mut cache);
    println!("Part 1: {part1}");
}

fn dfs(
    current: &str,
    world: &HashMap<&str, Valve>,
    used: &[&str],
    level: u64,
    cache: &mut HashMap<(String, u64), u64>,
) -> u64 {
    // println!("Level: {level} - Valve: {current}");
    if level == 0 {
        return 0;
    }
    /*    if let Some(cached) = cache.get(&(current.to_string(), level)) {
        return *cached;
    } */
    let curr = world.get(&current).unwrap();
    let res = if level > 1 && !used.contains(&current) {
        let mut new_used = used.to_vec();
        new_used.push(current);
        (curr.rate * (level - 1)
            + curr
                .connected_valve
                .iter()
                .map(|&valve_name| dfs(valve_name, world, &new_used, level - 2, cache))
                .max()
                .unwrap())
        .max(
            curr.connected_valve
                .iter()
                .map(|&valve_name| dfs(valve_name, world, used, level - 1, cache))
                .max()
                .unwrap(),
        )
    } else {
        curr.connected_valve
            .iter()
            .map(|&valve_name| dfs(valve_name, world, used, level - 1, cache))
            .max()
            .unwrap()
    };
    cache.insert((current.to_string(), level), res);
    // println!("{current} at level {level} produces : {res}");
    res
}
