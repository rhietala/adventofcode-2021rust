use itertools::Itertools;
use lazy_static::lazy_static;
use regex::Regex;

type Coord = (u16, u16);
type Line = (Coord, Coord);

fn main() -> Result<(), Box<dyn std::error::Error>> {
    use adventofcode::*;

    let input: Vec<String> = read_file_string("input/day05.txt");

    let output1 = part1(input.clone());
    println!("part 1: {}", output1);

    let output2 = part2(input);
    println!("part 2: {}", output2);

    Ok(())
}

fn part1(input: Vec<String>) -> usize {
    parts(input, false)
}

fn part2(input: Vec<String>) -> usize {
    parts(input, true)
}

fn parts(input: Vec<String>, include_diagonal: bool) -> usize {
    let mut lines: Vec<Line> = vec![];
    for x in input.into_iter().map(|i| parse_coords(i)) {
        match x {
            Some(c) => lines.push(c),
            _ => {}
        }
    }
    let coords: Vec<Coord> = lines
        .into_iter()
        .map(|l| calculate_covered(l, include_diagonal))
        .into_iter()
        .flatten()
        .collect();

    let counted = count_coordinates(coords);

    let filtered: Vec<(Coord, usize)> = counted
        .into_iter()
        .filter(|(_, count)| *count >= 2)
        .collect();

    filtered.len()
}

fn parse_coords(s: String) -> Option<Line> {
    lazy_static! {
        static ref RE: Regex =
            Regex::new(r"^(?P<from_x>\d+),(?P<from_y>\d+) -> (?P<to_x>\d+),(?P<to_y>\d+)").unwrap();
    }
    RE.captures(s.as_str()).and_then(|cap| {
        match (
            cap.name("from_x"),
            cap.name("from_y"),
            cap.name("to_x"),
            cap.name("to_y"),
        ) {
            (Some(from_x), Some(from_y), Some(to_x), Some(to_y)) => Some((
                (
                    from_x.as_str().parse::<u16>().unwrap(),
                    from_y.as_str().parse::<u16>().unwrap(),
                ),
                (
                    to_x.as_str().parse::<u16>().unwrap(),
                    to_y.as_str().parse::<u16>().unwrap(),
                ),
            )),
            _ => None,
        }
    })
}

fn calculate_covered(l: Line, include_diagonal: bool) -> Vec<Coord> {
    let x1 = l.0 .0;
    let y1 = l.0 .1;
    let x2 = l.1 .0;
    let y2 = l.1 .1;

    let x_range: Vec<u16> = if x1 <= x2 {
        (x1..=x2).collect()
    } else {
        (x2..=x1).rev().collect()
    };
    let y_range: Vec<u16> = if y1 <= y2 {
        (y1..=y2).collect()
    } else {
        (y2..=y1).rev().collect()
    };

    if x1 == x2 {
        y_range.into_iter().map(|y| (x1, y)).collect::<Vec<Coord>>()
    } else if y1 == y2 {
        x_range.into_iter().map(|x| (x, y1)).collect::<Vec<Coord>>()
    } else if include_diagonal
        && i32::abs(i32::from(y2) - i32::from(y1)) == i32::abs(i32::from(x2) - i32::from(x1))
    {
        x_range.into_iter().zip(y_range)
            .map(|(x, y)| (x, y))
            .collect::<Vec<Coord>>()
    } else {
        vec![]
    }
}

fn count_coordinates(coords: Vec<Coord>) -> Vec<(Coord, usize)> {
    let mut sorted = coords;
    sorted.sort();
    sorted
        .into_iter()
        .group_by(|c| *c)
        .into_iter()
        .map(|(c, g)| (c, g.collect::<Vec<_>>().len()))
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: [&str; 10] = [
        "0,9 -> 5,9",
        "8,0 -> 0,8",
        "9,4 -> 3,4",
        "2,2 -> 2,1",
        "7,0 -> 7,4",
        "6,4 -> 2,0",
        "0,9 -> 2,9",
        "3,4 -> 1,4",
        "0,0 -> 8,8",
        "5,5 -> 8,2",
    ];

    #[test]
    fn parse_coords_test() {
        assert_eq!(parse_coords(String::from(INPUT[0])), Some(((0, 9), (5, 9))));
    }

    #[test]
    fn calculate_covered_straight_test() {
        assert_eq!(
            calculate_covered(((0, 9), (5, 9)), false),
            vec!((0, 9), (1, 9), (2, 9), (3, 9), (4, 9), (5, 9),)
        )
    }

    #[test]
    fn calculate_covered_straight_test2() {
        assert_eq!(
            calculate_covered(((2, 2), (2, 1)), false),
            vec!((2, 2), (2, 1))
        )
    }

    #[test]
    fn calculate_covered_diagonal_test() {
        assert_eq!(
            calculate_covered(((2, 2), (1, 1)), true),
            vec!((2, 2), (1, 1))
        )
    }

    #[test]
    fn count_coordinates_test() {
        assert_eq!(
            count_coordinates(vec![(0, 9), (5, 9), (0, 9)]),
            vec![((0, 9), 2), ((5, 9), 1)]
        )
    }

    #[test]
    fn part1_test() {
        let input = INPUT.map(|s| String::from(s)).into_iter().collect();
        assert_eq!(part1(input), 5);
    }

    #[test]
    fn part2_test() {
        let input = INPUT.map(|s| String::from(s)).into_iter().collect();
        assert_eq!(part2(input), 12);
    }
}
