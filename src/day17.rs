static ROCKS: [[[u8; 4]; 4]; 5] = [
    [
        *b"####", //
        *b"    ", //
        *b"    ", //
        *b"    ", //
    ],
    [
        *b" #  ", //
        *b"### ", //
        *b" #  ", //
        *b"    ", //
    ],
    [
        *b"### ", //
        *b"  # ", //
        *b"  # ", //
        *b"    ", //
    ],
    [
        *b"#   ", //
        *b"#   ", //
        *b"#   ", //
        *b"#   ", //
    ],
    [
        *b"##  ", //
        *b"##  ", //
        *b"    ", //
        *b"    ", //
    ],
];

fn collision(rock: &[[u8; 4]; 4], x: usize, y: usize, board: &[[u8; 9]]) -> bool {
    for dy in 0..4 {
        for dx in 0..4 {
            if rock[dy][dx] == b'#' && board[y + dy][x + dx] == b'#' {
                return true;
            }
        }
    }
    false
}

fn find_repeating<T: Eq>(range: &[T]) -> Option<usize> {
    let len = range.len();

    (1.max(len / 3)..len / 2)
        .find(|&sub_len| range[len - sub_len * 2..len - sub_len].eq(&range[len - sub_len..]))
}

fn implementation(input: &str, amount: usize) -> Result<String, String> {
    let input: Vec<isize> = input
        .chars()
        .map(|c| match c {
            '<' => Ok(-1isize),
            '>' => Ok(1isize),
            _ => Err("Invalid input".to_string()),
        })
        .collect::<Result<_, _>>()?;

    let mut rocks = ROCKS.iter().enumerate().cycle();
    let mut wind = input.iter().copied().enumerate().cycle();

    let mut board: Vec<[u8; 9]> = vec![[b'#'; 9]; 1];
    let mut max = 0;

    let mut moves = Vec::new();
    let mut heights = Vec::new();

    let mut to_add = None;

    let mut i = 0;
    while i < amount {
        i += 1;
        let (kind, rock) = rocks.next().unwrap();
        board.resize(max + 8, *b"#       #");
        let start = max + 4;
        let mut y = start;
        let mut x = 3isize;
        let (mut wind_idx, mut wind_dir);
        loop {
            (wind_idx, wind_dir) = wind.next().unwrap();
            let new_x = x + wind_dir;
            if !collision(rock, new_x as usize, y, &board) {
                x = new_x;
            }
            let new_y = y - 1;
            if collision(rock, x as usize, new_y, &board) {
                break;
            }
            y = new_y;
        }

        for dy in 0..4 {
            for dx in 0..4 {
                if rock[dy][dx] == b'#' {
                    board[y + dy][x as usize + dx] = b'#';
                    max = max.max(y + dy);
                }
            }
        }

        if to_add.is_none() {
            moves.push((kind, wind_idx, x, start - y));
            if let Some(len) = find_repeating(&moves) {
                let rocks_left = amount - i;
                let height_diff = max - heights[heights.len() - len];
                let batches = rocks_left / len;
                to_add = Some(height_diff * batches);
                i += batches * len;
            }
            heights.push(max);
        }
    }

    Ok((max + to_add.unwrap_or(0)).to_string())
}

fn part1(input: &str) -> Result<String, String> {
    implementation(input, 2022)
}

fn part2(input: &str) -> Result<String, String> {
    implementation(input, 1_000_000_000_000)
}

fn main() {
    let input = include_str!("../data/day_2022_17.data").trim();
    println!("{:?}", part1(input));
    println!("{:?}", part2(input));
}
