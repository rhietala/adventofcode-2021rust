fn part1(input: Vec<i32>) -> i32 {
    let mut prev: Option<i32> = None;
    let mut increase_count: i32 = 0;

    for x in &input {
        increase_count += match prev {
            Some(y) if x > &y => 1,
            _ => 0,
        };

        prev = Some(*x);
    }

    increase_count
}

const SLIDING_WINDOW_SIZE: usize = 3;

fn sliding_windows(xs: Vec<i32>) -> Vec<i32> {
    xs.windows(SLIDING_WINDOW_SIZE)
        .map(|window| window.iter().sum())
        .collect::<Vec<i32>>()
        .into()
}

fn part2(input: Vec<i32>) -> i32 {
    let window_sums = sliding_windows(input);
    part1(window_sums)
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    use adventofcode::*;

    let input: Vec<i32> = read_file_i32("input/day01.txt");

    let output1 = part1(input.clone());
    println!("part 1: {}", output1);

    let output2 = part2(input);
    println!("part 2: {}", output2);

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: [i32; 10] = [199, 200, 208, 210, 200, 207, 240, 269, 260, 263];

    #[test]
    fn part1_test() {
        assert_eq!(part1(INPUT.to_vec()), 7);
    }

    #[test]
    fn sliding_window_test() {
        assert_eq!(
            sliding_windows(INPUT.to_vec()),
            vec!(607, 618, 618, 617, 647, 716, 769, 792)
        );
    }

    #[test]
    fn part2_test() {
        assert_eq!(part2(INPUT.to_vec()), 5);
    }
}
