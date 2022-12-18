use hashbrown::{HashMap, HashSet};
use nom::{
    character::complete::{char, i32, newline},
    combinator::{all_consuming, map},
    multi::separated_list0,
    sequence::tuple,
    IResult,
};

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
struct LavaBlock {
    x: i32,
    y: i32,
    z: i32,
}

static DELTAS: [(i32, i32, i32); 6] = [
    (-1, 0, 0),
    (1, 0, 0),
    (0, -1, 0),
    (0, 1, 0),
    (0, 0, -1),
    (0, 0, 1),
];

impl LavaBlock {
    fn free_neighbours(&self, world: &[LavaBlock]) -> Vec<(i32, i32, i32)> {
        DELTAS
            .iter()
            .map(|(dx, dy, dz)| (self.x + dx, self.y + dy, self.z + dz))
            .filter(|&(x, y, z)| !world.contains(&LavaBlock { x, y, z }))
            .collect()
    }

    fn free_neighbours_count(&self, world: &[LavaBlock]) -> usize {
        self.free_neighbours(world).len()
    }
}

fn lava_block(data: &str) -> IResult<&str, LavaBlock> {
    map(
        tuple((i32, char(','), i32, char(','), i32)),
        |(x, _, y, _, z)| LavaBlock { x, y, z },
    )(data)
}

pub fn main() {
    let raw = include_str!("../data/day_2022_18.data").trim();
    let blocks = all_consuming(separated_list0(newline, lava_block))(raw)
        .unwrap()
        .1;

    let part1 = part1(&blocks);
    println!("Part 1: {}", part1);
    println!("Part 2: {}", part2(&blocks));
}

fn part1(world: &[LavaBlock]) -> usize {
    world
        .iter()
        .map(|block| block.free_neighbours_count(world))
        .sum()
}

fn part2(world: &[LavaBlock]) -> usize {
    let mut cache = HashMap::new();
    world
        .iter()
        .flat_map(|block| block.free_neighbours(world).into_iter())
        .filter(|n| has_path_to_out(*n, world, &mut cache))
        .count()
}

fn adjacents(
    visited: &HashSet<(i32, i32, i32)>,
    current: &HashSet<(i32, i32, i32)>,
    marble: &[LavaBlock],
) -> HashSet<(i32, i32, i32)> {
    let mut hset = HashSet::new();

    for &pts in current {
        hset.extend(
            DELTAS
                .iter()
                .map(|(dx, dy, dz)| (pts.0 + dx, pts.1 + dy, pts.2 + dz))
                .filter(|pts| !visited.contains(pts))
                .filter(|pts| {
                    !marble.contains(&LavaBlock {
                        x: pts.0,
                        y: pts.1,
                        z: pts.2,
                    })
                }),
        );
    }
    hset
}

fn has_path_to_out(
    point: (i32, i32, i32),
    world: &[LavaBlock],
    cache: &mut HashMap<(i32, i32, i32), bool>,
) -> bool {
    if let Some(val) = cache.get(&point) {
        return *val;
    }

    let mut current = HashSet::new();
    current.insert(point);
    let mut visited = HashSet::new();
    visited.insert(point);

    while !visited.contains(&(-1, -1, -1)) {
        current = adjacents(&visited, &current, world);

        if current.is_empty() {
            cache.insert(point, false);
            return false;
        }

        for pts in &current {
            visited.insert(*pts);
        }
        if current
            .iter()
            .any(|pts| pts.0 < 0 || pts.1 < 0 || pts.2 < 0)
        {
            break;
        }
    }
    cache.insert(point, true);
    true
}
