use std::collections::VecDeque;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    use adventofcode::*;

    let input: Vec<Vec<u16>> = read_file_u16_csv("input/day06.txt");

    let output1 = part1(input[0].clone());
    println!("part 1: {}", output1);

    let output2 = part2(input[0].clone());
    println!("part 2: {}", output2);

    Ok(())
}

fn part1(input: Vec<u16>) -> u64 {
    calculate(input, 80)
}

fn part2(input: Vec<u16>) -> u64 {
    calculate(input, 256)
}

fn calculate(input: Vec<u16>, loops: i16) -> u64 {
    let mut state: VecDeque<u64> = initialize_deque(input);
    let mut store: VecDeque<u64> = VecDeque::from([0, 0]);

    for _t in -1..=loops {
        let (new_state, new_store) = step(state, store);
        state = new_state;
        store = new_store;
    }

    state.iter().sum::<u64>()
}

fn step(mut state: VecDeque<u64>, mut store: VecDeque<u64>) -> (VecDeque<u64>, VecDeque<u64>) {
    let cur = state.pop_front().unwrap_or(0);
    state.push_back(cur + store.pop_front().unwrap_or(0));
    store.push_back(cur);
    (state, store)
}

fn initialize_deque(input: Vec<u16>) -> VecDeque<u64> {
    let initial: Vec<u64> = (0..7)
        .map(|i| u64::try_from(input.clone().into_iter().filter(|&i2| i2 == i).count()).unwrap())
        .collect();
    VecDeque::from(initial)
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: [u16; 5] = [3, 4, 3, 1, 2];

    #[test]
    fn initialize_deque_test() {
        assert_eq!(
            initialize_deque(INPUT.to_vec()),
            VecDeque::from([0, 1, 1, 2, 1, 0, 0])
        );
    }

    #[test]
    fn step_test1() {
        assert_eq!(
            step(VecDeque::from([0, 1, 1, 2, 1, 0, 0]), VecDeque::from([0, 0])),
            (VecDeque::from([1, 1, 2, 1, 0, 0, 0]), VecDeque::from([0, 0]))
        );
    }

    #[test]
    fn step_test2() {
        assert_eq!(
            step(VecDeque::from([1, 1, 2, 1, 0, 0, 0]), VecDeque::from([0, 0])),
            (VecDeque::from([1, 2, 1, 0, 0, 0, 1]), VecDeque::from([0, 1]))
        );
    }

    #[test]
    fn step_test3() {
        assert_eq!(
            step(VecDeque::from([1, 2, 1, 0, 0, 0, 1]), VecDeque::from([0, 1])),
            (VecDeque::from([2, 1, 0, 0, 0, 1, 1]), VecDeque::from([1, 1]))
        );
    }

    #[test]
    fn step_test4() {
        assert_eq!(
            step(VecDeque::from([2, 1, 0, 0, 0, 1, 1]), VecDeque::from([1, 1])),
            (VecDeque::from([1, 0, 0, 0, 1, 1, 3]), VecDeque::from([1, 2]))
        );
    }

    #[test]
    fn part1_test() {
        assert_eq!(part1(INPUT.to_vec()), 5934);
    }

    // this takes some time
    #[test]
    fn part2_test() {
        assert_eq!(part2(INPUT.to_vec()), 26984457539);
    }
}
