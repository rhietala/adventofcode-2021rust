fn main() -> Result<(), Box<dyn std::error::Error>> {
    use adventofcode::*;

    let input: Vec<Vec<u16>> = read_file_u16_csv("input/day07.txt");

    let output1 = part1(input[0].clone());
    println!("part 1: {}", output1);

    let output2 = part2(input[0].clone());
    println!("part 2: {}", output2);

    Ok(())
}

fn part1(input: Vec<u16>) -> isize {
    let max_pos = input.clone().into_iter().max().unwrap();
    (0..=max_pos).map(|p| cost_to_pos_part1(p, &*input)).min().unwrap()
}

fn part2(input: Vec<u16>) -> isize {
    let max_pos = input.clone().into_iter().max().unwrap();
    (0..=max_pos).map(|p| cost_to_pos_part2(p, &*input)).min().unwrap()
}

fn cost_to_pos_part1(pos: u16, crab_pos: &[u16]) -> isize {
    isize::try_from(
        crab_pos
            .iter()
            .map(|&c| (c as isize - pos as isize).abs())
            .sum::<isize>(),
    )
    .unwrap()
}

fn cost_to_pos_part2(pos: u16, crab_pos: &[u16]) -> isize {
    isize::try_from(
        crab_pos
            .iter()
            .map(|&c| {
                let d = (c as isize - pos as isize).abs();
                // sum 1 + 2 + 3 + .. + d
                (d * (d + 1)) / 2
            })
            .sum::<isize>(),
    )
    .unwrap()
}


#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: [u16; 10] = [16, 1, 2, 0, 4, 2, 7, 1, 2, 14];

    #[test]
    fn part1_test() {
        assert_eq!(part1(INPUT.to_vec()), 37);
    }

    #[test]
    fn part2_test() {
        assert_eq!(part2(INPUT.to_vec()), 168);
    }

    #[test]
    fn cost_to_pos_part1_test1() {
        assert_eq!(cost_to_pos_part1(1, &INPUT), 41);
    }

    #[test]
    fn cost_to_pos_part1_test2() {
        assert_eq!(cost_to_pos_part1(10, &INPUT), 71);
    }

    #[test]
    fn cost_to_pos_part2_test1() {
        assert_eq!(cost_to_pos_part2(2, &INPUT), 206);
    }

    #[test]
    fn cost_to_pos_part2_test2() {
        assert_eq!(cost_to_pos_part2(5, &INPUT), 168);
    }
}
