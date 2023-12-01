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

fn search_digit<I>(line: &str, indices: I) -> u32
where
    I: Iterator<Item = usize>,
{
    for index in indices {
        let digit = find_digit_str(&line[index..]);

        if let Some(digit) = digit {
            return digit;
        }
    }

    panic!("unable to find digit");
}

#[aoc(day1, part2)]
fn part2(input: &str) -> u32 {
    let mut sum = 0;

    for line in input.lines() {
        let left_digit = search_digit(line, 0..line.len());
        let right_digit = search_digit(line, (0..line.len()).rev());
        sum += digits_to_value(left_digit, right_digit)
    }

    sum
}
