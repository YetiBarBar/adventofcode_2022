use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{alpha0, alpha1, char, newline, space1, u64},
    combinator::map,
    multi::{separated_list0, separated_list1},
    sequence::{delimited, preceded, tuple},
    IResult,
};

#[derive(Debug)]
struct Valve<'a> {
    name: &'a str,
    rate: u64,
    connected_valve: Vec<&'a str>,
}

fn valve(data: &str) -> IResult<&str, Valve> {
    map(
        tuple((
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
        )),
        |(name, rate, connected_valve)| Valve {
            name,
            rate,
            connected_valve,
        },
    )(data)
}

const DURATION: usize = 30;

pub fn main() {
    let valve = separated_list1(newline, valve)(include_str!("../data/day_2022_16.data")).unwrap();

    println!("{:#?}", valve.1);
}
