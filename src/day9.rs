use std::str::FromStr;

use hashbrown::HashSet;

struct Rope {
    knots: Vec<(isize, isize)>,
}

#[derive(Debug, Copy, Clone)]
enum Movement {
    Up,
    Down,
    Right,
    Left,
}

impl FromStr for Movement {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let m = s.split_ascii_whitespace().next().unwrap();
        match m {
            "R" => Ok(Self::Right),
            "U" => Ok(Self::Up),
            "D" => Ok(Self::Down),
            "L" => Ok(Self::Left),
            _ => Err(()),
        }
    }
}

impl Rope {
    fn new() -> Self {
        Self {
            knots: vec![(0, 0); 11],
        }
    }
    fn make_move(&mut self, movement: Movement) {
        match movement {
            Movement::Up => self.knots[0].1 += 1,
            Movement::Down => self.knots[0].1 -= 1,
            Movement::Right => self.knots[0].0 += 1,
            Movement::Left => self.knots[0].0 -= 1,
        }

        let mut previous = self.knots[0];
        for knot in self.knots.iter_mut().skip(1) {
            if (previous.1 - knot.1).abs() > 1 || (previous.0 - knot.0).abs() > 1 {
                match (previous.0.cmp(&knot.0), previous.1.cmp(&knot.1)) {
                    (std::cmp::Ordering::Equal, std::cmp::Ordering::Equal) => (),
                    (std::cmp::Ordering::Equal, std::cmp::Ordering::Greater) => knot.1 += 1,
                    (std::cmp::Ordering::Equal, std::cmp::Ordering::Less) => knot.1 -= 1,
                    (std::cmp::Ordering::Less, std::cmp::Ordering::Equal) => knot.0 -= 1,
                    (std::cmp::Ordering::Greater, std::cmp::Ordering::Equal) => knot.0 += 1,
                    (std::cmp::Ordering::Less, std::cmp::Ordering::Less) => {
                        knot.1 -= 1;
                        knot.0 -= 1;
                    }
                    (std::cmp::Ordering::Greater, std::cmp::Ordering::Greater) => {
                        knot.1 += 1;
                        knot.0 += 1;
                    }
                    (std::cmp::Ordering::Less, std::cmp::Ordering::Greater) => {
                        knot.1 += 1;
                        knot.0 -= 1;
                    }

                    (std::cmp::Ordering::Greater, std::cmp::Ordering::Less) => {
                        knot.1 -= 1;
                        knot.0 += 1;
                    }
                }
            }
            previous = *knot;
        }
    }
}

pub fn main() {
    let moves: Vec<(Movement, usize)> = include_str!("../data/day_2022_9.data")
        .lines()
        .map(|l| l.split_once(' ').unwrap())
        .map(|(left, right)| (left.parse().unwrap(), right.parse().unwrap()))
        .collect();
    let mut knot = Rope::new();
    let mut part1 = HashSet::new();
    let mut part2 = HashSet::new();

    for (mvt, times) in moves {
        for _ in 0..times {
            knot.make_move(mvt);
            part1.insert(knot.knots[1]);
            part2.insert(knot.knots[9]);
        }
    }

    println!("Part 1: {}", part1.len());
    println!("Part 2: {}", part2.len());
}
