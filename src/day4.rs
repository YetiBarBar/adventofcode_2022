use adventofcode_tooling::AocError;
use std::str::FromStr;

struct Segment {
    begin: u32,
    end: u32,
}

impl FromStr for Segment {
    type Err = AocError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let Some((begin, end)) = s.split_once('-') else {
            return Err(AocError::ParsingError);
        };

        Ok(Self {
            begin: begin.parse()?,
            end: end.parse()?,
        })
    }
}

impl Segment {
    fn is_included(&self, other: &Segment) -> bool {
        self.begin >= other.begin && self.end <= other.end
    }

    fn is_overlapping(&self, other: &Segment) -> bool {
        self.begin <= other.end && self.end >= other.begin
    }
}

struct SegmentPair(Segment, Segment);

impl SegmentPair {
    fn has_inclusion(&self) -> bool {
        self.1.is_included(&self.0) || self.0.is_included(&self.1)
    }

    fn has_overlap(&self) -> bool {
        self.1.is_overlapping(&self.0) || self.0.is_overlapping(&self.1)
    }
}

impl FromStr for SegmentPair {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (one, two) = s.split_once(',').unwrap();
        Ok(Self(one.parse().unwrap(), two.parse().unwrap()))
    }
}

pub fn main() {
    let data: Vec<SegmentPair> = include_str!("../data/day_2022_4.data")
        .lines()
        .map(str::parse)
        .map(Result::unwrap)
        .collect();

    println!(
        "Part 1: {}",
        data.iter().filter(|pair| pair.has_inclusion()).count()
    );

    println!(
        "Part 2: {}",
        data.iter().filter(|pair| pair.has_overlap()).count()
    );
}
