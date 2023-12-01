use aoc_runner_derive::aoc;

const RADIX: u32 = 10;

fn digits_to_value(tens: u32, ones: u32) -> u32 {
    (tens * RADIX) + ones
}

fn find_digit<I>(mut chars: I) -> u32
where
    I: Iterator<Item = char>,
{
    chars
        .find(|c| c.is_digit(RADIX))
        .unwrap()
        .to_digit(RADIX)
        .unwrap()
}

#[aoc(day1, part1)]
fn part1(input: &str) -> u32 {
    let mut sum = 0;

    for line in input.lines() {
        let left_digit = find_digit(line.chars());
        let right_digit = find_digit(line.chars().rev());

        sum += digits_to_value(left_digit, right_digit)
    }

    sum
}

const NUMS: [&str; RADIX as usize] = [
    "zero", "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
];

fn find_digit_str(subline: &str) -> Option<u32> {
    // -- Check for digit.
    let first_char = subline.chars().next().unwrap();

    if first_char.is_digit(RADIX) {
        return Some(first_char.to_digit(RADIX).unwrap());
    }

    // -- Check for string.
    for (num, word) in NUMS.iter().enumerate() {
        if subline.starts_with(word) {
            return Some(num as u32);
        }
    }

    None
}

fn find_value(line: &str) -> u32 {
    // -- Find left digit.
    let mut left_digit = None;

    for index in 0..line.len() {
        let digit = find_digit_str(&line[index..]);

        if digit.is_some() {
            left_digit = digit;
            break;
        }
    }

    // -- Find right digit.
    let mut right_digit = None;

    for index in (0..line.len()).rev() {
        let digit = find_digit_str(&line[index..]);

        if digit.is_some() {
            right_digit = digit;
            break;
        }
    }

    // -- Calcuate value and return.
    digits_to_value(left_digit.unwrap(), right_digit.unwrap())
}

#[aoc(day1, part2)]
fn part2(input: &str) -> u32 {
    let mut sum = 0;

    for line in input.lines() {
        sum += find_value(line);
    }

    sum
}
