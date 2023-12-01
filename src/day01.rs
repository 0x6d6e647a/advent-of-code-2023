use aoc_runner_derive::aoc;
use std::cmp::min;

const RADIX: u32 = 10;

fn find_digit<I>(mut chars: I) -> Option<u32>
where
    I: Iterator<Item = char>,
{
    chars.find(|c| c.is_digit(RADIX))?.to_digit(RADIX)
}

#[aoc(day1, part1)]
fn part1(input: &str) -> u32 {
    let mut sum = 0;

    for line in input.lines() {
        let left_digit = find_digit(line.chars()).unwrap();
        let right_digit = find_digit(line.chars().rev()).unwrap();

        sum += (left_digit * RADIX) + right_digit;
    }

    sum
}

const NUMS: [(u32, &str); RADIX as usize] = [
    (0, "zero"),
    (1, "one"),
    (2, "two"),
    (3, "three"),
    (4, "four"),
    (5, "five"),
    (6, "six"),
    (7, "seven"),
    (8, "eight"),
    (9, "nine"),
];

fn find_digit_str(subline: &str) -> Option<u32> {
    // -- Check for digit.
    let first_char = subline.chars().next().unwrap();

    if first_char.is_digit(RADIX) {
        return Some(first_char.to_digit(RADIX).unwrap());
    }

    // -- Check for string.
    for (num, word) in NUMS {
        if subline.starts_with(word) {
            return Some(num);
        }
    }

    None
}

fn find_value(line: &str) -> u32 {
    // -- Find left digit.
    let mut left_digit = None;

    'left: for left_index in 0..line.len() {
        let right_limit = min(left_index + LONGEST_NUM_WORD_LEN, line.len());

        for right_index in left_index..right_limit {
            let curr_word = &line[left_index..=right_index];

            if let Some(num) = find_digit_str(curr_word) {
                left_digit = Some(num);
                break 'left;
            }
        }
    }

    let left_digit = left_digit.unwrap();

    // -- Find right digit.
    let mut right_digit = None;

    'right: for right_index in (0..line.len()).rev() {
        const LONGEST_NUM_WORD_LEN_INDEX: usize = LONGEST_NUM_WORD_LEN - 1;
        let left_begin = if right_index >= LONGEST_NUM_WORD_LEN_INDEX {
            right_index - LONGEST_NUM_WORD_LEN_INDEX
        } else {
            0
        };

        for left_index in (left_begin..=right_index).rev() {
            let curr_word = &line[left_index..=right_index];

            if let Some(num) = find_digit_str(curr_word) {
                right_digit = Some(num);
                break 'right;
            }
        }
    }

    let right_digit = right_digit.unwrap();

    // -- Calcuate value and return.
    (left_digit * RADIX) + right_digit
}

const LONGEST_NUM_WORD_LEN: usize = 5;

#[aoc(day1, part2)]
fn part2(input: &str) -> u32 {
    let mut sum = 0;

    for line in input.lines() {
        sum += find_value(line);
    }

    sum
}
