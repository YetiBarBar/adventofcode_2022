use nom::{
    character::complete::{char, u64},
    combinator::map,
    multi::separated_list0,
    sequence::separated_pair,
    IResult,
};

fn segment_from_str(data: &str) -> IResult<&str, Segment> {
    map(separated_pair(u64, char('-'), u64), |(begin, end)| {
        Segment { begin, end }
    })(data)
}

fn segment_pair_from_str(data: &str) -> IResult<&str, SegmentPair> {
    map(
        separated_pair(segment_from_str, char(','), segment_from_str),
        |(pair1, pair2)| SegmentPair(pair1, pair2),
    )(data)
}

fn segment_pair_vect(data: &str) -> IResult<&str, Vec<SegmentPair>> {
    separated_list0(char('\n'), segment_pair_from_str)(data)
}

struct Segment {
    begin: u64,
    end: u64,
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

pub fn main() {
    if let Ok((_, data)) = segment_pair_vect(include_str!("../data/day_2022_4.data")) {
        println!(
            "Part 1: {}",
            data.iter().filter(|pair| pair.has_inclusion()).count()
        );

        println!(
            "Part 2: {}",
            data.iter().filter(|pair| pair.has_overlap()).count()
        );
    }
}
