use adventofcode_tooling::Matrix2D;
use nom::{
    bytes::complete::tag,
    character::complete::{char, newline, u32},
    combinator::{all_consuming, map},
    multi::separated_list0,
    sequence::separated_pair,
    IResult,
};

enum Part {
    Part1,
    Part2,
}

#[derive(Debug)]
struct Segment {
    points: Vec<(u32, u32)>,
}

fn segment(data: &str) -> IResult<&str, Segment> {
    map(
        separated_list0(tag(" -> "), separated_pair(u32, char(','), u32)),
        |list| Segment { points: list },
    )(data)
}

fn fill_matrix(matrix: &mut Matrix2D<char>, segments: &[Segment]) {
    for segment in segments {
        for p in segment.points.windows(2) {
            let p1 = p[0];
            let p2 = p[1];
            if p1.0 == p2.0 {
                let x = p1.0 as usize;
                // x is equal!
                let ymin = p2.1.min(p1.1) as usize;
                let ymax = p2.1.max(p1.1) as usize;
                for y in ymin..=ymax {
                    matrix.values[y * matrix.width + x] = '█';
                }
            } else if p1.1 == p2.1 {
                let y = p1.1 as usize;
                // x is equal!
                let xmin = p2.0.min(p1.0) as usize;
                let xmax = p2.0.max(p1.0) as usize;
                for x in xmin..=xmax {
                    matrix.values[y * matrix.width + x] = '█';
                }
            } else {
                panic!("Unsupported non linear!")
            }
        }
    }
}

fn fall_point(matrix: &mut Matrix2D<char>, part: &Part) -> bool {
    let part2 = matches!(part, Part::Part2);
    let (mut coord_x, mut coord_y) = (500, 0);
    loop {
        let (left, center, right) = (
            matrix.get(coord_x - 1, coord_y + 1).unwrap(),
            matrix.get(coord_x, coord_y + 1).unwrap(),
            matrix.get(coord_x + 1, coord_y + 1).unwrap(),
        );
        if matrix.get(500, 0) == Some('░') {
            return false;
        }
        if left != ' ' && center != ' ' && right != ' ' || (coord_y == matrix.height - 2 && part2) {
            // Then we have a support and put a ball!
            matrix.values[coord_y * matrix.width + coord_x] = '░';
            return true;
        } else if left == ' ' && center != ' ' {
            // we have room to fall left
            coord_x -= 1;
        } else if left != ' ' && center != ' ' && right == ' ' {
            coord_x += 1;
        }
        coord_y += 1;
        if !part2 && coord_y == 169 {
            break;
        }
    }
    false
}

fn run_part(segments: &[Segment], ymax: usize, part: Part) -> usize {
    // Let's fill a matrix with walls!
    let mut matrix = Matrix2D {
        width: 1000,
        height: ymax as usize + 3,
        values: vec![' '; 1000 * (ymax as usize + 3)],
    };
    fill_matrix(&mut matrix, &segments);

    let mut count = 0;
    while fall_point(&mut matrix, &part) {
        count += 1;
    }
    count
}

pub fn main() {
    let data = include_str!("../data/day_2022_14.data");
    let segments = all_consuming(separated_list0(newline, segment))(data)
        .unwrap()
        .1;

    // Look for xmin, xmax, ymin, ymin
    let (_, _, _, ymax) = segments.iter().flat_map(|s| s.points.iter()).fold(
        (u32::MAX, 0, u32::MAX, 0),
        |(xmin, xmax, ymin, ymax), (x, y)| (xmin.min(*x), xmax.max(*x), ymin.min(*y), ymax.max(*y)),
    );

    println!(
        "Part 1: {}",
        run_part(&segments, ymax as usize, Part::Part1)
    );
    println!(
        "Part 2: {}",
        run_part(&segments, ymax as usize, Part::Part2)
    );
}
