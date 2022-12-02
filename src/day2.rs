use std::str::FromStr;

struct Game(char, char);

impl Game {
    fn score_p1(&self) -> u64 {
        match (self.1, self.0) {
            ('X', 'A') => 4,
            ('X', 'B') => 1,
            ('X', 'C') => 7,
            ('Y', 'A') => 8,
            ('Y', 'B') => 5,
            ('Y', 'C') => 2,
            ('Z', 'A') => 3,
            ('Z', 'B') => 9,
            ('Z', 'C') => 6,
            _ => unreachable!(),
        }
    }
    fn score_p2(&self) -> u64 {
        match (self.1, self.0) {
            ('X', 'A') => 3,
            ('X', 'B') => 1,
            ('X', 'C') => 2,
            ('Y', 'A') => 4,
            ('Y', 'B') => 5,
            ('Y', 'C') => 6,
            ('Z', 'A') => 8,
            ('Z', 'B') => 9,
            ('Z', 'C') => 7,
            _ => unreachable!(),
        }
    }
}

impl FromStr for Game {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let chrs: Vec<char> = s.chars().collect();
        Ok(Self(chrs[0], chrs[2]))
    }
}

pub fn main() {
    let data: Vec<Game> = include_str!("../data/day_2022_2.data")
        .lines()
        .map(str::parse)
        .map(Result::unwrap)
        .collect();

    println!("{}", data.iter().map(Game::score_p1).sum::<u64>());
    println!("{}", data.iter().map(Game::score_p2).sum::<u64>());
}
