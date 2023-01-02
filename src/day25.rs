fn from_snafu(in_value: &str) -> i128 {
    in_value
        .chars()
        .rev()
        .zip(0..)
        .map(|(chr, pow)| {
            let val: i128 = match chr {
                '1' => 1,
                '2' => 2,
                '0' => 0,
                '-' => -1,
                '=' => -2,
                _ => panic!(),
            };
            val * 5_i128.pow(pow)
        })
        .sum()
}

fn to_snafu(in_value: i128) -> String {
    const DIVIDER: i128 = 5;
    let mut res = in_value;
    let mut output = vec![];

    while res != 0 {
        let new_char = match res.rem_euclid(DIVIDER) {
            0 => '0',
            1 => {
                res -= 1;
                '1'
            }
            2 => {
                res -= 2;
                '2'
            }
            3 => {
                res += 2;
                '='
            }
            4 => {
                res += 1;
                '-'
            }
            _ => unreachable!(),
        };

        res /= DIVIDER;
        output.push(new_char);
    }

    output.iter().rev().collect()
}

pub fn main() {
    let res = to_snafu(
        include_str!("../data/day_2022_25.data")
            .trim()
            .lines()
            .map(from_snafu)
            .sum::<i128>(),
    );
    println!("Part 1: {res}");
}
