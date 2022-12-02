pub fn main() {
    let calories = parse_data();
    let calories = calories.into_sorted_vec();

    println!("Part 1: {}", calories.iter().rev().take(1).sum::<u64>());
    println!("Part 2: {}", calories.iter().rev().take(3).sum::<u64>());
}

fn parse_data() -> std::collections::BinaryHeap<u64> {
    let calories = include_str!("../data/day_2022_1.data")
        .split("\n\n")
        .map(|block| {
            block
                .lines()
                .map(str::parse::<u64>)
                .map(Result::unwrap)
                .sum::<u64>()
        })
        .collect();
    calories
}
