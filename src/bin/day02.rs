type Coord = (i32, i32);

fn parse_input_part1(s: String) -> Coord {
    match s.split(" ").collect::<Vec<&str>>().as_slice() {
        ["forward", x] => (x.parse().unwrap(), 0),
        ["down", x] => (0, x.parse().unwrap()),
        ["up", x] => (0, x.parse::<i32>().unwrap() * -1),
        _ => (0, 0),
    }
}

fn navigate(directions: Vec<Coord>) -> Coord {
    let mut location: Coord = (0, 0);
    for (delta_x, delta_y) in directions {
        location = (location.0 + delta_x, location.1 + delta_y)
    }
    location
}

fn part1(input: Vec<String>) -> i32 {
    let directions: Vec<Coord> = input.iter().map(|s| parse_input_part1(s.clone())).collect();
    let location = navigate(directions);

    location.0 * location.1
}

fn part2(input: Vec<String>) -> i32 {
    let mut location: Coord = (0, 0);
    let mut aim: i32 = 0;

    for s in &input {
        match s.split(" ").collect::<Vec<&str>>().as_slice() {
            ["down", x] => { aim += x.parse::<i32>().unwrap() },
            ["up", x] => { aim -= x.parse::<i32>().unwrap() },
            ["forward", x] => {
                let i: i32 = x.parse().unwrap();
                location = (location.0 + i, location.1 + aim * i)
            },
            _ => {},
        }
    }

    location.0 * location.1
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    use adventofcode::*;

    let input: Vec<String> = read_file_string("input/day02.txt");

    let output1 = part1(input.clone());
    println!("part 1: {}", output1);

    let output2 = part2(input);
    println!("part 2: {}", output2);

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: [&str; 6] = [
        "forward 5",
        "down 5",
        "forward 8",
        "up 3",
        "down 8",
        "forward 2",
    ];

    const DIRECTIONS: [Coord; 6] = [(5, 0), (0, 5), (8, 0), (0, -3), (0, 8), (2, 0)];

    fn map_to_string(input: [&str; 6]) -> Vec<String> {
        input.into_iter().map(|l| String::from(l)).collect()
    }

    #[test]
    fn parse_input_part1_test() {
        assert_eq!(parse_input_part1(String::from(INPUT[0])), DIRECTIONS[0]);
    }

    #[test]
    fn parse_input_part1_test2() {
        assert_eq!(parse_input_part1(String::from(INPUT[1])), DIRECTIONS[1]);
    }

    #[test]
    fn parse_input_part1_test3() {
        assert_eq!(parse_input_part1(String::from(INPUT[3])), DIRECTIONS[3]);
    }

    #[test]
    fn navigate_test() {
        assert_eq!(navigate(DIRECTIONS.to_vec()), (15, 10));
    }

    #[test]
    fn part1_test() {
        assert_eq!(part1(map_to_string(INPUT)), 150);
    }

    #[test]
    fn part2_test() {
        assert_eq!(part2(map_to_string(INPUT)), 900);
    }
}
