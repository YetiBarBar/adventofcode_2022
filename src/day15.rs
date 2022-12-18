use hashbrown::HashSet;
use nom::{
    bytes::complete::tag,
    character::complete::{i64, newline},
    combinator::map,
    multi::separated_list0,
    sequence::{preceded, separated_pair, tuple},
    IResult,
};

#[derive(Debug)]
struct Sensor {
    position: (i64, i64),
    nearest_beacon: (i64, i64),
    manhattan_coverage: i64,
}

impl Sensor {
    fn coverage_line(&self, line_number: i64) -> HashSet<i64> {
        // On a single line
        let mut delta = (self.position.1 - line_number).abs();

        if delta > self.manhattan_coverage {
            HashSet::new()
        } else {
            delta = self.manhattan_coverage - delta;
            // println!("{:?}: delta: {}", self, delta);
            let start = self.position.0 - delta;
            let end = self.position.0 + delta;
            // println!("{:?}: start: {}, end: {}", self, start, end);
            if self.nearest_beacon.1 == line_number {
                (start..end)
                    .filter(|x| x != &self.nearest_beacon.0)
                    .collect()
            } else {
                (start..=end).collect()
            }
        }
    }
    fn coverage_line_part2(&self, line_number: i64, occupied: &mut [bool]) {
        // On a single line
        let mut delta = (self.position.1 - line_number).abs();

        if delta > self.manhattan_coverage {
            ()
        } else {
            delta = self.manhattan_coverage - delta;
            let start = self.position.0 - delta;
            let end = self.position.0 + delta;

            for idx in start.max(0)..end.min(4_000_000) {
                occupied[idx as usize] = true;
            }
        }
    }
}

fn position(data: &str) -> IResult<&str, (i64, i64)> {
    separated_pair(
        preceded(tag("x="), i64),
        tag(", "),
        preceded(tag("y="), i64),
    )(data)
}

fn sensor(data: &str) -> IResult<&str, Sensor> {
    map(
        tuple((
            tag("Sensor at "),
            position,
            tag(": closest beacon is at "),
            position,
        )),
        |(_, position, _, nearest_beacon)| Sensor {
            position,
            nearest_beacon,
            manhattan_coverage: (nearest_beacon.0 - position.0).abs()
                + (nearest_beacon.1 - position.1).abs(),
        },
    )(data)
}

pub fn main() {
    let raw = include_str!("../data/day_2022_15.data");
    let sensors = separated_list0(newline, sensor)(raw).unwrap().1;

    let mut occupied: HashSet<i64> = HashSet::new();

    for sensor in &sensors {
        occupied.extend(sensor.coverage_line(2000000).iter());
    }

    println!("Part 1: {}", occupied.len());
    println!("Warning! More than 1 hour run...");
    for y in 0..4_000_000 {
        let mut occupied = vec![false; 4_000_000];

        for sensor in &sensors {
            sensor.coverage_line_part2(y, &mut occupied);
        }

        if occupied.contains(&false) {
            let x = occupied
                .iter()
                .enumerate()
                .find(|(_, x)| **x == false)
                .map(|(x, _)| x)
                .unwrap();
            println!("Part 2: {}", x as u128 * 4000000 + y as u128);
            break;
        }
    }
}
