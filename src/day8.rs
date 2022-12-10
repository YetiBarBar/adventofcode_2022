use adventofcode_tooling::Matrix2D;
pub fn main() {
    /*     let matrix = Matrix2D {
            values: r#"30373
    25512
    65332
    33549
    35390"#
                .lines()
                .flat_map(|line| line.chars().map(|v| v.to_digit(10).unwrap()))
                .collect(),
            height: 5,
            width: 5,
        }; */
    let matrix = Matrix2D {
        values: include_str!("../data/day_2022_8.data")
            .lines()
            .flat_map(|line| line.chars().map(|v| v.to_digit(10).unwrap()))
            .collect(),
        height: 99,
        width: 99,
    };
    let visible_count = part_1(&matrix);

    println!("Part 1: {}", visible_count);

    let part2 = (0..matrix.values.len())
        .map(|position| (position % matrix.width, position / matrix.width))
        .map(|(x, y)| tree_scenic(&matrix, x, y))
        .max();
    println!("Part 2: {:?}", part2);
}

fn part_1(matrix: &Matrix2D<u32>) -> i32 {
    let mut visible_count = 0;
    for position in 0..matrix.values.len() {
        let (x, y) = (position % matrix.width, position / matrix.width);

        let row = matrix.row(y);
        let col = matrix.col(x);
        let current = row[x];

        if row[..x].iter().all(|v| current > *v)
            || row[x + 1..].iter().all(|v| current > *v)
            || col[..y].iter().all(|v| current > *v)
            || col[y + 1..].iter().all(|v| current > *v)
        {
            visible_count += 1;
        }
    }
    visible_count
}

fn tree_scenic(matrix: &Matrix2D<u32>, x: usize, y: usize) -> usize {
    let row = matrix.row(y);
    let col = matrix.col(x);
    let current = row[x];
    let x1 = row[x + 1..].iter().take_while(|v| **v < current).count()
        + usize::from(row[x + 1..].iter().any(|v| *v >= current));
    let x2 = row[..x].iter().rev().take_while(|v| **v < current).count()
        + usize::from(row[..x].iter().rev().any(|v| *v >= current));
    let y1 = col[y + 1..].iter().take_while(|v| **v < current).count()
        + usize::from(col[y + 1..].iter().any(|v| *v >= current));
    let y2 = col[..y].iter().rev().take_while(|v| **v < current).count()
        + usize::from(col[..y].iter().rev().any(|v| *v >= current));

    x1 * x2 * y1 * y2
}
