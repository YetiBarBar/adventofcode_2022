use nom::{
    bytes::complete::tag,
    character::complete::{char, newline, u32},
    combinator::{all_consuming, map},
    multi::separated_list0,
    sequence::{delimited, preceded, separated_pair, tuple},
    IResult,
};

#[derive(Debug)]
struct BluePrint {
    id: u32,
    ore_robot_ore_cost: u32,
    clay_robot_ore_cost: u32,
    obsidian_robot_ore_cost: u32,
    obsidian_robot_clay_cost: u32,
    geode_robot_ore_cost: u32,
    geode_robot_obsidian_cost: u32,
}

fn blueprint(data: &str) -> IResult<&str, BluePrint> {
    map(
        tuple((
            preceded(tag("Blueprint "), u32),
            preceded(tag(": Each ore robot costs "), u32),
            preceded(tag(" ore. Each clay robot costs "), u32),
            preceded(tag(" ore. Each obsidian robot costs "), u32),
            preceded(tag("ore and "), u32),
            preceded(tag(" clay. Each geode robot costs "), u32),
            delimited(tag(" ore and "), u32, tag(" obsidian.")),
        )),
        |(
            id,
            ore_robot_ore_cost,
            clay_robot_ore_cost,
            obsidian_robot_ore_cost,
            obsidian_robot_clay_cost,
            geode_robot_ore_cost,
            geode_robot_obsidian_cost,
        )| BluePrint {
            id,
            ore_robot_ore_cost,
            clay_robot_ore_cost,
            obsidian_robot_ore_cost,
            obsidian_robot_clay_cost,
            geode_robot_ore_cost,
            geode_robot_obsidian_cost,
        },
    )(data)
}

pub fn main() {
    let raw = include_str!("../data/day_2022_19.data").trim();
    let blueprints = all_consuming(separated_list0(newline, blueprint))(raw)
        .unwrap()
        .1;

    println!("{:?}", blueprints);
}
