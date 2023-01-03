pub fn main() {
    println!("Part 1: {}", part(1, 1));
    println!("Part 2: {}", part(811589153, 10));
}

fn part(key: i64, rounds: usize) -> i64 {
    let mut data = include_str!("../data/day_2022_20.data")
        .trim()
        .lines()
        .map(str::parse)
        .map(Result::unwrap)
        .map(|x: i64| x * key)
        .enumerate()
        .collect::<Vec<(usize, i64)>>();

    for _ in 0..rounds {
        // For each number in their original order.
        for original_index in 0..data.len() {
            let index = data.iter().position(|x| x.0 == original_index).unwrap();
            let value = data[index].1;
            let new_index = index as i64 + value;
            let new_index = new_index.rem_euclid(data.len() as i64 - 1);

            // Pull out number from current index and insert it at new index.
            let tmp = data.remove(index);
            data.insert(new_index as usize, tmp);
        }
    }
    // Calculate result.
    let zero = data.iter().position(|x| x.1 == 0).unwrap();
    let x1 = data[(zero + 1_000) % data.len()].1;
    let x2 = data[(zero + 2_000) % data.len()].1;
    let x3 = data[(zero + 3_000) % data.len()].1;
    x1 + x2 + x3
}
