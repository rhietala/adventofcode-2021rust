fn main() -> Result<(), Box<dyn std::error::Error>> {
    use adventofcode::*;

    let input: Vec<Vec<u8>> = read_file_u8_csv("input/day06.txt");

    let output1 = part1(input[0].clone());
    println!("part 1: {}", output1);

    let output2 = part2(input[0].clone());
    println!("part 2: {}", output2);

    Ok(())
}

fn part1(mut input: Vec<u8>) -> usize {
    for _loop in 0..80 {
        input = step(input);
    }
    input.len()
}

fn part2(mut input: Vec<u8>) -> usize {
    for i in 0..256 {
        println!("Loop number {}, fish count {}", i, input.len());
        input = step(input);
    }
    input.len()
}

fn step(mut timers: Vec<u8>) -> Vec<u8> {
    let new_count: usize = timers.clone().into_iter().filter(|&t| t == 0).count();
    let mut new_timers: Vec<u8> = vec![8].into_iter().cycle().take(new_count).collect();

    timers = timers
        .iter()
        .map(|&t| if t == 0 { 6 } else { t - 1 })
        .collect();
    timers.append(&mut new_timers);
    timers
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: [u8; 5] = [3, 4, 3, 1, 2];

    #[test]
    fn part1_test() {
        assert_eq!(part1(INPUT.to_vec()), 5934);
    }

    #[test]
    fn step_test() {
        assert_eq!(step(INPUT.to_vec()), vec!(2, 3, 2, 0, 1));
    }

    #[test]
    fn step_and_spawn_test() {
        assert_eq!(step(vec!(2, 3, 2, 0, 1)), vec!(1, 2, 1, 6, 0, 8));
    }

    // this takes some time
    fn part2_test() {
        assert_eq!(part2(INPUT.to_vec()), 26984457539);
    }
}
