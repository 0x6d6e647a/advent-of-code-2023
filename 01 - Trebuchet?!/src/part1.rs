use std::io::{stdin, BufRead};

const RADIX: u32 = 10;

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

fn digits_to_value(tens: u32, ones: u32) -> u32 {
    (tens * RADIX) + ones
}

fn main() {
    let mut sum = 0;

    let mut lines = stdin().lock().lines();

    while let Some(Ok(line)) = lines.next() {
        let left_digit = find_digit(line.chars());
        let right_digit = find_digit(line.chars().rev());
        sum += digits_to_value(left_digit, right_digit)
    }

    println!("{sum}");
}
