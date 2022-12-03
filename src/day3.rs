pub fn main() {
    let values: std::collections::HashMap<char, usize> =
        ('a'..='z').chain('A'..='Z').zip(1..).collect();

    let input = include_str!("../data/day_2022_3.data")
        .lines()
        .collect::<Vec<_>>();

    let part1 = part_1(&input, &values);
    println!("Part 1: {}", part1);

    let part2 = part_2(&input, &values);
    println!("Part 2: {}", part2);
}

fn part_2(input: &[&str], values: &std::collections::HashMap<char, usize>) -> usize {
    let part2 = input.chunks(3).fold(0_usize, |acc, line| {
        if let Some(item) = line[0]
            .chars()
            .filter(|chr| line[1].contains(*chr))
            .find(|chr| line[2].contains(*chr))
        {
            values.get(&item).unwrap() + acc
        } else {
            acc
        }
    });
    part2
}

fn part_1(input: &[&str], values: &std::collections::HashMap<char, usize>) -> usize {
    input.iter().fold(0_usize, |acc, line| {
        let (begin, end) = line.split_at(line.len() / 2);
        if let Some(item) = begin.chars().find(|chr| end.contains(*chr)) {
            values.get(&item).unwrap() + acc
        } else {
            acc
        }
    })
}
