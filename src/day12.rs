use adventofcode_tooling::Matrix2D;
use hashbrown::HashSet;

pub fn main() {
    let lines: Vec<&str> = include_str!("../data/day_2022_12.data")
        .lines()
        .filter(|line| !line.is_empty())
        .collect();

    let mut matrix = Matrix2D {
        width: lines[0].len(),
        height: lines.len(),
        values: lines.iter().flat_map(|line| line.chars()).collect(),
    };

    let e_pos = matrix
        .values
        .iter()
        .enumerate()
        .find(|(_, c)| c == &&'E')
        .map(|(idx, _)| idx)
        .unwrap();

    matrix.values[e_pos] = 'z';

    let e_pos = (e_pos % matrix.width, e_pos / matrix.width);
    let s_pos = matrix
        .values
        .iter()
        .enumerate()
        .find(|(_, c)| c == &&'S')
        .map(|(idx, _)| idx)
        .unwrap();

    matrix.values[s_pos] = 'a';
    let s_pos = (s_pos % matrix.width, s_pos / matrix.width);

    println!("Part 1: {}", bfs(e_pos, s_pos, &matrix, false));
    println!("Part 2: {}", bfs(e_pos, s_pos, &matrix, true));
}

fn adjacents(
    visited: &HashSet<(usize, usize)>,
    marble: &Matrix2D<char>,
) -> HashSet<(usize, usize)> {
    let mut hset = HashSet::new();
    for &(pos_x, pos_y) in visited {
        let current_char = marble.get(pos_x, pos_y).unwrap() as u8;
        if pos_x != 0
            && current_char.saturating_sub(marble.get(pos_x - 1, pos_y).unwrap() as u8) < 2
            || marble.get(pos_x - 1, pos_y) == Some('S')
        {
            hset.insert((pos_x - 1, pos_y));
        }
        if pos_y != 0
            && current_char.saturating_sub(marble.get(pos_x, pos_y - 1).unwrap() as u8) < 2
            || marble.get(pos_x, pos_y - 1) == Some('S')
        {
            hset.insert((pos_x, pos_y - 1));
        }
        if pos_y < marble.height - 1
            && current_char.saturating_sub(marble.get(pos_x, pos_y + 1).unwrap() as u8) < 2
            || marble.get(pos_x, pos_y + 1) == Some('S')
        {
            hset.insert((pos_x, pos_y + 1));
        }
        if pos_x < marble.width - 1
            && current_char.saturating_sub(marble.get(pos_x + 1, pos_y).unwrap() as u8) < 2
            || marble.get(pos_x + 1, pos_y) == Some('S')
        {
            hset.insert((pos_x + 1, pos_y));
        }
    }
    hset
}

fn bfs(
    source: (usize, usize),
    destination: (usize, usize),
    marble: &Matrix2D<char>,
    part2: bool,
) -> usize {
    let mut steps = 0_usize;
    let mut visited: HashSet<(usize, usize)> = [source].iter().copied().collect();
    let mut current: HashSet<(usize, usize)> = [source].iter().copied().collect();

    while !visited.contains(&destination) {
        steps += 1;
        current = adjacents(&current, marble);
        if part2 && current.iter().any(|(x, y)| marble.get(*x, *y) == Some('a')) {
            break;
        }
        for pts in &current {
            visited.insert(*pts);
        }
    }
    steps
}
