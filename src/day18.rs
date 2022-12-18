use nom::{
    character::complete::{char, newline, u32},
    combinator::{all_consuming, map},
    multi::separated_list0,
    sequence::tuple,
    IResult,
};

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
struct LavaBlock {
    x: u32,
    y: u32,
    z: u32,
}

impl LavaBlock {
    fn free_neighbours(&self, world: &[LavaBlock]) -> usize {
        let mut free = 6;
        if self.x > 0 {
            if world.contains(&LavaBlock {
                x: self.x - 1,
                ..*self
            }) {
                free -= 1;
            }
        }
        if self.y > 0 {
            if world.contains(&LavaBlock {
                y: self.y - 1,
                ..*self
            }) {
                free -= 1;
            }
        }
        if self.z > 0 {
            if world.contains(&LavaBlock {
                z: self.z - 1,
                ..*self
            }) {
                free -= 1;
            }
        }
        if world.contains(&LavaBlock {
            x: self.x + 1,
            ..*self
        }) {
            free -= 1;
        }
        if world.contains(&LavaBlock {
            y: self.y + 1,
            ..*self
        }) {
            free -= 1;
        }
        if world.contains(&LavaBlock {
            z: self.z + 1,
            ..*self
        }) {
            free -= 1;
        }
        free
    }
}

fn lava_block(data: &str) -> IResult<&str, LavaBlock> {
    map(
        tuple((u32, char(','), u32, char(','), u32)),
        |(x, _, y, _, z)| LavaBlock { x, y, z },
    )(data)
}

pub fn main() {
    let raw = include_str!("../data/day_2022_18.data").trim();
    let blocks = all_consuming(separated_list0(newline, lava_block))(raw)
        .unwrap()
        .1;

    println!("{}", part1(&blocks));
}

fn part1(world: &[LavaBlock]) -> usize {
    world.iter().map(|block| block.free_neighbours(world)).sum()
}
