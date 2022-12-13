use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{char, newline, u32},
    combinator::map,
    multi::{separated_list0, separated_list1},
    sequence::{delimited, separated_pair},
    IResult,
};

fn parse_packet(data: &str) -> IResult<&str, Packet> {
    alt((
        map(u32, Packet::Integer),
        map(
            delimited(
                char('['),
                separated_list0(char(','), parse_packet),
                char(']'),
            ),
            Packet::List,
        ),
    ))(data)
}

fn parse_pairs(data: &str) -> IResult<&str, [Packet; 2]> {
    map(
        separated_pair(parse_packet, newline, parse_packet),
        |(p1, p2)| [p1, p2],
    )(data)
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum Packet {
    Integer(u32),
    List(Vec<Packet>),
}

impl PartialOrd for Packet {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        match (self, other) {
            (Packet::Integer(l), Packet::Integer(r)) => l.partial_cmp(r),
            (Packet::List(l), Packet::List(r)) => l.partial_cmp(r),
            (Packet::Integer(_), Packet::List(_)) => {
                Packet::List(vec![self.clone()]).partial_cmp(other)
            }
            (Packet::List(_), Packet::Integer(_)) => {
                self.partial_cmp(&Packet::List(vec![other.clone()]))
            }
        }
    }
}

impl Ord for Packet {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.partial_cmp(other).unwrap()
    }
}

pub fn main() {
    let packets =
        separated_list1(tag("\n\n"), parse_pairs)(include_str!("../data/day_2022_13.data"));
    let packets = packets.unwrap().1;

    let part1 = packets
        .iter()
        .zip(1..)
        .filter(|(pair, _)| pair[0] < pair[1])
        .map(|(_, v)| v)
        .sum::<u32>();

    println!("Part 1: {}", part1);

    let mut packets = packets
        .into_iter()
        .flat_map(std::iter::IntoIterator::into_iter)
        .collect::<Vec<_>>();

    let v2 = parse_packet("[[2]]").unwrap().1;
    let v6 = parse_packet("[[6]]").unwrap().1;

    packets.push(v2.clone());
    packets.push(v6.clone());
    packets.sort();

    let p2 = packets.iter().zip(1..).find(|(v, _)| v >= &&v2).unwrap().1;
    let p6 = packets.iter().zip(1..).find(|(v, _)| v >= &&v6).unwrap().1;

    println!("Part 2: {}", p2 * p6);
}
