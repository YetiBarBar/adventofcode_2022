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

fn part_1(input: &[&str], values: &std::collections::HashMap<char, usize>) -> usize {
    input
        .iter()
        .map(|line| line.split_at(line.len() / 2))
        .map(|(begin, end)| {
            begin
                .chars()
                .find(|chr| end.contains(*chr))
                .and_then(|chr| values.get(&chr))
                .unwrap_or(&0)
        })
        .sum()
}

fn part_2(input: &[&str], values: &std::collections::HashMap<char, usize>) -> usize {
    let part2 = input
        .chunks(3)
        .map(|chunks| {
            chunks
                .first()
                .unwrap_or(&"")
                .chars()
                .find(|chr| chunks.iter().all(|chunk| chunk.contains(*chr)))
                .and_then(|chr| values.get(&chr))
                .unwrap_or(&0)
        })
        .sum();
    part2
}
