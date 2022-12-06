use hashbrown::HashSet;

pub fn main() {
    // let raw = "mjqjpqmgbljsphdztnvjfqwrcgsmlb";
    let raw = include_str!("../data/day_2022_6.data");
    let raw: Vec<char> = raw.chars().collect();
    println!("Part 1: {}", answer(&raw, 4));
    println!("Part 2: {}", answer(&raw, 14));
}

fn answer(data: &[char], len: usize) -> usize {
    data.windows(len)
        .enumerate()
        .find(|(_, win)| win.iter().collect::<HashSet<_>>().len() == len)
        .unwrap()
        .0
        + len
}
