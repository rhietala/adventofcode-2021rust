const LSB_MASK: u16 = 0b0000_0000_0000_0001;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    use adventofcode::*;

    let input: Vec<String> = read_file_string("input/day03.txt");

    let output1 = part1(input.clone());
    println!("part 1: {}", output1);

    let output2 = part2(input);
    println!("part 2: {}", output2);

    Ok(())
}

fn part1(input: Vec<String>) -> u32 {
    let input_len: usize = input.clone()[0].len();

    let ints: Vec<u16> = input.into_iter().map(|s| bit_string_to_int(s)).collect();

    let mut inverse_mask: u16 = 0;
    let mut gamma: u16 = 0;

    // loop in MSB
    for i in 0..input_len {
        inverse_mask <<= 1;
        inverse_mask |= LSB_MASK;
        gamma <<= 1;
        // so have to inverse i for this function position
        gamma |= most_common_bit_in_position(ints.clone(), (input_len - 1) - i, false);
    }

    let epsilon = !gamma & inverse_mask;

    u32::from(gamma) * u32::from(epsilon)
}

fn part2(input: Vec<String>) -> u32 {
    let input_len: usize = input.clone()[0].len();

    let ints: Vec<u16> = input.into_iter().map(|s| bit_string_to_int(s)).collect();
    let ints2 = ints.clone();

    let oxygen: u16 = filter_by_pos(ints2, input_len - 1, false);
    let scrubber: u16 = filter_by_pos(ints, input_len - 1, true);

    u32::from(oxygen) * u32::from(scrubber)
}

// LSB
fn filter_by_pos(ints: Vec<u16>, pos: usize, invert: bool) -> u16 {
    let selected_bit = most_common_bit_in_position(ints.clone(), pos, invert);
    let filtered: Vec<u16> = ints
        .into_iter()
        .filter(|&i| bit_in_position(i, pos) == selected_bit)
        .collect();

    if filtered.len() == 1 || pos == 0 {
        filtered[0]
    } else {
        filter_by_pos(filtered, pos - 1, invert)
    }
}

fn bit_string_to_int(s: String) -> u16 {
    let mut byte: u16 = 0;

    // loop in MSB
    for c in s.chars().collect::<Vec<_>>() {
        // first shift is useless, but the byte is all zeroes in the start, so
        // it doesn't matter
        byte <<= 1;
        if c == '1' {
            byte |= LSB_MASK;
        }
    }

    byte
}

// LSB
fn bit_in_position(int: u16, pos: usize) -> u16 {
    let mut i = int;
    i >>= pos;
    i &= LSB_MASK;
    i
}

// position in LSB (0 is the rightmost)
fn most_common_bit_in_position(ints: Vec<u16>, pos: usize, invert: bool) -> u16 {
    let count: usize = ints.len();
    let mut count_1: usize = 0;
    for i in ints {
        count_1 += usize::from(bit_in_position(i, pos));
    }
    let count_0: usize = count - count_1;
    if !invert {
        // If 0 and 1 are equally common, keep values with a 1 in the position being considered.
        if count_1 >= count_0 {
            1
        } else {
            0
        }
    } else {
        if count_0 <= count_1 {
            0
        } else {
            1
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: [&str; 12] = [
        "00100", "11110", "10110", "10111", "10101", "01111", "00111", "11100", "10000", "11001",
        "00010", "01010",
    ];

    #[test]
    fn bit_string_to_int_test() {
        assert_eq!(bit_string_to_int(String::from(INPUT[0])), 4);
    }

    #[test]
    fn most_common_bit_in_position_test() {
        let ints: Vec<u16> = INPUT
            .map(|s| bit_string_to_int(String::from(s)))
            .into_iter()
            .collect();
        assert_eq!(most_common_bit_in_position(ints.clone(), 0, false), 0);
        assert_eq!(most_common_bit_in_position(ints, 1, false), 1);
    }

    #[test]
    fn part1_test() {
        let input = INPUT.map(|s| String::from(s)).into_iter().collect();
        assert_eq!(part1(input), 198);
    }

    #[test]
    fn filter_by_pos_test() {
        let bytes: Vec<u16> = INPUT
            .map(|s| bit_string_to_int(String::from(s)))
            .into_iter()
            .collect();
        assert_eq!(filter_by_pos(bytes.clone(), 4, false), 23);
    }

    #[test]
    fn filter_by_pos_inverted_test() {
        let bytes: Vec<u16> = INPUT
            .map(|s| bit_string_to_int(String::from(s)))
            .into_iter()
            .collect();
        assert_eq!(filter_by_pos(bytes.clone(), 4, true), 10);
    }

    #[test]
    fn part2_test() {
        let input = INPUT.map(|s| String::from(s)).into_iter().collect();
        assert_eq!(part2(input), 230);
    }
}
